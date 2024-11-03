use std::{collections::HashSet, fs, path::Path};

use lrol_parser::{
    parser::LrolModel, types::Metadata, Action, Evaluation, LrolParser, ParserError,
};

use crate::{
    analyzer::RuleAnalyzer,
    error::{AnalyzerError, FileValidationError, ValidationError},
};

pub struct RuleValidator {
    analyzer: RuleAnalyzer,
}

impl RuleValidator {
    pub fn new() -> Self {
        Self {
            analyzer: RuleAnalyzer::new(),
        }
    }

    /// Validates an LROL rule string by both parsing and analyzing it
    pub fn validate(&mut self, input: &str) -> Result<LrolModel, Vec<ValidationError>> {
        let mut errors = Vec::new();

        // First try to parse the input
        let model = match LrolParser::parse(input) {
            Ok(model) => model,
            Err(e) => {
                errors.push(ValidationError::Parser(e));
                return Err(errors);
            }
        };

        // If parsing succeeds, analyze the model
        if let Err(analyzer_errors) = self.analyzer.analyze(&model) {
            errors.extend(analyzer_errors.into_iter().map(ValidationError::Analyzer));
        }

        if errors.is_empty() {
            Ok(model)
        } else {
            Err(errors)
        }
    }

    /// Provides a detailed report of all validation issues
    pub fn validate_with_report(&mut self, input: &str) -> ValidationReport {
        let mut report = ValidationReport::new();

        match LrolParser::parse(input) {
            Ok(model) => {
                report.model = Some(model.clone());

                // Add analysis phase if parsing succeeds
                if let Err(analyzer_errors) = self.analyzer.analyze(&model) {
                    report.analyzer_errors = analyzer_errors;
                }
            }
            Err(e) => {
                report.parser_error = Some(e);
            }
        }

        report
    }

    /// Validates an LROL rule from a file path
    pub fn validate_with_report_from_file<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ValidationReport, FileValidationError> {
        let path_str = path.as_ref().to_string_lossy().into_owned();

        // Check if file exists
        if !path.as_ref().exists() {
            return Err(FileValidationError::FileNotFound(path_str));
        }

        // Read file content
        let content =
            fs::read_to_string(&path).map_err(|error| FileValidationError::FileReadError {
                path: path_str.clone(),
                error,
            })?;

        // Validate content
        let report = self.validate_with_report(&content);

        if report.is_valid() {
            Ok(report)
        } else {
            Err(FileValidationError::ValidationErrors(report))
        }
    }

    /// Validates multiple LROL rule files from a directory
    pub fn validate_directory<P: AsRef<Path>>(
        &mut self,
        dir_path: P,
    ) -> Vec<(String, Result<ValidationReport, FileValidationError>)> {
        let mut results = Vec::new();

        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    let file_name = path.to_string_lossy().into_owned();
                    let validation_result = self.validate_with_report_from_file(&path);
                    results.push((file_name, validation_result));
                }
            }
        }

        results
    }
}

/// Struct to hold detailed validation results
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub file_path: Option<String>,
    pub model: Option<LrolModel>,
    pub parser_error: Option<ParserError>,
    pub analyzer_errors: Vec<AnalyzerError>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            file_path: None,
            model: None,
            parser_error: None,
            analyzer_errors: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.parser_error.is_none() && self.analyzer_errors.is_empty()
    }

    pub fn with_file_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            file_path: Some(path.as_ref().to_string_lossy().into_owned()),
            model: None,
            parser_error: None,
            analyzer_errors: Vec::new(),
        }
    }

    pub fn format_errors(&self) -> String {
        let mut output = String::new();

        if let Some(ref path) = self.file_path {
            output.push_str(&format!("File: {}\n", path));
        }

        if let Some(ref parser_error) = self.parser_error {
            output.push_str(&format!("Parser Error: {}\n", parser_error));
        }

        if !self.analyzer_errors.is_empty() {
            output.push_str("Analyzer Errors:\n");
            for (i, error) in self.analyzer_errors.iter().enumerate() {
                output.push_str(&format!("{}. {:?}\n", i + 1, error));
            }
        }

        if output.is_empty() || output.ends_with("File: {}\n") {
            output.push_str("No validation errors found.");
        }

        output
    }
}

#[derive(Debug)]
pub struct SchemaValidator {
    valid_evaluation_types: HashSet<String>,
    valid_comparison_operators: HashSet<String>,
    valid_logical_operators: HashSet<String>,
    // valid_action_types: HashSet<String>,
}

impl SchemaValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            valid_evaluation_types: HashSet::new(),
            valid_comparison_operators: HashSet::new(),
            // valid_action_types: HashSet::new(),
            valid_logical_operators: HashSet::new(),
        };

        // Initialize with schema-defined values
        validator.valid_evaluation_types.extend(
            [
                "comparison",
                "aggregation",
                "logical",
                "time-based",
                "conditional",
            ]
            .iter()
            .map(|s| s.to_string()),
        );

        validator
            .valid_logical_operators
            .extend(["AND", "OR"].iter().map(|s| s.to_string()));

        validator.valid_comparison_operators.extend(
            [
                ">", "<", ">=", "<=", "==", "!=", "IN", "NOT IN", "LIKE", "NOT LIKE",
            ]
            .iter()
            .map(|s| s.to_string()),
        );

        // validator.valid_action_types.extend(
        //     ["flag_transaction", "block_transaction", "send_alert"]
        //         .iter()
        //         .map(|s| s.to_string()),
        // );

        validator
    }
}

