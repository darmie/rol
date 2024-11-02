use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::complete::bool as Bool;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize, value, verify},
    error::{context, convert_error, ParseError, VerboseError, VerboseErrorKind},
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    Err as NomErr, IResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::{
        convert_nom_error, syntax_error, ParserError, INVALID_EVAL_TYPE, INVALID_LOGICAL_OP,
        INVALID_OPERANDS, INVALID_WEIGHT, MISSING_LEFT, MISSING_NAME, MISSING_OPERANDS,
        MISSING_OPERATOR, MISSING_RIGHT, MISSING_TYPE,
    },
    types::{Action, Evaluation, EvaluationType, Value},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LrolModel {
    pub model_id: String,
    pub name: String,
    pub description: Option<String>,
    pub threshold: f64,
    pub evaluations: Vec<Evaluation>,
    pub actions: Vec<Action>,
}

pub struct LrolParser;

impl LrolParser {
    pub fn parse(input: &str) -> Result<LrolModel, ParserError> {
        match Self::parse_model(input) {
            Ok((_, model)) => Ok(model),
            Err(NomErr::Error(e) | NomErr::Failure(e)) => {
                eprintln!("Parse error:\n{}", convert_error(input, e.clone()));
                Err(convert_nom_error(input, e))
            }
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

    fn parse_model_contents(mut input: &str) -> IResult<&str, LrolModel, VerboseError<&str>> {
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
                            } else {
                                return syntax_error(new_input, "Invalid model_id");
                            }
                        }
                        "name" => {
                            if let Value::String(v) = value {
                                name = Some(v)
                            } else {
                                return syntax_error(new_input, "Invalid name");
                            }
                        }
                        "description" => {
                            if let Value::String(v) = value {
                                description = Some(v)
                            } else {
                                return syntax_error(new_input, "Invalid description");
                            }
                        }
                        "threshold" => {
                            if let Value::Number(v) = value {
                                threshold = Some(v)
                            } else {
                                return syntax_error(
                                    new_input,
                                    "Invalid threshold type: expected Number",
                                );
                            }
                        }
                        "evaluations" => {
                            if let Value::Array(v) = value {
                                evaluations = Some(v);
                            } else {
                                return syntax_error(
                                    new_input,
                                    "Invalid evaluations type: expected array",
                                );
                            }
                        }
                        "actions" => {
                            if let Value::Array(v) = value {
                                actions = Some(v);
                            } else {
                                return syntax_error(
                                    new_input,
                                    "Invalid actions type: expected array",
                                );
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
                evaluations: Self::parse_evaluations_array(input, evaluations.unwrap_or_default()),
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
                map(Self::parse_boolean, Value::Bool),
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
    fn parse_string(input: &str) -> IResult<&str, String, VerboseError<&str>> {
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

    fn parse_boolean(input: &str) -> IResult<&str, bool, VerboseError<&str>> {
        context(
            "boolean",
            alt((
                map(tag("true"), |_| true),
                map(tag("false"), |_| false),
            ))
        )(input)
    }

    // Parse an array
    fn parse_array(input: &str) -> IResult<&str, Vec<Value>, VerboseError<&str>> {
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
    fn parse_evaluations_array(input: &str, values: Vec<Value>) -> Vec<Evaluation> {
        values
            .into_iter()
            .filter_map(|v| {
                if let Value::Object(fields) = v {
                    if let Some((_, ev)) = Self::parse_evaluation_from_fields(input, &fields).ok() {
                        Some(ev)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    // Parse a single evaluation from fields
    fn parse_evaluation_from_fields<'a>(
        current_input: &'a str,
        fields: &[(String, Value)],
    ) -> IResult<&'a str, Evaluation, VerboseError<&'a str>> {
        let mut name = None;
        let mut eval_type = None;
        let mut left = None;
        let mut operator = None;
        let mut right = None;
        let mut operands = None;
        let mut weight = None;

        for (key, value) in fields {
            match (key.as_str(), value) {
                ("name", Value::String(v)) => name = Some(v.clone()),
                ("type", Value::String(v)) => {
                    eval_type = match EvaluationType::from_str(v) {
                        Ok(t) => Some(t),
                        Err(_) => return syntax_error(current_input, INVALID_EVAL_TYPE),
                    };
                }
                ("operator", Value::String(v)) => {
                    operator = Some(v.clone());
                    // Validate logical operators immediately
                    if let Some(EvaluationType::Logical) = eval_type {
                        if !matches!(v.as_str(), "AND" | "OR") {
                            return syntax_error(current_input, INVALID_LOGICAL_OP);
                        }
                    }
                }
                ("operands", Value::Array(arr)) => {
                    operands = match arr.iter().try_fold(Vec::new(), |mut acc, v| {
                        if let Value::String(s) = v {
                            acc.push(s.clone());
                            Ok(acc)
                        } else {
                            Err(())
                        }
                    }) {
                        Ok(ops) => Some(ops),
                        Err(_) => return syntax_error(current_input, INVALID_OPERANDS),
                    };
                }
                ("left", Value::String(v)) => left = Some(v.clone()),
                ("right", v) => right = Some(v.clone()),
                ("weight", Value::Number(v)) => weight = Some(*v as i32),
                ("weight", _) => return syntax_error(current_input, INVALID_WEIGHT),
                _ => {} // Ignore unknown fields for forward compatibility
            }
        }

        // Validate required fields based on evaluation type
        if let Some(eval_type) = eval_type.as_ref() {
            match eval_type {
                EvaluationType::Logical => {
                    if operands.is_none() {
                        return syntax_error(current_input, MISSING_OPERANDS);
                    }
                    if operator.is_none() {
                        return syntax_error(current_input, MISSING_OPERATOR);
                    }
                }
                EvaluationType::Comparison => {
                    if left.is_none() {
                        return syntax_error(current_input, MISSING_LEFT);
                    }
                    if operator.is_none() {
                        return syntax_error(current_input, MISSING_OPERATOR);
                    }
                    if right.is_none() {
                        return syntax_error(current_input, MISSING_RIGHT);
                    }
                }
                _ => {} // Add validation for other types as needed
            }
        } else {
            return syntax_error(current_input, MISSING_TYPE);
        }

        // Validate name is present
        let name = match name {
            Some(n) => n,
            None => return syntax_error(current_input, MISSING_NAME),
        };

        Ok((
            current_input,
            Evaluation {
                name,
                evaluation_type: eval_type.unwrap(),
                left,
                operator,
                right,
                operands,
                weight,
            },
        ))
    }

    // Helper method to parse a single evaluation
    fn parse_single_evaluation(input: &str) -> IResult<&str, Evaluation, VerboseError<&str>> {
        let (remaining, fields) = context("evaluation object", Self::parse_object_value)(input)?;

        Self::parse_evaluation_from_fields(remaining, &fields)
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
        let evaluations = LrolParser::parse_evaluations_array(input, values);
        assert_eq!(evaluations.len(), 1);

        let eval = &evaluations[0];
        assert_eq!(eval.name, "Test_Check");
        assert_eq!(eval.evaluation_type, EvaluationType::Comparison);
        assert_eq!(eval.left, Some(String::from("value")));
        assert_eq!(eval.operator, Some(String::from(">")));
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
        assert_eq!(model.evaluations.len(), 3);
        let first_eval = &model.evaluations[0];
        assert_eq!(first_eval.name, "Transaction_Amount_Check");
        assert_eq!(first_eval.evaluation_type, EvaluationType::Comparison);
        assert_eq!(first_eval.left, Some(String::from("transaction_amount")));
        assert_eq!(first_eval.operator, Some(String::from(">")));
        assert_eq!(first_eval.right, Some(Value::Number(10000.0)));
        assert_eq!(first_eval.weight, Some(4));

        let second_eval = &model.evaluations[1];
        assert_eq!(second_eval.name, "Account_Age_Check");
        assert_eq!(second_eval.evaluation_type, EvaluationType::Comparison);
        assert_eq!(second_eval.left, Some(String::from("account_age_days")));
        assert_eq!(second_eval.operator, Some(String::from("<=")));
        assert_eq!(second_eval.right, Some(Value::Number(30.0)));
        assert_eq!(second_eval.weight, Some(3));

        let third_eval = &model.evaluations[2];
        assert_eq!(third_eval.name, "High_Risk_Transaction_Logic");
        assert_eq!(third_eval.evaluation_type, EvaluationType::Logical);
        assert_eq!(third_eval.operator, Some(String::from("AND")));
        assert_eq!(
            third_eval.operands,
            Some(vec![
                "Transaction_Amount_Check".to_string(),
                "Account_Age_Check".to_string()
            ])
        );
        assert_eq!(third_eval.weight, Some(5));

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
            Err(ParserError::InvalidSyntax {
                line,
                column,
                message,
            }) => {
                println!("Error at line {}, column {}: {}", line, column, message);
                assert!(line > 1, "Error should not be on first line");
            }
            _ => panic!("Expected InvalidSyntax error"),
        }
    }

    #[test]
    fn test_error_position_calculation() {
        let input = "line1\nline2\nline3\nline4\nerror";
        let error_input = "error";
        let (line, column) = crate::error::get_error_position(input, error_input);
        assert_eq!(line, 5);
        assert_eq!(column, 1);
    }

    #[test]
    fn test_parse_with_wrong_type() {
        let input = r#"{
            "model_id": "M501",
            "name": "Test Model",
            "threshold": "not_a_number",
            "evaluations": []
        }"#;

        let result = LrolParser::parse(input);
        assert!(result.is_err());

        match result {
            Err(ParserError::InvalidSyntax {
                line,
                column,
                message,
            }) => {
                println!("Error at line {}, column {}: {}", line, column, message);
                assert_eq!(line, 4, "Error should be on line 4");
            }
            _ => panic!("Expected InvalidSyntax error"),
        }
    }
}
