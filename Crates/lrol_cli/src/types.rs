use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct AnalysisReport {
    pub file_path: String,
    pub summary: AnalysisSummary,
    pub details: AnalysisDetails,
    pub warnings: Vec<AnalysisWarning>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct AnalysisSummary {
    pub  total_evaluations: usize,
    pub  evaluation_types: HashMap<String, usize>,
    pub  max_evaluation_depth: usize,
    pub  dependency_count: usize,
    pub  complexity_score: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct AnalysisDetails {
    pub evaluation_dependencies: HashMap<String, Vec<String>>,
    pub datetime_expressions: Vec<String>,
    pub reference_chains: Vec<Vec<String>>,
    pub evaluation_weights: HashMap<String, i32>,
}

#[derive(Debug, serde::Serialize)]
pub struct AnalysisWarning {
    pub  severity: WarningSeverity,
    pub category: WarningCategory,
    pub message: String,
    pub  context: String,
}

#[derive(Debug, serde::Serialize)]
pub enum WarningSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, serde::Serialize)]
pub enum WarningCategory {
    Complexity,
    Performance,
    Maintainability,
    BestPractice,
}