impl RuleAnalyzer {
    pub fn validate_schema_requirements(&self, model: &LrolModel, errors: &mut Vec<AnalyzerError>) {
        let schema_validator = SchemaValidator::new();

        // Validate model-level requirements
        self.validate_model_requirements(model, errors);

        // Validate evaluations
        for evaluation in &model.evaluations {
            self.validate_evaluation_schema(&schema_validator, evaluation, errors);
        }

        // Validate actions
        for action in &model.actions {
            self.validate_action_schema(action, errors);
        }

        // Validate metadata if present
        if let Some(ref metadata) = model.metadata {
            self.validate_metadata_schema(metadata, errors);
        }
    }

    fn validate_model_requirements(&self, model: &LrolModel, errors: &mut Vec<AnalyzerError>) {
        // Validate required fields
        if model.model_id.is_empty() {
            errors.push(AnalyzerError::MissingRequiredSchemaField {
                field: "model_id".to_string(),
            });
        }
        if model.name.is_empty() {
            errors.push(AnalyzerError::MissingRequiredSchemaField {
                field: "name".to_string(),
            });
        }

        // Validate threshold range (0 to 1)
        if model.threshold < 0.0 || model.threshold > 1.0 {
            errors.push(AnalyzerError::InvalidThreshold {
                value: model.threshold,
                reason: "Threshold must be between 0 and 1".to_string(),
            });
        }

        // Validate at least one evaluation exists
        if model.evaluations.is_empty() {
            errors.push(AnalyzerError::MissingRequiredSchemaField {
                field: "evaluations".to_string(),
            });
        }

        // Validate at least one action exists
        if model.actions.is_empty() {
            errors.push(AnalyzerError::MissingRequiredSchemaField {
                field: "actions".to_string(),
            });
        }
    }

    fn validate_evaluation_schema(
        &self,
        schema_validator: &SchemaValidator,
        evaluation: &Evaluation,
        errors: &mut Vec<AnalyzerError>,
    ) {
        // Validate evaluation type
        let eval_type = evaluation.evaluation_type.to_string();
        if !schema_validator.valid_evaluation_types.contains(&eval_type) {
            errors.push(AnalyzerError::InvalidEvaluationType {
                evaluation_name: evaluation.name.clone(),
                found_type: eval_type,
            });
        }

        if let Some(ref operator) = evaluation.operator {
            match evaluation.evaluation_type {
                lrol_parser::EvaluationType::Comparison => {
                    if !schema_validator
                        .valid_comparison_operators
                        .contains(operator)
                    {
                        errors.push(AnalyzerError::InvalidComparisonOperator {
                            evaluation_name: evaluation.name.clone(),
                            operator: operator.clone(),
                        });
                    }
                }
                lrol_parser::EvaluationType::Logical => {
                    if !schema_validator.valid_logical_operators.contains(operator) {
                        errors.push(AnalyzerError::InvalidLogicalOperator {
                            evaluation_name: evaluation.name.clone(),
                            operator: operator.clone(),
                        });
                    }
                }
                lrol_parser::EvaluationType::TimeBased => todo!(),
                lrol_parser::EvaluationType::Conditional => todo!(),
                _ => {}
            }
        }

        // Validate weight range (1-5)
        if let Some(weight) = evaluation.weight {
            if weight < 1 || weight > 5 {
                errors.push(AnalyzerError::InvalidWeightRange {
                    evaluation_name: evaluation.name.clone(),
                    weight,
                });
            }
        }
    }

    fn validate_action_schema(
        &self,
        // schema_validator: &SchemaValidator,
        action: &Action,
        errors: &mut Vec<AnalyzerError>,
    ) {
        // Validate action type
        // if !schema_validator
        //     .valid_action_types
        //     .contains(&action.action_type)
        // {
           
        // }
        if action.action_type.trim().is_empty() {
            errors.push(AnalyzerError::InvalidActionType {
                action_type: action.action_type.clone(),
            });
        }
       

        // Validate reason is present and not empty
        if action.reason.trim().is_empty() {
            errors.push(AnalyzerError::MissingActionReason {
                action_type: action.action_type.clone(),
            });
        }
    }

    fn validate_metadata_schema(&self, metadata: &Metadata, errors: &mut Vec<AnalyzerError>) {
        // Validate date-time formats
        if let Some(ref created_at) = metadata.created_at {
            if !self.is_valid_datetime(created_at) {
                errors.push(AnalyzerError::InvalidMetadataFormat {
                    field: "created_at".to_string(),
                    reason: "Invalid datetime format".to_string(),
                });
            }
        }

        if let Some(ref last_updated) = metadata.last_updated {
            if !self.is_valid_datetime(last_updated) {
                errors.push(AnalyzerError::InvalidMetadataFormat {
                    field: "last_updated".to_string(),
                    reason: "Invalid datetime format".to_string(),
                });
            }
        }
    }

