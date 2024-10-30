Here are sample **LROL rule definitions** to illustrate fraud detection and risk management use cases. These examples showcase the flexibility of LROL in defining rules for various scenarios.

---

### 1. **High-Value Cross-Border Payment Detection (Fraud Detection)**

Detects high-value cross-border payments that could indicate potential fraud.

```json
{
  "model_id": "FRAUD-CB-001",
  "name": "High-Value Cross-Border Payments",
  "description": "Flags transactions over $10,000 originating from high-risk countries.",
  "threshold": 0.9,
  "evaluations": [
    {
      "name": "Amount_Check",
      "type": "comparison",
      "left": "transaction_amount",
      "operator": ">",
      "right": 10000,
      "weight": 4
    },
    {
      "name": "Country_Check",
      "type": "comparison",
      "left": "origin_country",
      "operator": "IN",
      "right": ["Country_X", "Country_Y"],
      "weight": 3
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "High-value transaction flagged from high-risk country."
    }
  ]
}
```

---

### 2. **Transaction Velocity Monitoring (Fraud Detection)**

Monitors the frequency of transactions within a short timeframe to detect abnormal transaction velocity.

```json
{
  "model_id": "FRAUD-VEL-002",
  "name": "Transaction Velocity Check",
  "description": "Flags accounts with high transaction volumes in a short period.",
  "threshold": 0.85,
  "evaluations": [
    {
      "name": "Transaction_Sum",
      "type": "aggregation",
      "aggregation": "SUM",
      "field": "transaction_amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "datetime(transaction.timestamp)",
          "operator": ">=",
          "right": "datetime(now, '-15 minutes')"
        }
      ],
      "weight": 5
    },
    {
      "name": "Sum_Check",
      "type": "comparison",
      "left": "@Transaction_Sum",
      "operator": ">",
      "right": 5000,
      "weight": 3
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "Unusual transaction volume detected in short timeframe."
    }
  ]
}
```

---

### 3. **Device and Location Anomaly Detection (Fraud Detection)**

Flags transactions originating from unfamiliar devices or unusual locations for a given account.

```json
{
  "model_id": "FRAUD-DEVLOC-003",
  "name": "Device and Location Anomaly",
  "description": "Detects transactions from new devices or unexpected locations.",
  "threshold": 0.8,
  "evaluations": [
    {
      "name": "Device_Check",
      "type": "comparison",
      "left": "device_id",
      "operator": "NOT_IN",
      "right": "user_trusted_devices",
      "weight": 4
    },
    {
      "name": "Location_Check",
      "type": "comparison",
      "left": "location",
      "operator": "NOT_IN",
      "right": "user_frequent_locations",
      "weight": 4
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "Transaction from unrecognized device or location."
    }
  ]
}
```

---

### 4. **Exposure to High-Risk Industries (Risk Management)**

Monitors transactions with businesses in high-risk sectors to evaluate potential regulatory and compliance risks.

```json
{
  "model_id": "RISK-HIGHIND-004",
  "name": "High-Risk Industry Exposure",
  "description": "Flags transactions with entities in high-risk industries.",
  "threshold": 0.75,
  "evaluations": [
    {
      "name": "Industry_Check",
      "type": "comparison",
      "left": "merchant_industry",
      "operator": "IN",
      "right": ["Gambling", "Cryptocurrency"],
      "weight": 3
    },
    {
      "name": "Transaction_Amount_Check",
      "type": "comparison",
      "left": "transaction_amount",
      "operator": ">",
      "right": 10000,
      "weight": 2
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "Transaction flagged due to high-risk industry exposure."
    }
  ]
}
```

---
*Scenario*: Detecting high-value transfers that happen in a short time period from new accounts, combining time-based, comparison, aggregation, and logical evaluations.

```json
{
  "model_id": "M304",
  "name": "Detect New Account High-Frequency Transfers",
  "description": "This rule detects multiple large transactions within a short time period from accounts less than 30 days old.",
  "threshold": 0.85,
  "evaluations": [
    {
      "name": "Account_Age_Check",
      "type": "comparison",
      "left": "account_age_days",
      "operator": "<=",
      "right": 30,
      "weight": 3
    },
    {
      "name": "Recent_Transfers_Sum",
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
      ],
      "weight": 4
    },
    {
      "name": "Compare_Sum_To_Threshold",
      "type": "comparison",
      "left": "@Recent_Transfers_Sum",
      "operator": ">=",
      "right": 10000,
      "weight": 5
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "Multiple high-value transactions detected from a new account within a short time window."
    }
  ]
}

```

### Key Aspects:

1. **Multi-dimensional Feature Extraction**: The rule sums up all transactions from the last hour (`@Recent_Transfers_Sum`) and compares the total to a $10,000 threshold.
2. **Combined Evaluations**: The rule uses comparison (account age and transfer sum), aggregation (sum of recent transactions), and time-based conditions (last hour).
3. **Readable**: The schema remains easy to follow, making it accessible for both technical and non-technical users.

---

These examples provide a practical starting point for implementing LROL across different use cases in fraud detection and risk management. Each rule showcases a combination of evaluations, weights, and actions to tailor detection strategies to specific scenarios.