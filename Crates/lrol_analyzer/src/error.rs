use lrol_parser::ParserError;

use crate::validator::ValidationReport;


#[derive(Debug, Clone)]
pub enum AnalyzerError {
    DuplicateEvaluationName(String),
    MissingOperandReference {
        evaluation_name: String,
        missing_operand: String,
    },
    CircularDependency {
        evaluation_name: String,
        dependency_chain: Vec<String>,
    },
    InvalidWeight {
        evaluation_name: String,
        weight: i32,
    },
    MissingRequiredField {
        evaluation_name: String,
        field_name: String,
    },
    InvalidLogicalOperator {
        evaluation_name: String,
        operator: String,
    },
    EmptyOperands(String),
    InvalidStringReference {
        evaluation_name: String,
        field_name: String,
        reference: String,
    },
    InvalidDateTimeExpression {
        evaluation_name: String,
        field_name: String,
        expression: String,
        reason: String,
    },
    InvalidDurationFormat {
        evaluation_name: String,
        field_name: String,
        duration: String,
        reason: String,
    },

    // Model-level validation errors
    InvalidThreshold {
        value: f64,
        reason: String,
    },

    // Evaluation-specific errors
    InvalidEvaluationType {
        evaluation_name: String,
        found_type: String,
    },
    InvalidComparisonOperator {
        evaluation_name: String,
        operator: String,
    },

    InvalidAggregationType {
        evaluation_name: String,
        aggregation: String,
    },
    InvalidWeightRange {
        evaluation_name: String,
        weight: i32,
    },

    // Action-specific errors
    InvalidActionType {
        action_type: String,
    },
    MissingActionReason {
        action_type: String,
    },

    // Metadata validation errors
    InvalidMetadataFormat {
        field: String,
        reason: String,
    },

    MissingRequiredSchemaField {
        field: String,
    },
}


// Combined error type to handle both parser and analyzer errors
#[derive(Debug)]
pub enum ValidationError {
    Parser(ParserError),
    Analyzer(AnalyzerError),
}

impl From<ParserError> for ValidationError {
    fn from(error: ParserError) -> Self {
        ValidationError::Parser(error)
    }
}

impl From<AnalyzerError> for ValidationError {
    fn from(error: AnalyzerError) -> Self {
        ValidationError::Analyzer(error)
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Parser(e) => write!(f, "Parser error: {}", e),
            ValidationError::Analyzer(e) => write!(f, "Analyzer error: {:?}", e),
        }
    }
}



#[derive(Debug)]
pub enum FileValidationError {
    FileNotFound(String),
    FileReadError { path: String, error: std::io::Error },
    InvalidUtf8 { path: String },
    ValidationErrors(ValidationReport),
}

impl std::fmt::Display for FileValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileValidationError::FileNotFound(path) => {
                write!(f, "File not found: {}", path)
            }
            FileValidationError::FileReadError { path, error } => {
                write!(f, "Error reading file {}: {}", path, error)
            }
            FileValidationError::InvalidUtf8 { path } => {
                write!(f, "File {} contains invalid UTF-8", path)
            }
            FileValidationError::ValidationErrors(report) => {
                write!(f, "Validation errors in file:\n{}", report.format_errors())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::validator::RuleValidator;

    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, filename: &str, content: &str) -> std::io::Result<()> {
        let file_path = dir.join(filename);
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    #[test]
    fn test_validate_valid_file() {
        let temp_dir = TempDir::new().unwrap();
        let valid_rule = r#"{
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

        create_test_file(temp_dir.path(), "valid_rule.json", valid_rule).unwrap();
        let file_path = temp_dir.path().join("valid_rule.json");

        let mut validator = RuleValidator::new();
        let result = validator.validate_with_report_from_file(&file_path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_valid());
    }

    #[test]
    fn test_validate_invalid_file() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_rule = r#"{
            "model_id": "TEST001",
            "name": "Invalid Rule",
            "threshold": 1.5,  // Invalid threshold
            "evaluations": [],  // Empty evaluations
            "actions": []
        }"#;

        create_test_file(temp_dir.path(), "invalid_rule.json", invalid_rule).unwrap();
        let file_path = temp_dir.path().join("invalid_rule.json");

        let mut validator = RuleValidator::new();
        let result = validator.validate_with_report_from_file(&file_path);
        assert!(matches!(result, Err(FileValidationError::ValidationErrors(_))));
    }

    #[test]
    fn test_validate_directory() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create multiple test files
        let valid_rule = r#"{
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

        let invalid_rule = r#"{
            "model_id": "TEST002",
            "name": "Invalid Rule",
            "threshold": 1.5,
            "evaluations": [],
            "actions": []
        }"#;

        create_test_file(temp_dir.path(), "valid_rule.json", valid_rule).unwrap();
        create_test_file(temp_dir.path(), "failed_rule.json", invalid_rule).unwrap();
        create_test_file(temp_dir.path(), "not_json.txt", "not a json file").unwrap();

        let mut validator = RuleValidator::new();
        let results = validator.validate_directory(temp_dir.path());

        // Should only process .json files
        assert_eq!(results.len(), 2);
        
        // Check results
        let mut valid_found = false;
        let mut invalid_found = false;

        

        for (file_name, result) in results {
            if file_name.ends_with("valid_rule.json") {
                valid_found = true;
                assert!(result.is_ok());
            } else if file_name.ends_with("failed_rule.json") {
                invalid_found = true;
                assert!(matches!(result, Err(FileValidationError::ValidationErrors(_))));
            }
        }

        assert!(valid_found && invalid_found);
    }

    #[test]
    fn test_file_not_found() {
        let mut validator = RuleValidator::new();
        let result = validator.validate_with_report_from_file("nonexistent.json");
        assert!(matches!(result, Err(FileValidationError::FileNotFound(_))));
    }

    #[test]
    fn test_invalid_utf8_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid_utf8.json");
        
        // Create file with invalid UTF-8
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0xFF, 0xFF, 0xFF, 0xFF]).unwrap();

        let mut validator = RuleValidator::new();
        let result = validator.validate_with_report_from_file(&file_path);
        assert!(matches!(result, Err(FileValidationError::FileReadError { .. })));
    }
}