use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize},
    error::{context, convert_error, ParseError, VerboseError, VerboseErrorKind},
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    Err as NomErr, IResult,
};

use crate::error::{convert_nom_error, ParserError};

#[derive(Debug, PartialEq)]
pub struct LrolModel {
    pub model_id: String,
    pub name: String,
    pub description: Option<String>,
    pub threshold: f64,
    pub evaluations: Vec<Evaluation>,
    pub actions: Vec<Action>,
}

#[derive(Debug, PartialEq)]
pub struct Evaluation {
    pub name: String,
    pub evaluation_type: String,
    pub left: String,
    pub operator: String,
    pub right: Value,
    pub weight: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct Action {
    pub action_type: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

pub struct LrolParser;

impl LrolParser {
    pub fn parse(input: &str) -> Result<LrolModel, ParserError> {
        match Self::parse_model(input) {
            Ok((_, model)) => Ok(model),
            Err(NomErr::Error(e) | NomErr::Failure(e)) => {
                eprintln!("Parse error:\n{}", convert_error(input, e.clone()));
                Err(convert_nom_error(input, e))
            },
            Err(NomErr::Incomplete(_)) => Err(ParserError::InvalidSyntax {
                line: 1,
                column: 1,
                message: "Incomplete input".to_string(),
            }),
        }
    }


    // Generic parse model function that can work with different error types
    fn parse_model(input: &str) -> IResult<&str, LrolModel, VerboseError<&str>> {
        let (input, _) = multispace0(input)?;
        let (input, _) = char('{')(input)?;
        let (input, model) = Self::parse_model_contents(input)?;
        let (input, _) = char('}')(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, model))
    }

    // Generic object parser
    fn parse_object<F, T>(content_parser: F) -> impl Fn(&str) -> IResult<&str, T>
    where
        F: Fn(&str) -> IResult<&str, T> + Copy,
    {
        move |input: &str| {
            delimited(
                char('{'),
                delimited(multispace0, content_parser, multispace0),
                char('}'),
            )(input)
        }
    }

    // Parse model contents
    fn parse_model_contents(mut input: &str) ->  IResult<&str, LrolModel, VerboseError<&str>>  {
        let mut model_id = None;
        let mut name = None;
        let mut description = None;
        let mut threshold = None;
        let mut evaluations = None;
        let mut actions = None;

        loop {
            let (new_input, _) = multispace0(input)?;
            input = new_input;

            // Try to parse a field
            match Self::parse_field(input) {
                Ok((new_input, (field_name, value))) => {
                    match field_name {
                        "model_id" => {
                            if let Value::String(v) = value {
                                model_id = Some(v)
                            }
                        }
                        "name" => {
                            if let Value::String(v) = value {
                                name = Some(v)
                            }
                        }
                        "description" => {
                            if let Value::String(v) = value {
                                description = Some(v)
                            }
                        }
                        "threshold" => {
                            if let Value::Number(v) = value {
                                threshold = Some(v)
                            }
                        }
                        "evaluations" => {
                            if let Value::Array(v) = value {
                                evaluations = Some(v);
                            }
                        }
                        "actions" => {
                            if let Value::Array(v) = value {
                                actions = Some(v);
                            }
                        }
                        _ => {}
                    }

                    // Handle comma and continue
                    let (new_input, maybe_comma) =
                        opt(tuple((multispace0, char(','), multispace0)))(new_input)?;
                    input = new_input;

                    if maybe_comma.is_none() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        Ok((
            input,
            LrolModel {
                model_id: model_id.unwrap_or_default(),
                name: name.unwrap_or_default(),
                description,
                threshold: threshold.unwrap_or_default(),
                evaluations: Self::parse_evaluations_array(evaluations.unwrap_or_default()),
                actions: Self::parse_actions_array(actions.unwrap_or_default()),
            },
        ))
    }

    // Parse a single field
    fn parse_field(input: &str) -> IResult<&str, (&str, Value), VerboseError<&str>> {
        let (input, key) = delimited(char('"'), take_while1(|c| c != '"'), char('"'))(input)?;
        let (input, _) = tuple((multispace0, char(':'), multispace0))(input)?;
        let (input, value) = Self::parse_value(input)?;

        Ok((input, (key, value)))
    }

    // Parse a JSON value
    fn parse_value(input: &str) -> IResult<&str, Value, VerboseError<&str>> {
        delimited(
            multispace0,
            alt((
                map(Self::parse_string, Value::String),
                map(Self::parse_number, Value::Number),
                map(Self::parse_array, Value::Array),
                map(Self::parse_object_value, Value::Object),
            )),
            multispace0,
        )(input)
    }

    // Parse a JSON object as a value
    fn parse_object_value(input: &str) -> IResult<&str, Vec<(String, Value)>, VerboseError<&str>> {
        delimited(
            char('{'),
            delimited(
                multispace0,
                separated_list0(
                    tuple((multispace0, char(','), multispace0)),
                    Self::parse_key_value,
                ),
                multispace0,
            ),
            char('}'),
        )(input)
    }

    // Parse a key-value pair
    fn parse_key_value(input: &str) -> IResult<&str, (String, Value), VerboseError<&str>> {
        let (input, _) = multispace0(input)?;
        let (input, key) = delimited(char('"'), take_while1(|c| c != '"'), char('"'))(input)?;
        let (input, _) = tuple((multispace0, char(':'), multispace0))(input)?;
        let (input, value) = Self::parse_value(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, (key.to_string(), value)))
    }

    // Parse a string value
    fn parse_string(input: &str) -> IResult<&str, String, VerboseError<&str>>  {
        delimited(
            char('"'),
            map(take_while1(|c| c != '"'), |s: &str| s.to_string()),
            char('"'),
        )(input)
    }

    // Parse a number value
    fn parse_number(input: &str) -> IResult<&str, f64, VerboseError<&str>> {
        map_res(
            recognize(tuple((
                opt(char('-')),
                digit1,
                opt(pair(char('.'), digit1)),
            ))),
            |s: &str| s.parse::<f64>(),
        )(input)
    }

    // Parse an array
    fn parse_array(input: &str) -> IResult<&str, Vec<Value>, VerboseError<&str>>  {
        delimited(
            char('['),
            delimited(
                multispace0,
                separated_list0(
                    delimited(multispace0, char(','), multispace0),
                    Self::parse_value,
                ),
                multispace0,
            ),
            char(']'),
        )(input)
    }

    // Parse evaluations array into Evaluation structs
    fn parse_evaluations_array(values: Vec<Value>) -> Vec<Evaluation> {
        values
            .into_iter()
            .filter_map(|v| {
                if let Value::Object(fields) = v {
                    Self::parse_evaluation_from_fields(&fields).ok()
                } else {
                    None
                }
            })
            .collect()
    }

    // Parse a single evaluation from fields
    fn parse_evaluation_from_fields(
        fields: &[(String, Value)],
    ) -> Result<Evaluation, &'static str> {
        let mut name = None;
        let mut eval_type = None;
        let mut left = None;
        let mut operator = None;
        let mut right = None;
        let mut weight = None;

        for (key, value) in fields {
            match (key.as_str(), value) {
                ("name", Value::String(v)) => name = Some(v.clone()),
                ("type", Value::String(v)) => eval_type = Some(v.clone()),
                ("left", Value::String(v)) => left = Some(v.clone()),
                ("operator", Value::String(v)) => operator = Some(v.clone()),
                ("right", v) => right = Some(v.clone()),
                ("weight", Value::Number(v)) => weight = Some(*v as i32),
                (field, unexpected) => return Err(
                    match field {
                        "name" | "type" | "left" | "operator" => "Expected string value",
                        "weight" => "Expected number value",
                        _ => "Unexpected field",
                    }
                ),
            }
        }

        Ok(Evaluation {
            name: name.ok_or("Missing required field 'name'")?,
            evaluation_type: eval_type.ok_or("Missing required field 'type'")?,
            left: left.ok_or("Missing required field 'left'")?,
            operator: operator.ok_or("Missing required field 'operator'")?,
            right: right.ok_or("Missing required field 'right'")?,
            weight,
        })
    }

    // Helper method to parse a single evaluation
    fn parse_single_evaluation(input: &str) -> IResult<&str, Evaluation, VerboseError<&str>> {
        let (remaining, fields) = context(
            "evaluation object",
            Self::parse_object_value
        )(input)?;

        match Self::parse_evaluation_from_fields(&fields) {
            Ok(eval) => Ok((remaining, eval)),
            Err(msg) => Err(nom::Err::Error(VerboseError { errors: vec![
                (input, VerboseErrorKind::Context(msg))
            ]})),
        }
    }

    // Parse actions array into Action structs
    fn parse_actions_array(values: Vec<Value>) -> Vec<Action> {
        values
            .into_iter()
            .filter_map(|v| {
                if let Value::Object(fields) = v {
                    Self::parse_action_from_fields(&fields).ok()
                } else {
                    None
                }
            })
            .collect()
    }

    // Parse a single action from fields
    fn parse_action_from_fields(fields: &[(String, Value)]) -> Result<Action, &'static str> {
        let mut action_type = None;
        let mut reason = None;

        for (key, value) in fields {
            match (key.as_str(), value) {
                ("type", Value::String(v)) => action_type = Some(v.clone()),
                ("reason", Value::String(v)) => reason = Some(v.clone()),
                _ => {}
            }
        }

        Ok(Action {
            action_type: action_type.ok_or("Missing action type")?,
            reason: reason.ok_or("Missing reason")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_arrays() {
        let input = r#"{
            "model_id": "M501",
            "name": "Test Model",
            "threshold": 0.9,
            "evaluations": [],
            "actions": []
        }"#;

        let result = LrolParser::parse(input);
        assert!(result.is_ok());
        let model = result.unwrap();
        assert!(model.evaluations.is_empty());
        assert!(model.actions.is_empty());
    }

    #[test]
    fn test_parse_single_evaluation() {
        let input = r#"{
            "name": "Test_Check",
            "type": "comparison",
            "left": "value",
            "operator": ">",
            "right": 100,
            "weight": 3
        }"#;

        let result = LrolParser::parse_single_evaluation(input);
        assert!(
            result.is_ok(),
            "Failed to parse with error: {:?}",
            result.err()
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_evaluations_array() {
        let input = r#"[
            {
                "name": "Test_Check",
                "type": "comparison",
                "left": "value",
                "operator": ">",
                "right": 100,
                "weight": 3
            }
        ]"#;

        let (_rest, values) = LrolParser::parse_array(input).unwrap();
        let evaluations = LrolParser::parse_evaluations_array(values);
        assert_eq!(evaluations.len(), 1);

        let eval = &evaluations[0];
        assert_eq!(eval.name, "Test_Check");
        assert_eq!(eval.evaluation_type, "comparison");
        assert_eq!(eval.left, "value");
        assert_eq!(eval.operator, ">");
        assert_eq!(eval.weight, Some(3));
    }

    #[test]
    fn test_parse_readme_example() {
        let input = r#"{
            "model_id": "M501",
            "name": "High-Risk Open Banking Transactions",
            "description": "Detects high-value transactions initiated by third-party apps",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "Transaction_Amount_Check",
                    "type": "comparison",
                    "left": "transaction_amount",
                    "operator": ">",
                    "right": 10000,
                    "weight": 4
                },
                {
                    "name": "Account_Age_Check",
                    "type": "comparison",
                    "left": "account_age_days",
                    "operator": "<=",
                    "right": 30,
                    "weight": 3
                },
                {
                    "name": "High_Risk_Transaction_Logic",
                    "type": "logical",
                    "operator": "AND",
                    "operands":[
                        "Transaction_Amount_Check",
                        "Account_Age_Check"
                    ],
                    "weight": 5
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "Transaction flagged due to high value and new account."
                }
            ]
        }"#;

        let result = LrolParser::parse(input);
        assert!(
            result.is_ok(),
            "Failed to parse with error: {:?}",
            result.err()
        );

        let model = result.unwrap();

        // Verify model fields
        assert_eq!(model.model_id, "M501");
        assert_eq!(model.name, "High-Risk Open Banking Transactions");
        assert_eq!(model.threshold, 0.9);

        // Verify evaluations
        assert_eq!(model.evaluations.len(), 2);
        let first_eval = &model.evaluations[0];
        assert_eq!(first_eval.name, "Transaction_Amount_Check");
        assert_eq!(first_eval.evaluation_type, "comparison");
        assert_eq!(first_eval.left, "transaction_amount");
        assert_eq!(first_eval.operator, ">");
        assert_eq!(first_eval.weight, Some(4));

        // Verify actions
        assert_eq!(model.actions.len(), 1);
        let action = &model.actions[0];
        assert_eq!(action.action_type, "flag_transaction");
        assert_eq!(
            action.reason,
            "Transaction flagged due to high value and new account."
        );
    }

    #[test]
    fn test_parse_partial_evaluation() {
        let input = r#"{
            "model_id": "M501",
            "name": "Test Model",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "Partial_Check",
                    "type": "comparison",
                    "left": "value",
                    "operator": ">"
                }
            ],
            "actions": []
        }"#;

        let result = LrolParser::parse(input);
        assert!(result.is_ok());
    }


    #[test]
    fn test_parse_with_syntax_error() {
        let input = r#"{
            "model_id": "M501",
            "name": "Test Model",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "Test_Check"
                    "type": "comparison",  // Missing comma
                    "left": "value",
                    "operator": ">",
                    "right": 100
                }
            ]
        }"#;

        let result = LrolParser::parse(input);
        assert!(result.is_err());
        
        match result {
            Err(ParserError::InvalidSyntax { line, column, message }) => {
                println!("Error at line {}, column {}: {}", line, column, message);
                assert!(line > 1, "Error should not be on first line");
            },
            _ => panic!("Expected InvalidSyntax error"),
        }
    }
}
