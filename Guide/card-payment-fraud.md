# Sudden spike in card activity, a deviation from past average transactions

Here's the rule definition for a scenario where an entity/customer suddenly starts making a larger volume of payments within a short time (2 hours), and the total value of the last 3 payments exceeds the average payment value over the past 2 days (you could increase the look back period). This rule is designed to detect suspicious spikes in transaction activity that could indicate card payment fraud. 


---

This rule leverages multi-modal feature extraction and dynamic referencing with statistical checks like summing, averaging, and thresholding. It combines short-term patterns with historical context to detect anomalies, embodying principles similar to anomaly detection models in ML

---

```json
{
  "model_id": "FRAUD-VOL-003",
  "name": "Sudden Increase in Payment Volume and Value",
  "description": "Flags entities making high-volume payments in a short period, with recent payment totals exceeding average payment values over the last 2 days.",
  "threshold": 0.9,
  "evaluations": [
    {
      "name": "Recent_Payments_Sum",
      "type": "aggregation",
      "aggregation": "SUM",
      "field": "transaction_amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "transaction_date",
          "operator": ">=",
          "right": "datetime(now, '-2 hours')"
        }
      ],
      "weight": 4
    },
    {
      "name": "Last_3_Payments_Sum",
      "type": "aggregation",
      "aggregation": "SUM",
      "field": "transaction_amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "transaction_date",
          "operator": ">=",
          "right": "datetime(now, '-2 hours')"
        }
      ],
      "weight": 4,
      "limit": 3
    },
    {
      "name": "Average_Payment_Value_Last_2_Days",
      "type": "aggregation",
      "aggregation": "AVG",
      "field": "transaction_amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "transaction_date",
          "operator": ">=",
          "right": "datetime(now, '-2 days')"
        }
      ],
      "weight": 3
    },
    {
      "name": "High_Value_Recent_Payments_Check",
      "type": "comparison",
      "left": "@Last_3_Payments_Sum",
      "operator": ">",
      "right": "@Average_Payment_Value_Last_2_Days",
      "weight": 5
    },
    {
      "name": "High_Payment_Volume_Logic",
      "type": "logical",
      "operator": "AND",
      "operands": [
        "Recent_Payments_Sum",
        "High_Value_Recent_Payments_Check"
      ],
      "weight": 5
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "Sudden increase in payment volume and value in a short period detected."
    }
  ]
}
```

---

### Goal of the Rule
The rule looks for a sudden increase in:

The total amount of payments being made in a short time (2 hours).
The value of the last few payments compared to what’s normal for the cardholder over the past 2 days.
Such behavior could mean someone is using the card to make high-value purchases or a series of quick, large transactions, which can be signs of fraud.

### Rule Breakdown
The rule is split into different checks, or evaluations. Each evaluation focuses on one part of the transaction activity, and together they help decide if the activity looks suspicious.

1. **Recent_Payments_Sum**: Aggregates the total transaction amount for all payments made within the last **2 hours**, capturing a short-term increase in volume.

2. **Last_3_Payments_Sum**: Aggregates the sum of the last **3 payments** within the last **2 hours**, focusing on the most recent transactions to detect high values over a brief span.

3. **Average_Payment_Value_Last_2_Days**: Calculates the average payment amount over the last **2 days**, providing a baseline for typical transaction values.

4. **High_Value_Recent_Payments_Check**: Compares the sum of the last 3 payments to the **2-day average**. If the recent payments exceed this average, it indicates a potential anomaly.

5. **High_Payment_Volume_Logic**: Combines `Recent_Payments_Sum` and `High_Value_Recent_Payments_Check` using `AND` to ensure that both high volume and high value are present before triggering the rule.

## Enhanced Payment Volume and Risk-Based Anomaly Detection

This rule targets fraud patterns that exhibit sudden changes in transaction volume and unusual transaction times, as well as significant deviations from typical transaction amounts.
---

