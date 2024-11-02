use std::path::PathBuf;
use clap::{Parser, Subcommand};
use colored::*;
use anyhow::{Context, Result};

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file, output, verbose } => {
            handle_parse(file, &output, verbose)
        }
        Commands::Validate { file, verbose } => {
            handle_validate(file, verbose)
        }
    }
}

fn handle_parse(file: PathBuf, output: &str, verbose: bool) -> Result<()> {
    let result = lrol_parser::parse_file(&file)
        .with_context(|| format!("Failed to parse file: {}", file.display()))?;
    
    match output {
        "json" => {
            let json = serde_json::to_string_pretty(&result)
                .context("Failed to serialize to JSON")?;
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
    
    match lrol_parser::parse_file(&file) {
        Ok(_) => {
            println!("{}", "✓ File is valid LROL".green().bold());
            Ok(())
        }
        Err(e) => {
            println!("{}", "✗ Validation failed".red().bold());
            if verbose {
                println!("\nDetailed error:");
                println!("{}", format!("{:#?}", e).red());
            } else {
                println!("\nError: {}", e.to_string().red());
                println!("\nTip: Use -v for detailed error information");
            }
            std::process::exit(1);
        }
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
        println!("- {} ({})", 
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
        println!("- {} ({})", 
            action.action_type.bold(), 
            action.reason
        );
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