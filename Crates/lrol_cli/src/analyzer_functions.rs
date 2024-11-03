use crate::types::{
    AnalysisDetails, AnalysisReport, AnalysisSummary, AnalysisWarning, WarningCategory,
    WarningSeverity,
};
use anyhow::{Context, Result};
use colored::Colorize;
use lrol_analyzer::{analyzer::RuleAnalyzer, validator::RuleValidator};
use lrol_parser::{parser::LrolModel, Evaluation, Value};
use std::{collections::HashMap, path::PathBuf};

pub fn handle_analyze(file: PathBuf, verbose: bool, output: &str) -> Result<()> {
    println!("{}", "Analyzing LROL file...".cyan());
    let file_path = file.display();

    let mut validator = RuleValidator::new();
    match validator.validate_with_report_from_file(&file) {
        Ok(validation_report) => {
            if let Some(model) = validation_report.model {
                let analysis_report = analyze_model(&model, &file_path.to_string());
                print_analysis_report(&analysis_report, verbose, output)?;
                Ok(())
            } else {
                println!("{}", "✗ No model found to analyze".red().bold());
                std::process::exit(1);
            }
        }
        Err(error) => {
            println!(
                "{}",
                format!("✗ Failed to analyze file: {}", error).red().bold()
            );
            std::process::exit(1);
        }
    }
}

fn analyze_model(model: &LrolModel, file_path: &str) -> AnalysisReport {
    // Create analysis report
    let mut summary = AnalysisSummary {
        total_evaluations: model.evaluations.len(),
        evaluation_types: HashMap::new(),
        max_evaluation_depth: 0,
        dependency_count: 0,
        complexity_score: 0.0,
    };

    let mut details = AnalysisDetails {
        evaluation_dependencies: HashMap::new(),
        datetime_expressions: Vec::new(),
        reference_chains: Vec::new(),
        evaluation_weights: HashMap::new(),
    };

    let mut warnings = Vec::new();
    let mut suggestions = Vec::new();

    // Analyze evaluation types
    for eval in &model.evaluations {
        *summary
            .evaluation_types
            .entry(format!("{:?}", eval.evaluation_type))
            .or_insert(0) += 1;

        // Track weights
        if let Some(weight) = eval.weight {
            details.evaluation_weights.insert(eval.name.clone(), weight);
        }

        // Analyze dependencies
        let deps = collect_evaluation_dependencies(eval);
        if !deps.is_empty() {
            details
                .evaluation_dependencies
                .insert(eval.name.clone(), deps);
            summary.dependency_count += 1;
        }

        // Check for datetime expressions
        if let Some(expr) = find_datetime_expression(eval) {
            details.datetime_expressions.push(expr);
        }
    }

    // Calculate complexity score
    summary.complexity_score = calculate_complexity_score(model);

    check_for_warnings(model, &mut warnings);

    generate_suggestions(model, &mut suggestions);

    AnalysisReport {
        file_path: file_path.to_string(),
        summary,
        details,
        warnings,
        suggestions,
    }
}

fn print_analysis_report(report: &AnalysisReport, verbose: bool, output: &str) -> Result<()> {
    match output {
        "json" => {
            let json = serde_json::to_string_pretty(report)
                .context("Failed to serialize analysis report to JSON")?;
            println!("{}", json);
        }
        _ => {
            println!("\n{}", "Analysis Summary:".green().bold());
            println!(
                "  Total Evaluations: {}",
                report.summary.total_evaluations.to_string().cyan()
            );
            println!("  Evaluation Types:");
            for (eval_type, count) in &report.summary.evaluation_types {
                println!("    {}: {}", eval_type, count.to_string().cyan());
            }
            println!(
                "  Complexity Score: {}",
                format!("{:.2}", report.summary.complexity_score).cyan()
            );

            if !report.warnings.is_empty() {
                println!("\n{}", "Warnings:".yellow().bold());
                for warning in &report.warnings {
                    let severity_color = match warning.severity {
                        WarningSeverity::Low => "yellow",
                        WarningSeverity::Medium => "red",
                        WarningSeverity::High => "bright red",
                    };
                    println!(
                        "  {} [{}] {}",
                        format!("{:?}", warning.severity).color(severity_color),
                        format!("{:?}", warning.category).cyan(),
                        warning.message
                    );
                    if verbose {
                        println!("    Context: {}", warning.context);
                    }
                }
            }

            if !report.suggestions.is_empty() {
                println!("\n{}", "Suggestions:".blue().bold());
                for suggestion in &report.suggestions {
                    println!("  • {}", suggestion);
                }
            }

            if verbose {
                println!("\n{}", "Detailed Analysis:".cyan().bold());
                println!("  Dependency Graph:");
                for (eval, deps) in &report.details.evaluation_dependencies {
                    println!("    {} → {}", eval, deps.join(", "));
                }

                if !report.details.datetime_expressions.is_empty() {
                    println!("\n  DateTime Expressions:");
                    for expr in &report.details.datetime_expressions {
                        println!("    • {}", expr);
                    }
                }
            }
        }
    }

    Ok(())
}