```json
{
  "model_id": "FRAUD-VOL-003",
  "name": "Enhanced Payment Volume and Risk-Based Anomaly Detection",
  "description": "Detects sudden increase in payment volumes with dynamic risk levels and statistical anomaly detection.",
  "threshold": 0.9,
  "evaluations": [
    {
      "name": "Night_Time_Transaction",
      "type": "comparison",
      "left": "strftime('%H', transaction_date)",
      "operator": "NOT IN",
      "right": ["06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20"],
      "weight": 2
    },
    {
      "name": "Historical_Payment_Avg",
      "type": "aggregation",
      "aggregation": "AVG",
      "field": "amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "transaction_date",
          "operator": ">=",
          "right": "datetime(now, '-30 days')"
        }
      ],
      "weight": 2
    },
    {
      "name": "Transaction_STDDEV",
      "type": "aggregation",
      "aggregation": "STDDEV",
      "field": "amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "transaction_date",
          "operator": ">=",
          "right": "datetime(now, '-30 days')"
        }
      ],
      "weight": 2
    },
    {
      "name": "STDDEV_Anomaly_Check",
      "type": "comparison",
      "left": "amount",
      "operator": ">",
      "right": "Historical_Payment_Avg + 2 * Transaction_STDDEV",
      "weight": 4
    },
    {
      "name": "Last_3_Payments_Avg",
      "type": "aggregation",
      "aggregation": "AVG",
      "field": "amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "transaction_date",
          "operator": ">=",
          "right": "datetime(now, '-3 hours')"
        }
      ],
      "weight": 3,
      "limit": 3
    },
    {
      "name": "Recent_Payments_Sum_Short",
      "type": "aggregation",
      "aggregation": "SUM",
      "field": "amount",
      "conditions": [
        {
          "type": "comparison",
          "left": "transaction_date",
          "operator": ">=",
          "right": "datetime(now, '-3 hours')"
        }
      ],
      "weight": 3
    },
    {
      "name": "High_Deviation_Check",
      "type": "comparison",
      "left": "@Last_3_Payments_Avg",
      "operator": ">",
      "right": "Historical_Payment_Avg * 2",
      "weight": 3
    },
    {
      "name": "Risk_Level_Assessment",
      "type": "conditional",
      "if": [
        {
          "condition":  {
              "type": "logical",
              "operator": "AND",
              "operands": [
                "Night_Time_Transaction",
                "High_Deviation_Check"
              ],
            "weight": 4
        },
          "result": "critical"
        }
            ],
      "else": "low"
    },
    {
      "name": "Critical_Risk_Check",
      "type": "comparison",
      "left": "@Risk_Level_Assessment",
      "operator": "=",
      "right": "critical",
      "weight": 5
    },
    {
      "name": "High_Risk_Transaction_Logic",
      "type": "logical",
      "operator": "AND",
      "operands": [
        "Recent_Payments_Sum_Short",
        "Last_3_Payments_Avg",
        "STDDEV_Anomaly_Check",
        "Critical_Risk_Check"
      ],
      "weight": 5
    }
  ],
  "actions": [
    {
      "type": "flag_transaction",
      "reason": "Unusual transaction volume and risk pattern detected based on risk level and anomaly."
    }
  ]
}
```

---
#### Key Components and Their Roles

- **Night_Time_Transaction**: Checks if the transaction time falls outside typical business hours (between 6 AM and 8 PM). Transactions outside this range are flagged, as unusual times are often associated with fraud. This has a lower weight (2), indicating it’s not conclusive by itself.

- **Historical_Payment_Avg** and **Transaction_STDDEV**: These aggregations calculate the **average transaction amount** and the **standard deviation** for the entity over the last 30 days, giving a baseline for detecting significant deviations. This allows for **contextual anomaly detection**, flagging only those transactions that deviate substantially from the usual pattern.

- **STDDEV_Anomaly_Check**: This comparison flags a transaction if the amount is more than **2 standard deviations above the 30-day average**, a common statistical threshold for identifying anomalies. The higher weight (4) emphasizes its significance as a potential fraud indicator.

- **Last_3_Payments_Avg** and **Recent_Payments_Sum_Short**: These calculate the average and total of the last three transactions within a **3-hour window**. This allows detection of sudden surges in payment activity, which may indicate unusual behavior.

- **High_Deviation_Check**: Compares the average of the last three payments to twice the historical average, flagging recent transactions that are disproportionately high compared to typical activity.

- **Risk_Level_Assessment** (Conditional Case Operation): This component assesses risk dynamically. If both **Night_Time_Transaction** and **High_Deviation_Check** are true (combined under `Critical_Risk_Logic`), the risk level is assigned as `"critical"`. Otherwise, the rule defaults to `"low"`. This component supports flexible, context-aware decision-making.

- **Critical_Risk_Check**: This comparison checks if `Risk_Level_Assessment` has assigned a `"critical"` risk level. If so, this condition contributes heavily to the final decision.

