pub mod error;
pub mod parser;
pub mod types;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub use error::{ParserError, ParserResult};
pub use parser::LrolParser;
pub use types::{Action, Evaluation, EvaluationType, Value};

/// Parses LROL content from a string
pub fn parse_str(content: &str) -> ParserResult<parser::LrolModel> {
    LrolParser::parse(content)
}

/// Parses LROL content from a file
pub fn parse_file<P: AsRef<Path>>(path: P) -> ParserResult<parser::LrolModel> {
    let mut file = File::open(path).map_err(|e| ParserError::InvalidSyntax {
        line: 0,
        column: 0,
        message: format!("Failed to open file: {}", e),
    })?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| ParserError::InvalidSyntax {
            line: 0,
            column: 0,
            message: format!("Failed to read file: {}", e),
        })?;

    parse_str(&content)
}
