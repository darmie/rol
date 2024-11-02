use std::fmt;

use nom::error::{VerboseError, VerboseErrorKind};

#[derive(Debug)]
pub enum ParserError {
    InvalidSyntax {
        line: usize,
        column: usize,
        message: String,
    },
    MissingField {
        field: String,
    },
    InvalidValue {
        field: String,
        expected: String,
        found: String,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidSyntax {
                line,
                column,
                message,
            } => write!(
                f,
                "Syntax error at line {}, column {}: {}",
                line, column, message
            ),
            ParserError::MissingField { field } => write!(f, "Missing required field: {}", field),
            ParserError::InvalidValue {
                field,
                expected,
                found,
            } => write!(
                f,
                "Invalid value for field {}: expected {}, found {}",
                field, expected, found
            ),
        }
    }
}

impl std::error::Error for ParserError {}

pub type ParserResult<T> = Result<T, ParserError>;

// Helper function to get line and column from input and position
fn get_error_position(full_input: &str, error_input: &str) -> (usize, usize) {
    // Calculate the offset where the error occurred
    let offset = full_input.len() - error_input.len();
    let mut line = 1;
    let mut column = 1;
    let mut current_line_start = 0;

    // Count lines and track last line start
    for (pos, ch) in full_input[..offset].chars().enumerate() {
        if ch == '\n' {
            line += 1;
            current_line_start = pos + 1;
        }
    }

    // Calculate column
    column = offset - current_line_start + 1;

    (line, column)
}

// Helper function to convert nom error to our custom error
pub fn convert_nom_error(full_input: &str, e: VerboseError<&str>) -> ParserError {
    // Find the last (most specific) error
    let error = e.errors.last().map(|(error_input, kind)| {
        let (line, column) = get_error_position(full_input, error_input);
        let message = match kind {
            VerboseErrorKind::Char(c) => format!("Unexpected character: '{}'", c),
            VerboseErrorKind::Context(ctx) => format!("Error in context: {}", ctx),
            VerboseErrorKind::Nom(kind) => format!("Parse error: {:?}", kind),
        };
        (line, column, message)
    }).unwrap_or((1, 1, "Unknown parse error".to_string()));

    ParserError::InvalidSyntax {
        line: error.0,
        column: error.1,
        message: error.2,
    }
}
