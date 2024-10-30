### **Evaluation Types and Field Breakdown**

LROL evaluations are flexible, enabling a variety of conditions and methods to detect risk or fraud scenarios. There are three primary evaluation types: **Comparison**, **Aggregation**, and **Logical Conditions**. Each type has specific fields that define how evaluations are conducted and what conditions they assess.

---

#### **Comparison Evaluations**
Comparison evaluations are the most basic type and involve checking if a field meets a specific condition.

- **Fields**:
  - `name`: A unique label for the evaluation.
  - `type`: Set to `"comparison"` for this type.
  - `left`: The field being evaluated (e.g., `transaction_amount`).
  - `operator`: The operator for comparison (`>`, `<`, `=`, etc.).
  - `right`: The value being compared against the field.

- **Example**:
  ```json
  {
    "name": "Amount_Check",
    "type": "comparison",
    "left": "transaction_amount",
    "operator": ">",
    "right": 5000
  }
  ```

This example evaluates if the transaction amount exceeds $5,000.

---

#### **Aggregation Evaluations**
Aggregation evaluations examine cumulative data within a specific timeframe, such as the sum of transaction amounts.

- **Fields**:
  - `name`: Descriptive label for the evaluation.
  - `type`: Set to `"aggregation"`.
  - `aggregation`: Defines the aggregation type (e.g., `SUM`, `COUNT`).
  - `field`: The field being aggregated (e.g., `transaction_amount`).
  - `conditions`: Additional filters for refining the aggregation, such as a timeframe.

- **Example**:
  ```json
  {
    "name": "High_Velocity_Check",
    "type": "aggregation",
    "aggregation": "SUM",
    "field": "transaction_amount",
    "conditions": [
      {
        "type": "comparison",
        "left": "datetime(transaction.timestamp)",
        "operator": ">=",
        "right": "datetime(now, '-1 hour')"
      }
    ]
  }
  ```

This example sums transaction amounts over the past hour to identify high-velocity transactions.

---

#### **Logical Conditions**
Logical conditions combine multiple evaluations, allowing complex rule definitions using `AND` and `OR` operators.

- **Fields**:
  - `name`: Name of the condition for easy identification.
  - `type`: Set to `"logical"`.
  - `operator`: Logical operator (`AND`, `OR`) to combine evaluations.
  - `evaluations`: An array of evaluations to be included in the condition.

- **Example**:
  ```json
  {
    "name": "High_Value_And_Risk_Country",
    "type": "logical",
    "operator": "AND",
    "evaluations": [
      {
        "name": "Amount_Check",
        "type": "comparison",
        "left": "transaction_amount",
        "operator": ">",
        "right": 10000
      },
      {
        "name": "Country_Check",
        "type": "comparison",
        "left": "origin_country",
        "operator": "IN",
        "right": ["Country_X", "Country_Y"]
      }
    ]
  }
  ```

In this case, the transaction is flagged if it is over $10,000 **and** originates from specified high-risk countries.

---
### Conditional Case Evaluation

**Conditional Case** evaluations in LROL enable complex branching logic based on **Boolean conditions**, allowing rules to follow specific pathways depending on criteria outcomes. This is similar to a **CASE statement** in SQL or conditional branching in programming, which can include **if-else** logic. 

#### Fields
- **name**: Descriptive label for easy identification.
- **type**: Set to `"conditional_case"`.
- **if**: Defines the initial condition to evaluate (e.g., `amount > 5000`).
- **else**: Specifies the alternative condition if the initial check fails.
- **input types**: Accepts strings, numbers, Booleans, or references to other operations.

#### Example
```json
{
  "name": "High_Risk_Amount_Case",
  "type": "conditional_case",
  "if": {
    "type": "comparison",
    "left": "transaction_amount",
    "operator": ">",
    "right": 10000
  },
  "then": {
    "type": "flag_transaction",
    "reason": "High-risk amount detected"
  },
  "else": {
    "type": "flag_transaction",
    "reason": "Moderate-risk amount"
  }
}
```

### Summary
The variety of evaluation types and their configurable fields allow LROL to adapt to different fraud and risk scenarios. **Comparison evaluations** focus on single-condition checks, **aggregation evaluations** handle cumulative metrics, and **logical conditions** enable complex, multi-layered rules. This modularity allows compliance and risk teams to construct precise, powerful rules that respond to dynamic threats across platforms.