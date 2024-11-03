# Sudden spike in card activity, a deviation from past average transactions

Here's the rule definition for the scenario where an entity suddenly starts making a larger volume of payments within a short time (2 hours), and the total value of the last 3 payments exceeds the average payment value over the past 2 days (you could increase the look back period). This rule is designed to detect suspicious spikes in transaction activity that could indicate card payment fraud. 


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

### Summary

This rule monitors for sudden spikes in spending that are much higher than usual for the cardholder. By checking both the amount spent in a short time and comparing recent transactions to past averages, it detects patterns typical in card fraud. When both conditions are met, the rule flags the transaction as suspicious, helping detect and respond to possible fraud quickly.