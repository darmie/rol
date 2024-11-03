# ROL
LROL is a domain-specific language that simplifies the creation of fraud/risk detection rules by using a structured JSON-based format.

## Overview

**LROL** (Loci Risk Orchestration Language) is a flexible, JSON-based domain-specific language designed for defining, managing, and evaluating risk models in real-time systems. It allows organizations to handle complex fraud detection, transaction monitoring, and risk management tasks. LROL provides a standard way to write rules that can be deployed across multiple platforms like Apache Flink, Spark, and others, simplifying rule management and improving scalability.

## Prerequisites

Before you begin, ensure you have the following:

- Basic understanding of JSON structure and syntax.
- A working system with tools or platforms that can evaluate JSON rules (e.g., Apache Flink, Kafka, or custom real-time engines).
- Knowledge of fraud detection and transaction monitoring use cases is recommended but not mandatory.

---

## Description

LROL helps teams like fraud analysts, compliance officers, and engineers collaborate effectively by using a unified, platform-agnostic rule language. It is built for **real-time fraud detection**, with features for:

- Writing complex rule definitions.
- Defining multi-dimensional risk evaluations (e.g., comparison, aggregation, time-based evaluations).
- Incorporating dynamic, machine learning-driven anomaly scoring.
- Providing an easy-to-maintain JSON structure that can scale across systems.

---

## Why Use It?

- **Unified Language**: Provides a common, standardized format for teams to write and document fraud detection rules.
- **Platform Independence**: Works across various platforms (Flink, Spark, Kafka) without needing to rewrite rules.
- **Scalability**: Handles both simple and complex fraud detection models.
- **Operational Efficiency**: Reduces redundancy and improves accuracy with centralized rule management.
- **Advanced Fraud Detection**: Combines machine learning-driven insights and human-defined rules for a robust, adaptable system.

---

## When to Use It?

- When you need **real-time fraud detection** for transactions, payments, or financial operations.
- When managing fraud rules across multiple platforms (e.g., real-time and batch processing systems).
- When creating flexible and transparent fraud detection systems that comply with audit and regulatory requirements.
- When you need to **scale fraud detection rules** across different teams or systems while maintaining consistency.

---

## Examples

Here is an example of an **LROL rule** for detecting high-value transactions initiated via open banking APIs:
```json
{
  "model_id": "M501",
  "name": "High-Risk Open Banking Transactions",
  "description": "Detects high-value transactions initiated by third-party apps",
  "threshold": 0.9,
  "evaluations": [
    {
      "name": "Transaction_Amount_Check",
      "type": "comparison",
      "left": "transaction_amount",
      "operator": ">",
      "right": 10000,
      "weight": 4
    },
    {
      "name": "Account_Age_Check",
      "type": "comparison",
      "left": "account_age_days",
      "operator": "<=",
      "right": 30,
      "weight": 3
    },
    {
      "name": "High_Risk_Transaction_Logic",
      "type": "logical",
      "operator": "AND",
      "operands": [
        "Transaction_Amount_Check",
        "Account_Age_Check"
      ],
      "weight": 5
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "Transaction flagged due to high value and new account."
    }
  ]
}

```
---
## Usage

1. **Define Rules**: Create fraud detection rules in JSON format, specifying evaluations like comparison, aggregation, and time-based conditions.
2. **Deploy Rules**: Integrate these rules with your real-time processing system (e.g., Flink, Spark) to evaluate transactions as they happen.
3. **Evaluate**: Use the rule’s threshold and evaluation scores to determine actions like flagging or blocking suspicious transactions.
4. **Extend with ML**: Optionally, integrate machine learning-driven scores or anomaly detection features into your LROL evaluations to enhance decision-making.

---
## Developer Tools & Libries
1. [**LROL PARSER**](./Crates/lrol_parser/)
2. [**LROL ANALYZER & VALIDATOR**](./Crates/lrol_analyzer/)
3. [**LROL CLI**](./lrol-cli.md)