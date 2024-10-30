## LROL (ROL) JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Loci Risk Orchestration Language (LROL)",
  "description": "A JSON schema for defining risk models in LROL",
  "type": "object",
  "properties": {
    "model_id": {
      "type": "string",
      "description": "A unique identifier for the risk model"
    },
    "name": {
      "type": "string",
      "description": "The name of the rule"
    },
    "description": {
      "type": "string",
      "description": "A brief description of the rule"
    },
    "threshold": {
      "type": "number",
      "description": "Threshold for the rule's score that determines if an action is triggered",
      "minimum": 0,
      "maximum": 1
    },
    "evaluations": {
      "type": "array",
      "description": "List of evaluations to be performed in the rule",
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "description": "A human-readable name for the evaluation"
          },
          "type": {
            "type": "string",
            "enum": ["comparison", "aggregation", "logical", "time-based", "conditional"],
            "description": "The type of evaluation being performed"
          },
          "left": {
            "type": ["string", "number"],
            "description": "The left operand in the evaluation"
          },
          "operator": {
            "type": "string",
            "enum": [">", "<", ">=", "<=", "==", "!=","IN", "NOT IN", "LIKE", "NOT LIKE"],
            "description": "Comparison operator to use in the evaluation"
          },
          "right": {
            "type": ["string", "number"],
            "description": "The right operand in the evaluation"
          },
          "aggregation": {
            "type": "string",
            "enum": ["SUM", "COUNT", "AVG", "MIN", "MAX", "STDDEV"],
            "description": "The aggregation function to be used (only applies to aggregation type evaluations)"
          },
          "conditions": {
            "type": "array",
            "description": "Optional conditions for filtering data",
            "items": {
              "type": "object",
              "properties": {
                "type": {
                  "type": "string",
                  "enum": ["comparison"],
                  "description": "The type of condition (e.g., comparison)"
                },
                "left": {
                  "type": ["string", "number"],
                  "description": "The left operand in the condition"
                },
                "operator": {
                  "type": "string",
                  "enum": [">", "<", ">=", "<=", "==", "!="],
                  "description": "Comparison operator for the condition"
                },
                "right": {
                  "type": ["string", "number"],
                  "description": "The right operand in the condition"
                }
              },
              "required": ["type", "left", "operator", "right"]
            }
          },
          "weight": {
            "type": "integer",
            "description": "The importance of the evaluation in the overall score (1 to 5)",
            "minimum": 1,
            "maximum": 5
          }
        },
        "required": ["type"]
      }
    },
    "actions": {
      "type": "array",
      "description": "List of actions to be performed if the rule's threshold is met",
      "items": {
        "type": "object",
        "properties": {
          "type": {
            "type": "string",
            "enum": ["flag_transaction", "block_transaction", "send_alert"],
            "description": "The type of action to take when the rule is triggered"
          },
          "reason": {
            "type": "string",
            "description": "The reason for performing this action"
          }
        },
        "required": ["type", "reason"]
      }
    },
    "metadata": {
      "type": "object",
      "description": "Optional metadata for the rule",
      "properties": {
        "created_by": {
          "type": "string",
          "description": "The person or system that created the rule"
        },
        "created_at": {
          "type": "string",
          "format": "date-time",
          "description": "Timestamp for when the rule was created"
        },
        "last_updated": {
          "type": "string",
          "format": "date-time",
          "description": "Timestamp for the last update to the rule"
        },
        "notes": {
          "type": "string",
          "description": "Any additional notes about the rule"
        }
      }
    }
  },
  "required": ["model_id", "name", "evaluations", "actions"]
}

```

### Key Elements:

1. **`model_id`**: A unique identifier for each risk model.
2. **`name`**: A human-readable name for the rule.
3. **`description`**: A brief description of what the rule is designed to achieve.
4. **`threshold`**: A score threshold that determines whether the rule's action should be triggered (scaled between 0 and 1).
5. **`evaluations`**: This is where the core logic of the rule is defined. Evaluations can be of several types (e.g., comparison, aggregation, logical, time-based, or conditional).
6. **`actions`**: Defines what should happen if the rule’s conditions are met (e.g., flagging, blocking transactions, or sending alerts).

---

## Not Just Simple Rules

The output of LROL can be viewed as a **model** rather than just a simple rule. Here’s the reasoning:

1. **Complexity and Flexibility**:
    - LROL supports multiple types of evaluations, including comparisons, aggregations, logical operations, and time-based conditions. These evaluations can interact and depend on each other to create sophisticated detection mechanisms.
    - A typical “rule” in traditional systems tends to be more static or isolated (e.g., “If X happens, do Y”). LROL, on the other hand, can represent complex, multi-layered decision-making processes, making it resemble a **risk model** rather than a basic rule.
2. **Scoring and Thresholds**:
    - LROL includes features like **weights**, **scoring**, and **thresholds**, which are typically characteristics of models, particularly in risk and fraud management. These features allow for more dynamic evaluations, with decisions based on aggregated scores rather than simple binary outcomes (true/false).
    - In many ways, this scoring mechanism resembles how **machine learning models** produce a confidence score for predictions.
3. **Multi-Dimensional Evaluation**:
    - LROL is designed to evaluate multiple factors across different dimensions (transaction amount, time-based conditions, account age, etc.). This multi-dimensional analysis, combining various metrics and thresholds, is more indicative of how **models** work, as models evaluate inputs based on various parameters rather than a single condition.