    fn is_valid_datetime(&self, datetime_str: &str) -> bool {
        datetime_str.parse::<dateparser::DateTimeUtc>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_rule() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Valid Rule",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "amount_check",
                    "type": "comparison",
                    "left": "transaction_amount",
                    "operator": ">",
                    "right": 1000,
                    "weight": 3
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "High amount transaction"
                }
            ]
        }"#;

        let mut validator = RuleValidator::new();
        let result = validator.validate(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_syntax_error() {
        let input = r#"{
            "model_id": "TEST001"
            "name": "Invalid Rule", // Missing comma
            "threshold": 0.9,
            "evaluations": [],
            "actions": []
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        assert!(report.parser_error.is_some());
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validate_semantic_error() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Rule with Invalid Reference",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "invalid_check",
                    "type": "logical",
                    "operator": "AND",
                    "operands": ["non_existent_check"],
                    "weight": 3
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "High amount detected"
                }
            ]
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        assert!(report.parser_error.is_none());
        assert!(!report.analyzer_errors.is_empty());
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validate_multiple_errors() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Rule with Multiple Issues",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "check1",
                    "type": "logical",
                    "operator": "INVALID",
                    "operands": ["non_existent"],
                    "weight": 10
                },
                {
                    "name": "check1",
                    "type": "comparison",
                    "left": "amount",
                    "operator": ">",
                    "right": 100,
                    "weight": 3
                }
            ],
           "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "High amount detected"
                }
            ]
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        assert!(report.parser_error.is_none());
        assert!(report.analyzer_errors.len() > 1);

        // Check error report formatting
        let error_report = report.format_errors();
        assert!(error_report.contains("Analyzer Errors:"));
    }

    #[test]
    fn test_validate_datetime_and_references() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Complex Rule",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "time_check",
                    "type": "comparison",
                    "left": "datetime(now, '-2 hours')",
                    "operator": ">",
                    "right": "transaction_time",
                    "weight": 3
                },
                {
                    "name": "reference_check",
                    "type": "comparison",
                    "left": "@time_check result",
                    "operator": "==",
                    "right": true,
                    "weight": 3
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "High amount detected"
                }
            ]
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        assert!(report.parser_error.is_none());
        assert!(
            report.is_valid(),
            "Validation errors: {}",
            report.format_errors()
        );
    }

    #[test]
    fn test_validation_report_formatting() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Rule with Issues",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "check1",
                    "type": "logical",
                    "operator": "INVALID",
                    "operands": ["non_existent"],
                    "weight": 10
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "High amount detected"
                }
            ]
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        let formatted_report = report.format_errors();

        assert!(!formatted_report.is_empty());
        assert!(formatted_report.contains("Analyzer Errors:"));
        assert!(formatted_report.contains("1."));
    }

    #[test]
    fn test_valid_model_with_metadata() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Complete Valid Model",
            "description": "Test model with metadata",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "amount_check",
                    "type": "comparison",
                    "left": "amount",
                    "operator": ">",
                    "right": 1000,
                    "weight": 3
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "High amount detected"
                }
            ],
            "metadata": {
                "created_by": "test_user",
                "created_at": "2024-01-01T12:00:00Z",
                "last_updated": "2024-01-01T12:00:00Z",
                "notes": "Test notes"
            }
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        assert!(
            report.is_valid(),
            "Validation errors: {}",
            report.format_errors()
        );
    }

    #[test]
    fn test_invalid_threshold() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Invalid Threshold Model",
            "threshold": 1.5,
            "evaluations": [
                {
                    "name": "test_check",
                    "type": "comparison",
                    "left": "value",
                    "operator": ">",
                    "right": 100,
                    "weight": 3
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "Test"
                }
            ]
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        assert!(!report.is_valid());
        assert!(report.analyzer_errors.iter().any(|e| matches!(e,
            AnalyzerError::InvalidThreshold { value, .. } if *value > 1.0
        )));
    }

    #[test]
    fn test_invalid_metadata_datetime() {
        let input = r#"{
            "model_id": "TEST001",
            "name": "Invalid Metadata",
            "threshold": 0.9,
            "evaluations": [
                {
                    "name": "test_check",
                    "type": "comparison",
                    "left": "value",
                    "operator": ">",
                    "right": 100,
                    "weight": 3
                }
            ],
            "actions": [
                {
                    "type": "flag_transaction",
                    "reason": "Test"
                }
            ],
            "metadata": {
                "created_at": "invalid-date",
                "last_updated": "2024-01-01T12:00:00Z"
            }
        }"#;

        let mut validator = RuleValidator::new();
        let report = validator.validate_with_report(input);
        assert!(!report.is_valid());
        assert!(report.analyzer_errors.iter().any(|e| matches!(e,
            AnalyzerError::InvalidMetadataFormat { field, .. } if field == "created_at"
        )));
    }
}
