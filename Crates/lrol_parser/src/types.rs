use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Evaluation {
    pub name: String,
    pub evaluation_type: EvaluationType,
    pub left: Option<String>,
    pub operator: Option<String>,
    pub right: Option<Value>,
    pub operands: Option<Vec<String>>,
    pub weight: Option<i32>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Action {
    pub action_type: String,
    pub reason: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}