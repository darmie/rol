use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    pub created_by:Option<String>,
    pub created_at:Option<String>,
    pub last_updated:Option<String>,
    pub notes:Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Evaluation {
    pub name: String,
    pub evaluation_type: EvaluationType,
    pub left: Option<String>,
    pub operator: Option<String>,
    pub right: Option<Value>,
    pub operands: Option<Vec<String>>,
    pub weight: Option<i32>,
    pub aggregation: Option<Aggregation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvaluationType {
    Comparison,
    Logical,
    Aggregation,
    TimeBased,
    Conditional,
}

impl FromStr for EvaluationType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "comparison" => Ok(EvaluationType::Comparison),
            "logical" => Ok(EvaluationType::Logical),
            "aggregation" => Ok(EvaluationType::Aggregation),
            "time-based" => Ok(EvaluationType::TimeBased),
            "conditional" => Ok(EvaluationType::Conditional),
            _ => Err("Invalid evaluation type"),
        }
    }
}

impl ToString for EvaluationType {
    fn to_string(&self) -> String {
        match self {
            EvaluationType::Comparison => "comparison".to_string(),
            EvaluationType::Logical => "logical".to_string(),
            EvaluationType::Aggregation => "aggregation".to_string(),
            EvaluationType::TimeBased => "time-based".to_string(),
            EvaluationType::Conditional => "conditional".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Aggregation {
    SUM,
    COUNT,
    AVG,
    MIN,
    MAX,
    STDDEV,
}

impl FromStr for Aggregation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUM" => Ok(Aggregation::SUM),
            "COUNT" => Ok(Aggregation::COUNT),
            "AVG" => Ok(Aggregation::AVG),
            "MIN" => Ok(Aggregation::MIN),
            "MAX" => Ok(Aggregation::MAX),
            "STDDEV" => Ok(Aggregation::STDDEV),
            _ => Err("Invalid aggregation type"),
        }
    }
}

impl ToString for Aggregation{
    fn to_string(&self) -> String {
        match self {
            Aggregation::SUM => "SUM".to_string(),
            Aggregation::COUNT => "COUNT".to_string(),
            Aggregation::AVG => "AVG".to_string(),
            Aggregation::MIN => "MIN".to_string(),
            Aggregation::MAX => "MAX".to_string(),
            Aggregation::STDDEV => "STDDEV".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    pub action_type: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}
