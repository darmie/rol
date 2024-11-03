use analyzer_functions::handle_analyze;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use lrol_analyzer::{
    error::*,
    validator::{RuleValidator, ValidationReport},
};
use lrol_parser::ParserError;
use std::path::PathBuf;

mod types;
mod analyzer_functions;

#[derive(Parser)]
#[command(name = "lrol")]
#[command(about = "LROL (Loci Risk Orchestration Language) parser and validator", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse LROL file and display its contents
    Parse {
        /// Path to the LROL JSON file
        #[arg(short, long)]
        file: PathBuf,

        /// Output format (text or json)
        #[arg(short, long, default_value = "text")]
        output: String,

        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Validate LROL file syntax and structure
    Validate {
        /// Path to the LROL JSON file
        #[arg(short, long)]
        file: PathBuf,

        /// Enable verbose output for detailed error messages
        #[arg(short, long)]
        verbose: bool,
    },
    /// Analyze LROL rules for potential issues and provide insights
    Analyze {
        /// Path to the LROL JSON file
        #[arg(short, long)]
        file: PathBuf,

        /// Enable verbose output for detailed analysis
        #[arg(short, long)]
        verbose: bool,

        /// Output format (text or json)
        #[arg(short = 'o', long, default_value = "text")]
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse {
            file,
            output,
            verbose,
        } => handle_parse(file, &output, verbose),
        Commands::Validate { file, verbose } => handle_validate(file, verbose),
        Commands::Analyze {
            file,
            verbose,
            output,
        } => handle_analyze(file, verbose, &output),
    }
}

fn handle_parse(file: PathBuf, output: &str, verbose: bool) -> Result<()> {
    let result = lrol_parser::parse_file(&file)
        .with_context(|| format!("Failed to parse file: {}", file.display()))?;

    match output {
        "json" => {
            let json =
                serde_json::to_string_pretty(&result).context("Failed to serialize to JSON")?;
            println!("{}", json);
        }
        "text" | _ => {
            print_model_summary(&result, verbose);
        }
    }

    Ok(())
}

fn handle_validate(file: PathBuf, verbose: bool) -> Result<()> {
    println!("{}", "Validating LROL file...".cyan());
    let file_path = file.display();

    let mut validator = RuleValidator::new();
    match validator.validate_with_report_from_file(&file) {
        Ok(report) => {
            print_validation_success(&report, verbose);
            Ok(())
        }
        Err(error) => match error {
            FileValidationError::FileNotFound(_) => {
                println!(
                    "{}",
                    format!("✗ File not found: {}", file_path).red().bold()
                );
                std::process::exit(1);
            }
            FileValidationError::FileReadError { path, error } => {
                println!(
                    "{}",
                    format!("✗ Error reading file {}: {}", path, error)
                        .red()
                        .bold()
                );
                std::process::exit(1);
            }
            FileValidationError::InvalidUtf8 { path } => {
                println!(
                    "{}",
                    format!("✗ File contains invalid UTF-8: {}", path)
                        .red()
                        .bold()
                );
                std::process::exit(1);
            }
            FileValidationError::ValidationErrors(report) => {
                print_validation_failure(&report, verbose);
                std::process::exit(1);
            }
        },
    }
}

fn print_validation_success(report: &ValidationReport, verbose: bool) {
    println!("{}", "✓ File is valid LROL".green().bold());

    // Print model summary if available
    if let Some(ref model) = report.model {
        print_model_summary(model, verbose);

        // Print metadata if available
        if let Some(ref metadata) = model.metadata {
            println!("\n{}", "Metadata:".cyan().bold());
            if let Some(ref created_by) = metadata.created_by {
                println!("  Created by: {}", created_by);
            }
            if let Some(ref created_at) = metadata.created_at {
                println!("  Created at: {}", created_at);
            }
            if let Some(ref last_updated) = metadata.last_updated {
                println!("  Last updated: {}", last_updated);
            }
        }
    }
}