fn collect_evaluation_dependencies(eval: &Evaluation) -> Vec<String> {
    let mut deps = Vec::new();

    // Check logical operands
    if let Some(ref operands) = eval.operands {
        deps.extend(operands.clone());
    }

    // Check string references
    if let Some(ref left) = eval.left {
        deps.extend(RuleAnalyzer::extract_references(left));
    }

    if let Some(Value::String(ref right)) = eval.right {
        deps.extend(RuleAnalyzer::extract_references(right));
    }

    deps
}

fn find_datetime_expression(eval: &Evaluation) -> Option<String> {
    let check_expr = |s: &str| {
        if s.starts_with("datetime(") {
            Some(s.to_string())
        } else {
            None
        }
    };

    if let Some(ref left) = eval.left {
        if let Some(expr) = check_expr(left) {
            return Some(expr);
        }
    }

    if let Some(Value::String(ref right)) = eval.right {
        if let Some(expr) = check_expr(right) {
            return Some(expr);
        }
    }

    None
}

fn calculate_complexity_score(model: &LrolModel) -> f64 {
    let mut score = 0.0;

    // Base score from number of evaluations
    score += model.evaluations.len() as f64 * 1.0;

    // Add complexity for dependencies
    for eval in &model.evaluations {
        if let Some(ref operands) = eval.operands {
            score += operands.len() as f64 * 0.5;
        }
        // Add complexity for string references
        if let Some(ref left) = eval.left {
            score += RuleAnalyzer::extract_references(left).len() as f64 * 0.3;
        }
        if let Some(Value::String(ref right)) = eval.right {
            score += RuleAnalyzer::extract_references(right).len() as f64 * 0.3;
        }
        // Add complexity for datetime expressions
        if find_datetime_expression(eval).is_some() {
            score += 0.5;
        }
    }

    score
}

fn check_for_warnings(model: &LrolModel, warnings: &mut Vec<AnalysisWarning>) {
    // Check complexity
    if model.evaluations.len() > 10 {
        warnings.push(AnalysisWarning {
            severity: WarningSeverity::Medium,
            category: WarningCategory::Complexity,
            message: "High number of evaluations may impact maintainability".to_string(),
            context: format!("Total evaluations: {}", model.evaluations.len()),
        });
    }

    for action in &model.actions {
        if !["flag_transaction", "block_transaction", "send_alert"]
            .contains(&action.action_type.as_str())
        {
            warnings.push(AnalysisWarning {
                severity: WarningSeverity::Medium,
                category: WarningCategory::BestPractice,
                message: "While user defined action types are accepted. We recommend that you use one the following action types 'flag_transaction', 'block_transaction' or 'send_alert'".to_string(),
                context: format!("Action type: '{}'", action.action_type),
            });
        }
    }

    // Check for deep dependency chains
    // let max_chain = find_longest_dependency_chain(model);
    // if max_chain.len() > 3 {
    //     warnings.push(AnalysisWarning {
    //         severity: WarningSeverity::Medium,
    //         category: WarningCategory::Performance,
    //         message: "Deep dependency chain detected".to_string(),
    //         context: format!("Longest chain: {}", max_chain.join(" → ")),
    //     });
    // }

    // TODO: Add more warning checks as needed...
}

fn generate_suggestions(model: &LrolModel, suggestions: &mut Vec<String>) {
    // Suggest documentation if description is missing
    if model.description.is_none() {
        suggestions.push("Consider adding a description to improve rule documentation".to_string());
    }

    // Suggest weight normalization
    let weights: Vec<_> = model.evaluations.iter().filter_map(|e| e.weight).collect();
    if !weights.is_empty() && weights.iter().any(|&w| w >= 4) {
        suggestions
            .push("Consider normalizing evaluation weights to improve rule balance".to_string());
    }

    // Add more suggestions as needed...
}

// fn find_longest_dependency_chain(model: &LrolModel) -> Vec<String> {
//     // Implementation of finding the longest dependency chain
//     Vec::new()
// }