- **High_Risk_Transaction_Logic**: Combines all major risk indicators (including `Critical_Risk_Check` and others) using `AND` logic. The overall rule triggers a **flag_transaction action** if all conditions are met, implying a high likelihood of fraud.

#### Purpose and Solution

This rule effectively identifies transactions that may be part of **fraudulent activity** by:
- Detecting statistically significant deviations in transaction volume.
- Considering the timing and recency of transactions, which are often indicative of fraud.
- Dynamically assessing risk based on contextual factors.

#### Impact on False Positives
- **Reduced False Positives**: By combining multiple factors (e.g., statistical deviation, timing, recent transaction patterns), the rule reduces the chances of flagging benign transactions that meet only one or two criteria.
- **Conditional Scoring**: The conditional scoring (`Risk_Level_Assessment`) helps avoid flagging low-risk transactions that may occur outside typical hours or are high value but otherwise show no signs of fraud.

#### Impact on False Negatives
- **Reduced False Negatives**: The use of **statistical deviation checks** (`STDDEV_Anomaly_Check`) and **dynamic risk assessments** ensures that subtle fraud patterns, such as frequent small payments or transactions that are just above normal limits, are less likely to be overlooked.
- **Sensitive to Combined Risk Factors**: The rule’s design allows it to catch fraud cases that may involve subtle but significant combinations of timing, amount, and recent activity, reducing the chances of missing fraud.

---
## LROL strenghts in use

### 1. **Multi-Modal Feature Extraction**
   - Multi-modal feature extraction involves collecting and analyzing different types of features (or data points) to create a complete picture of the transaction pattern.
   - **Application in the Rule**: Here, the rule extracts multiple features:
     - **Volume-based feature**: The `Recent_Payments_Sum` evaluation aggregates the total spending within a short period, capturing a high-level view of spending volume.
     - **Transaction-based feature**: The `Last_3_Payments_Sum` focuses on the last 3 transactions specifically, detecting high-value spending.
     - **Historical average feature**: The `Average_Payment_Value_Last_2_Days` provides a baseline from past transactions, setting up a comparison point.
   - **Why It’s Important**: By using multi-modal extraction, the rule is able to identify nuanced fraud patterns involving both recent volume and historical spending trends. This multi-faceted view increases accuracy and reduces the risk of false positives.

### 2. **Dynamic Referencing**
   - Dynamic referencing in LROL allows evaluations to reference the output of other evaluations using a `@` symbol, creating relationships between evaluations.
   - **Application in the Rule**: The `High_Value_Recent_Payments_Check` evaluation uses dynamic referencing by comparing `@Last_3_Payments_Sum` against `@Average_Payment_Value_Last_2_Days`. This allows for real-time, conditional comparisons based on recent calculations.
   - **Why It’s Important**: Dynamic referencing ensures that evaluations can use live results from other checks within the same rule. Here, it enables a flexible, data-driven comparison that dynamically adapts based on recent transactions, essential for detecting rapid changes indicative of fraud.

### 3. **Key Features and Statistical Techniques in LROL**
   - Aggregation functions like `SUM` and `AVG` are used here to calculate total spending and average payment values, which are foundational statistical techniques in fraud detection. These give a quantifiable view of spending patterns and make it easy to spot outliers.
   - **Threshold-Based Logic**: Thresholds (e.g., exceeding average spending) are applied to evaluate if current spending significantly deviates from norms. By setting thresholds (like comparing to twice the recent average), the rule flags transactions only when spending exceeds defined limits, minimizing noise.
   - **Real-Time Time Windowing**: Using time-based filters (like checking the last 2 hours and last 2 days) helps in setting precise evaluation windows. This is essential for spotting unusual short-term spikes within a longer trend, a classic signal of fraud.

### Additional Concepts
   - In many ML-driven fraud detection models, creating features based on historical averages, transaction sums, and time windows would be core feature engineering steps. This rule’s use of different evaluations represents feature engineering techniques common in ML.
   - **Anomaly Detection Principles**: The rule mirrors basic anomaly detection by highlighting deviations from an average, a key statistical method that is also fundamental in ML fraud detection. When total spending in the last 3 payments exceeds the 2-day average, it treats the activity as anomalous.

---

### Summary

This rule monitors for sudden spikes in spending that are much higher than usual for the cardholder. By checking both the amount spent in a short time and comparing recent transactions to past averages, it detects patterns typical in card fraud. When both conditions are met, the rule flags the transaction as suspicious, helping detect and respond to possible fraud quickly.