fn print_validation_failure(report: &ValidationReport, verbose: bool) {
    println!("{}", "✗ Validation failed".red().bold());

    if let Some(ref file_path) = report.file_path {
        println!("\nFile: {}", file_path.cyan());
    }

    if let Some(ref parser_error) = report.parser_error {
        println!("\n{}", "Parser Errors:".yellow().bold());
        match parser_error {
            ParserError::InvalidSyntax {
                line,
                column,
                message,
            } => {
                println!(
                    "  Line {}, Column {}: {}",
                    line.to_string().cyan(),
                    column.to_string().cyan(),
                    message
                );
            }
            ParserError::MissingField { field } => {
                println!("  Missing required field: {}", field.cyan());
            }
            ParserError::InvalidValue {
                field,
                expected,
                found,
            } => {
                println!(
                    "  Invalid value for {}: expected {}, found {}",
                    field.cyan(),
                    expected.yellow(),
                    found.red()
                );
            }
        }
    }

    if !report.analyzer_errors.is_empty() {
        println!("\n{}", "Analyzer Errors:".yellow().bold());
        for (i, error) in report.analyzer_errors.iter().enumerate() {
            let error_message = match error {
                AnalyzerError::InvalidThreshold { value, reason } => {
                    format!("Invalid threshold {}: {}", value.to_string().red(), reason)
                }
                AnalyzerError::DuplicateEvaluationName(name) => {
                    format!("Duplicate evaluation name: {}", name.red())
                }
                AnalyzerError::MissingOperandReference {
                    evaluation_name,
                    missing_operand,
                } => {
                    format!(
                        "Missing operand reference in {}: {}",
                        evaluation_name.cyan(),
                        missing_operand.red()
                    )
                }
                AnalyzerError::CircularDependency {
                    evaluation_name,
                    dependency_chain,
                } => {
                    format!(
                        "Circular dependency detected in {}: {}",
                        evaluation_name.cyan(),
                        dependency_chain.join(" → ").red()
                    )
                }
                AnalyzerError::InvalidDateTimeExpression {
                    evaluation_name,
                    field_name,
                    expression,
                    reason,
                } => {
                    format!(
                        "Invalid datetime in {} ({}): {} - {}",
                        evaluation_name.cyan(),
                        field_name,
                        expression.red(),
                        reason
                    )
                }
                // Add other error type formatting as needed...
                _ if verbose => {
                    format!("{:?}", error)
                }
                _ => {
                    format!("{:?}", error)
                }
            };
            println!("  {}. {}", (i + 1), error_message);
        }
    }

    if verbose {
        println!("\n{}", "Full Validation Report:".yellow().bold());
        println!("{}", report.format_errors());
    } else {
        println!("\nTip: Use -v for detailed error information");
    }
}

fn print_model_summary(model: &lrol_parser::parser::LrolModel, verbose: bool) {
    println!("{}", "LROL Model Summary".green().bold());
    println!("Model ID: {}", model.model_id);
    println!("Name: {}", model.name);
    if let Some(desc) = &model.description {
        println!("Description: {}", desc);
    }
    println!("Threshold: {}", model.threshold);

    println!("\n{}", "Evaluations:".yellow().bold());
    for eval in &model.evaluations {
        println!(
            "- {} ({})",
            eval.name.bold(),
            format!("{:?}", eval.evaluation_type).to_lowercase()
        );

        if verbose {
            if let Some(left) = &eval.left {
                println!("  Left: {}", left);
            }
            if let Some(operator) = &eval.operator {
                println!("  Operator: {}", operator);
            }
            if let Some(right) = &eval.right {
                println!("  Right: {:?}", right);
            }
            if let Some(weight) = eval.weight {
                println!("  Weight: {}", weight);
            }
            if let Some(operands) = &eval.operands {
                println!("  Operands: {:?}", operands);
            }
        }
    }

    println!("\n{}", "Actions:".yellow().bold());
    for action in &model.actions {
        println!("- {} ({})", action.action_type.bold(), action.reason);
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Parser error: {0}")]
    ParseError(#[from] lrol_parser::ParserError),

    #[error("Output error: {0}")]
    OutputError(String),

    #[error("Invalid output format: {0}")]
    InvalidOutputFormat(String),
}
