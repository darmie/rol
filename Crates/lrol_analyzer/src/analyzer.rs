use std::collections::{HashMap, HashSet};

use lrol_parser::{parser::LrolModel, Evaluation, EvaluationType, Value};

use crate::error::AnalyzerError;

#[derive(Debug)]
pub enum DurationUnit {
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Debug)]
pub struct Duration {
    pub value: i64,
    pub unit: DurationUnit,
}

#[derive(Debug)]
pub enum DateTimeRef {
    Now,
    Timestamp(String),
    Duration(Duration),
}

pub struct RuleAnalyzer {
    evaluation_names: HashSet<String>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl RuleAnalyzer {
    pub fn new() -> Self {
        Self {
            evaluation_names: HashSet::new(),
            dependency_graph: HashMap::new(),
        }
    }

    // Parse duration strings like "2 hours", "-3 days", etc.
    fn parse_duration(duration_str: &str) -> Result<Duration, String> {
        let trimmed = duration_str.trim().trim_matches('\'').trim_matches('"');
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        if parts.len() != 2 {
            return Err("Duration must contain a number and a unit".to_string());
        }

        let value = parts[0]
            .parse::<i64>()
            .map_err(|_| "Invalid duration value")?;

        let unit = match parts[1].to_lowercase().as_str() {
            "minute" | "minutes" | "min" | "mins" => DurationUnit::Minutes,
            "hour" | "hours" | "hr" | "hrs" => DurationUnit::Hours,
            "day" | "days" => DurationUnit::Days,
            "week" | "weeks" => DurationUnit::Weeks,
            "month" | "months" => DurationUnit::Months,
            "year" | "years" => DurationUnit::Years,
            _ => return Err("Invalid duration unit".to_string()),
        };

        Ok(Duration { value, unit })
    }

    pub fn analyze(&mut self, model: &LrolModel) -> Result<(), Vec<AnalyzerError>> {
        let mut errors = Vec::new();

        self.validate_schema_requirements(model, &mut errors);

        // First pass: collect all evaluation names and validate uniqueness
        for evaluation in &model.evaluations {
            if !self.evaluation_names.insert(evaluation.name.clone()) {
                errors.push(AnalyzerError::DuplicateEvaluationName(
                    evaluation.name.clone(),
                ));
            }
        }

        // Second pass: validate evaluations and build dependency graph
        for evaluation in &model.evaluations {
            // Validate evaluation-specific rules
            self.validate_evaluation(evaluation, &mut errors);

            // Build dependency graph for logical evaluations and string references
            let mut dependencies = Vec::new();

            // Add logical operand dependencies
            if let Some(operands) = &evaluation.operands {
                dependencies.extend(operands.clone());
            }

            // Add string reference dependencies
            self.collect_string_references(evaluation, &mut dependencies);

            if !dependencies.is_empty() {
                self.dependency_graph
                    .insert(evaluation.name.clone(), dependencies);
            }
        }

        // Third pass: check for circular dependencies
        if let Err(cycle) = self.check_circular_dependencies() {
            errors.push(cycle);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_evaluation(&self, evaluation: &Evaluation, errors: &mut Vec<AnalyzerError>) {
        // Add datetime validation
        self.validate_datetime_expressions(evaluation, errors);

        // Validate string references
        self.validate_string_references(evaluation, errors);

        // Validate weight range (1-5)
        if let Some(weight) = evaluation.weight {
            if weight < 1 || weight > 5 {
                errors.push(AnalyzerError::InvalidWeight {
                    evaluation_name: evaluation.name.clone(),
                    weight,
                });
            }
        }
        // Validate weight range (1-5)
        if let Some(weight) = evaluation.weight {
            if weight < 1 || weight > 5 {
                errors.push(AnalyzerError::InvalidWeight {
                    evaluation_name: evaluation.name.clone(),
                    weight,
                });
            }
        }

        // Validate evaluation type specific requirements
        match evaluation.evaluation_type {
            EvaluationType::Logical => {
                // Validate logical operator
                if let Some(ref operator) = evaluation.operator {
                    if operator != "AND" && operator != "OR" {
                        errors.push(AnalyzerError::InvalidLogicalOperator {
                            evaluation_name: evaluation.name.clone(),
                            operator: operator.clone(),
                        });
                    }
                } else {
                    errors.push(AnalyzerError::MissingRequiredField {
                        evaluation_name: evaluation.name.clone(),
                        field_name: "operator".to_string(),
                    });
                }

                // Validate operands
                match &evaluation.operands {
                    Some(operands) => {
                        if operands.is_empty() {
                            errors.push(AnalyzerError::EmptyOperands(evaluation.name.clone()));
                        }
                        // Check if all operands reference existing evaluations
                        for operand in operands {
                            if !self.evaluation_names.contains(operand) {
                                errors.push(AnalyzerError::MissingOperandReference {
                                    evaluation_name: evaluation.name.clone(),
                                    missing_operand: operand.clone(),
                                });
                            }
                        }
                    }
                    None => {
                        errors.push(AnalyzerError::MissingRequiredField {
                            evaluation_name: evaluation.name.clone(),
                            field_name: "operands".to_string(),
                        });
                    }
                }
            }
            EvaluationType::Comparison => {
                // Validate required fields for comparison
                if evaluation.left.is_none() {
                    errors.push(AnalyzerError::MissingRequiredField {
                        evaluation_name: evaluation.name.clone(),
                        field_name: "left".to_string(),
                    });
                }
                if evaluation.operator.is_none() {
                    errors.push(AnalyzerError::MissingRequiredField {
                        evaluation_name: evaluation.name.clone(),
                        field_name: "operator".to_string(),
                    });
                }
                if evaluation.right.is_none() {
                    errors.push(AnalyzerError::MissingRequiredField {
                        evaluation_name: evaluation.name.clone(),
                        field_name: "right".to_string(),
                    });
                }
            }
            // TODO: Add validation for other evaluation types as needed
            _ => {}
        }
    }

    // Helper function to extract evaluation names from string references
    pub fn extract_references(value: &str) -> Vec<String> {
        let mut refs = Vec::new();
        for word in value.split_whitespace() {
            if word.starts_with('@') {
                // Remove @ and collect reference
                refs.push(word[1..].to_string());
            }
        }
        refs
    }

    // Collect all string references from an evaluation
    fn collect_string_references(&self, evaluation: &Evaluation, dependencies: &mut Vec<String>) {
        if let Some(ref left) = evaluation.left {
            dependencies.extend(Self::extract_references(left));
        }

        if let Some(Value::String(ref right)) = evaluation.right {
            dependencies.extend(Self::extract_references(right));
        }

        // Add more fields that might contain references here
    }

    fn validate_string_references(&self, evaluation: &Evaluation, errors: &mut Vec<AnalyzerError>) {
        // Validate left operand references
        if let Some(ref left) = evaluation.left {
            for reference in Self::extract_references(left) {
                if !self.evaluation_names.contains(&reference) {
                    errors.push(AnalyzerError::InvalidStringReference {
                        evaluation_name: evaluation.name.clone(),
                        field_name: "left".to_string(),
                        reference,
                    });
                }
            }
        }

        // Validate right operand references if it's a string
        if let Some(Value::String(ref right)) = evaluation.right {
            for reference in Self::extract_references(right) {
                if !self.evaluation_names.contains(&reference) {
                    errors.push(AnalyzerError::InvalidStringReference {
                        evaluation_name: evaluation.name.clone(),
                        field_name: "right".to_string(),
                        reference,
                    });
                }
            }
        }
    }

    fn check_circular_dependencies(&self) -> Result<(), AnalyzerError> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        for start_node in self.dependency_graph.keys() {
            if !visited.contains(start_node) {
                if let Some(cycle) = self.detect_cycle(start_node, &mut visited, &mut path) {
                    return Err(AnalyzerError::CircularDependency {
                        evaluation_name: start_node.clone(),
                        dependency_chain: cycle,
                    });
                }
            }
        }

        Ok(())
    }

    fn detect_cycle(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        if path.contains(&node.to_string()) {
            let cycle_start = path.iter().position(|x| x == node).unwrap();
            return Some(path[cycle_start..].to_vec());
        }

        if visited.contains(node) {
            return None;
        }

        visited.insert(node.to_string());
        path.push(node.to_string());

        if let Some(dependencies) = self.dependency_graph.get(node) {
            for dep in dependencies {
                if let Some(cycle) = self.detect_cycle(dep, visited, path) {
                    return Some(cycle);
                }
            }
        }

        path.pop();
        None
    }

    // Parse datetime expressions like datetime(now, '-2 hours')
    fn parse_datetime_expression(expr: &str) -> Result<(DateTimeRef, Option<DateTimeRef>), String> {
        // Basic syntax check
        if !expr.starts_with("datetime(") || !expr.ends_with(')') {
            return Err("Invalid datetime function syntax".to_string());
        }

        // Extract arguments
        let args_str = &expr[9..expr.len() - 1];
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

        if args.is_empty() || args.len() > 2 {
            return Err("datetime() requires 1 or 2 arguments".to_string());
        }

        // Parse first argument
        let first_arg = match args[0] {
            "now" => DateTimeRef::Now,
            ts if ts.starts_with('\'') || ts.starts_with('"') => {
                DateTimeRef::Timestamp(ts.trim_matches('\'').trim_matches('"').to_string())
            }
            _ => return Err("Invalid first argument".to_string()),
        };

        // Parse optional second argument (duration)
        let second_arg = if args.len() == 2 {
            let duration_str = args[1].trim_matches('\'').trim_matches('"');
            if duration_str.is_empty() {
                return Err("Empty duration string".to_string());
            }
            Some(DateTimeRef::Duration(Self::parse_duration(duration_str)?))
        } else {
            None
        };

        Ok((first_arg, second_arg))
    }

    fn validate_datetime_expressions(
        &self,
        evaluation: &Evaluation,
        errors: &mut Vec<AnalyzerError>,
    ) {
        // Validate left operand
        if let Some(ref left) = evaluation.left {
            if left.starts_with("datetime(") {
                match Self::parse_datetime_expression(left) {
                    Err(reason) => {
                        errors.push(AnalyzerError::InvalidDateTimeExpression {
                            evaluation_name: evaluation.name.clone(),
                            field_name: "left".to_string(),
                            expression: left.clone(),
                            reason,
                        });
                    }
                    Ok(_) => {} // Valid datetime expression
                }
            }
        }

        // Validate right operand if it's a string
        if let Some(Value::String(ref right)) = evaluation.right {
            if right.starts_with("datetime(") {
                match Self::parse_datetime_expression(right) {
                    Err(reason) => {
                        errors.push(AnalyzerError::InvalidDateTimeExpression {
                            evaluation_name: evaluation.name.clone(),
                            field_name: "right".to_string(),
                            expression: right.clone(),
                            reason,
                        });
                    }
                    Ok(_) => {} // Valid datetime expression
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lrol_parser::types::{Action, Evaluation, EvaluationType, Metadata, Value};

    fn create_test_model() -> LrolModel {
        LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: Some("Test Description".to_string()),
            threshold: 0.9,
            evaluations: vec![
                Evaluation {
                    name: "amount_check".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("amount".to_string()),
                    operator: Some(">".to_string()),
                    right: Some(Value::Number(1000.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
                Evaluation {
                    name: "risk_check".to_string(),
                    evaluation_type: EvaluationType::Logical,
                    left: None,
                    operator: Some("AND".to_string()),
                    right: None,
                    operands: Some(vec!["amount_check".to_string()]),
                    weight: Some(4),
                    aggregation: None,
                },
            ],
            actions: vec![
                Action {
                action_type: "flag_transaction".to_string(),
                reason: "High risk transaction".to_string(),
            }
            ],
            metadata: Some(Metadata{
                created_by: Some("test_user".to_owned()),
                created_at: Some("2024-01-01T12:00:00Z".to_owned()),
                last_updated: Some("2024-01-01T12:00:00Z".to_owned()),
                notes: Some("Test notes".to_owned())
            })
        }
    }

    #[test]
    fn test_valid_model() {
        let model = create_test_model();
        let mut analyzer = RuleAnalyzer::new();
        let res = analyzer.analyze(&model);
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_duplicate_evaluation_names() {
        let mut model = create_test_model();
        model.evaluations.push(Evaluation {
            name: "amount_check".to_string(), // Duplicate name
            evaluation_type: EvaluationType::Comparison,
            left: Some("amount".to_string()),
            operator: Some(">".to_string()),
            right: Some(Value::Number(1000.0)),
            operands: None,
            weight: Some(3),
            aggregation: None,
        });

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);
        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e, AnalyzerError::DuplicateEvaluationName(_)))
        ));
    }

    #[test]
    fn test_invalid_operand_reference() {
        let mut model = create_test_model();
        model.evaluations.push(Evaluation {
            name: "invalid_reference".to_string(),
            evaluation_type: EvaluationType::Logical,
            left: None,
            operator: Some("AND".to_string()),
            right: None,
            operands: Some(vec!["non_existent".to_string()]),
            weight: Some(3),
            aggregation: None,
        });

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);

        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e, AnalyzerError::MissingOperandReference { .. }))
        ));
    }

    #[test]
    fn test_circular_dependency() {
        let mut model = create_test_model();
        model.evaluations = vec![
            Evaluation {
                name: "eval1".to_string(),
                evaluation_type: EvaluationType::Logical,
                left: None,
                operator: Some("AND".to_string()),
                right: None,
                operands: Some(vec!["eval2".to_string()]),
                weight: Some(3),
                aggregation: None,
            },
            Evaluation {
                name: "eval2".to_string(),
                evaluation_type: EvaluationType::Logical,
                left: None,
                operator: Some("AND".to_string()),
                right: None,
                operands: Some(vec!["eval1".to_string()]),
                weight: Some(3),
                aggregation: None,
            },
        ];

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);
        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e, AnalyzerError::CircularDependency { .. }))
        ));
    }

    #[test]
    fn test_invalid_weight() {
        let mut model = create_test_model();
        model.evaluations.push(Evaluation {
            name: "invalid_weight".to_string(),
            evaluation_type: EvaluationType::Comparison,
            left: Some("amount".to_string()),
            operator: Some(">".to_string()),
            right: Some(Value::Number(1000.0)),
            operands: None,
            weight: Some(6), // Invalid weight > 5
            aggregation: None,
        });

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);
        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e, AnalyzerError::InvalidWeight { .. }))
        ));
    }

    #[test]
    fn test_valid_string_reference() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: Some("Test Description".to_string()),
            threshold: 0.9,
            evaluations: vec![
                Evaluation {
                    name: "base_check".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("amount".to_string()),
                    operator: Some(">".to_string()),
                    right: Some(Value::Number(1000.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
                Evaluation {
                    name: "reference_check".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("@base_check result".to_string()),
                    operator: Some(">".to_string()),
                    right: Some(Value::Number(100.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
            ],
            actions: vec![Action {
                action_type: "flag_transaction".to_owned(),
                reason: "High amount detected".to_owned(),
            }],
            metadata: Some(Metadata{
                created_by: Some("test_user".to_owned()),
                created_at: Some("2024-01-01T12:00:00Z".to_owned()),
                last_updated: Some("2024-01-01T12:00:00Z".to_owned()),
                notes: Some("Test notes".to_owned())
            })
        };

        let mut analyzer = RuleAnalyzer::new();
        assert!(analyzer.analyze(&model).is_ok());
    }

    #[test]
    fn test_invalid_string_reference() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: Some("Test Description".to_string()),
            threshold: 0.9,
            evaluations: vec![Evaluation {
                name: "reference_check".to_string(),
                evaluation_type: EvaluationType::Comparison,
                left: Some("@non_existent_check value".to_string()),
                operator: Some(">".to_string()),
                right: Some(Value::Number(100.0)),
                operands: None,
                weight: Some(3),
                aggregation: None,
            }],
            actions: vec![Action {
                action_type: "flag_transaction".to_owned(),
                reason: "High amount detected".to_owned(),
            }],
            metadata: Some(Metadata{
                created_by: Some("test_user".to_owned()),
                created_at: Some("2024-01-01T12:00:00Z".to_owned()),
                last_updated: Some("2024-01-01T12:00:00Z".to_owned()),
                notes: Some("Test notes".to_owned())
            })
        };

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);
        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e,
                AnalyzerError::InvalidStringReference {
                    reference,
                    ..
                } if reference == "non_existent_check"
            ))
        ));
    }

    #[test]
    fn test_circular_string_reference() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: Some("Test Description".to_string()),
            threshold: 0.9,
            evaluations: vec![
                Evaluation {
                    name: "eval1".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("@eval2 value".to_string()),
                    operator: Some(">".to_string()),
                    right: Some(Value::Number(100.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
                Evaluation {
                    name: "eval2".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("@eval1 value".to_string()),
                    operator: Some(">".to_string()),
                    right: Some(Value::Number(100.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
            ],
            actions: vec![Action {
                action_type: "flag_transaction".to_owned(),
                reason: "High amount detected".to_owned(),
            }],
            metadata: Some(Metadata{
                created_by: Some("test_user".to_owned()),
                created_at: Some("2024-01-01T12:00:00Z".to_owned()),
                last_updated: Some("2024-01-01T12:00:00Z".to_owned()),
                notes: Some("Test notes".to_owned())
            })
        };

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);
        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e, AnalyzerError::CircularDependency { .. }))
        ));
    }

    #[test]
    fn test_multiple_string_references() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: Some("Test Description".to_string()),
            threshold: 0.9,
            evaluations: vec![
                Evaluation {
                    name: "eval1".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("amount".to_string()),
                    operator: Some(">".to_string()),
                    right: Some(Value::Number(100.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
                Evaluation {
                    name: "eval2".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("value".to_string()),
                    operator: Some("<".to_string()),
                    right: Some(Value::Number(50.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
                Evaluation {
                    name: "combined_check".to_string(),
                    evaluation_type: EvaluationType::Comparison,
                    left: Some("@eval1 and @eval2 check".to_string()), // Not a valid expression but just using this to detect the two references
                    operator: Some(">".to_string()),
                    right: Some(Value::Number(75.0)),
                    operands: None,
                    weight: Some(3),
                    aggregation: None,
                },
            ],
            actions: vec![Action {
                action_type: "flag_transaction".to_owned(),
                reason: "High amount detected".to_owned(),
            }],
            metadata: Some(Metadata{
                created_by: Some("test_user".to_owned()),
                created_at: Some("2024-01-01T12:00:00Z".to_owned()),
                last_updated: Some("2024-01-01T12:00:00Z".to_owned()),
                notes: Some("Test notes".to_owned())
            })
        };

        let mut analyzer = RuleAnalyzer::new();
        assert!(analyzer.analyze(&model).is_ok());
    }

    #[test]
    fn test_valid_datetime_expression() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: None,
            threshold: 0.9,
            evaluations: vec![Evaluation {
                name: "time_check".to_string(),
                evaluation_type: EvaluationType::Comparison,
                left: Some("transaction_time".to_string()),
                operator: Some(">".to_string()),
                right: Some(Value::String("datetime(now, '-2 hours')".to_string())),
                operands: None,
                weight: Some(3),
                aggregation: None,
            }],
            actions: vec![Action {
                action_type: "flag_transaction".to_string(),
                reason: "Transaction flagged based on tiered risk assessment.".to_string(),
            }],
            ..Default::default()
        };

        let mut analyzer = RuleAnalyzer::new();
        let res = analyzer.analyze(&model);
        assert!(res.is_ok());
    }

    #[test]
    fn test_invalid_datetime_syntax() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: None,
            threshold: 0.9,
            evaluations: vec![Evaluation {
                name: "time_check".to_string(),
                evaluation_type: EvaluationType::Comparison,
                left: Some("datetime(invalid syntax".to_string()),
                operator: Some(">".to_string()),
                right: Some(Value::Number(100.0)),
                operands: None,
                weight: Some(3),
                aggregation: None,
            }],
            actions: vec![Action {
                action_type: "flag_transaction".to_string(),
                reason: "Transaction flagged based on tiered risk assessment.".to_string(),
            }],
            ..Default::default()
        };

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);
        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e,
                AnalyzerError::InvalidDateTimeExpression { .. }
            ))
        ));
    }

    #[test]
    fn test_invalid_duration_format() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: None,
            threshold: 0.9,
            evaluations: vec![Evaluation {
                name: "time_check".to_string(),
                evaluation_type: EvaluationType::Comparison,
                left: Some("transaction_time".to_string()),
                operator: Some(">".to_string()),
                right: Some(Value::String(
                    "datetime(now, 'invalid duration')".to_string(),
                )),
                operands: None,
                weight: Some(3),
                aggregation: None,
            }],
            actions: vec![Action {
                action_type: "flag_transaction".to_string(),
                reason: "Transaction flagged based on tiered risk assessment.".to_string(),
            }],
            ..Default::default()
        };

        let mut analyzer = RuleAnalyzer::new();
        let result = analyzer.analyze(&model);
        assert!(matches!(
            result,
            Err(ref errors) if errors.iter().any(|e| matches!(e,
                AnalyzerError::InvalidDateTimeExpression { .. }
            ))
        ));
    }

    #[test]
    fn test_valid_duration_formats() {
        let test_cases = vec![
            "datetime(now, '-2 hours')",
            "datetime(now, '1 day')",
            "datetime(now, '-30 minutes')",
            "datetime(now, '2 weeks')",
            "datetime(now, '-3 months')",
            "datetime(now, '1 year')",
        ];

        for expr in test_cases {
            assert!(
                RuleAnalyzer::parse_datetime_expression(expr).is_ok(),
                "Failed to parse valid expression: {}",
                expr
            );
        }
    }

    #[test]
    fn test_datetime_with_timestamp() {
        let model = LrolModel {
            model_id: "TEST001".to_string(),
            name: "Test Model".to_string(),
            description: None,
            threshold: 0.9,
            evaluations: vec![Evaluation {
                name: "time_check".to_string(),
                evaluation_type: EvaluationType::Comparison,
                left: Some("transaction_time".to_string()),
                operator: Some(">".to_string()),
                right: Some(Value::String(
                    "datetime('2024-01-01', '-2 hours')".to_string(),
                )),
                operands: None,
                weight: Some(3),
                aggregation: None,
            }],
            actions: vec![Action {
                action_type: "flag_transaction".to_string(),
                reason: "Transaction flagged based on tiered risk assessment.".to_string(),
            }],
            ..Default::default()
        };

        let mut analyzer = RuleAnalyzer::new();
        assert!(analyzer.analyze(&model).is_ok());
    }
}
