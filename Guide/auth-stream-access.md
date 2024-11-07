
Here's a rule definition to detect transactions in the Auth Stream Access (ASA) that involve either a merchant registered in Connecticut (CT) or a merchant category code that falls into specific high-risk categories (5933 or 5945). It also confirms the entity has sufficient balance to cover teh transaction being authorized 

Here is a sample ASA request
```json
{
  "token": "cf751972-c15d-4b6c-8442-03cb03f0826b",
  "status": "AUTHORIZATION",
  "pos_terminal_attended": false,
  "pos_terminal_operator": "CARDHOLDER",
  "pos_terminal_on_premise": true,
  "pos_terminal_card_retention_capable": false,
  "pos_terminal_pin_capability": "UNSPECIFIED",
  "pos_terminal_type": "ECOMMERCE",
  "pos_terminal_partial_approval_capable": false,
  "pos_entry_mode_pan": "KEY_ENTERED",
  "pos_entry_mode_pin_entered": false,
  "pos_entry_mode_cardholder": "MAIL_ORDER",
  "pos_entry_mode_card": "NOT_PRESENT",
  "settled_amount": 0,
  "created": "2024-10-14T15:10:07Z",
  "amount": 48,
  "acquirer_fee": 0,
  "authorization_amount": 48,
  "card_token": "3c502208-eae2-49ae-bf40-683269405b9e",
  "card_hostname": "",
  "card_last_four": "4849",
  "card_state": "OPEN",
  "card_type": "UNLOCKED",
  "card_memo": "UNLOCKED card",
  "card_spend_limit": 0,
  "card_spend_limit_duration": "TRANSACTION",
  "merchant_descriptor": "coffee shop",
  "merchant_city": "NEW YORK",
  "merchant_state": "NY",
  "merchant_country": "USA",
  "merchant_acceptor_id": "183011111111",
  "merchant_mcc": "4913",
  "avs_zipcode": "75051",
  "avs_address": null,
  "events": [],
  "funding": []
}

```

---


Here’s the rule with the **available balance check** included.

### Revised Rule Definition with Available Balance Check

```json
{
  "model_id": "AUTH-CARD-001",
  "name": "High-Risk Merchant Detection with Balance Check for Card Authorization",
  "description": "Detects transactions with merchants registered in Connecticut (CT) or with high-risk merchant category codes, ensuring the entity has sufficient balance.",
  "threshold": 0.8,
  "evaluations": [
    {
      "name": "Merchant_State_Check",
      "type": "comparison",
      "left": "merchant_state",
      "operator": "==",
      "right": "CT",
      "weight": 3
    },
    {
      "name": "High_Risk_MCC_Check",
      "type": "comparison",
      "left": "merchant_mcc",
      "operator": "IN",
      "right": "highrisk_mcc.code",
      "weight": 3
    },
    {
      "name": "Sufficient_Balance_Check",
      "type": "comparison",
      "left": "profile.available_balance",
      "operator": ">=",
      "right": "amount",
      "weight": 4
    },
    {
      "name": "High_Risk_Merchant_Logic",
      "type": "logical",
      "operator": "AND",
      "operands": [
        {
          "type": "logical",
          "operator": "OR",
          "operands": [
            "Merchant_State_Check",
            "High_Risk_MCC_Check"
          ]
        },
        "Sufficient_Balance_Check"
      ],
      "weight": 5
    }
  ],
  "actions": [
    {
      "type": "deny_transaction",
      "reason": "Deny/Block. Transaction involves a high-risk merchant and entity has sufficient balance."
    }
  ]
}
```

---

### Explanation of the Rule/Model

1. **Merchant_State_Check**:
   - This evaluation checks if the `merchant_state` is `"CT"`, indicating that the merchant is registered in Connecticut.
   - Weighted at 3, it provides a moderate risk indicator but is not sufficient on its own.

2. **High_Risk_MCC_Check**:
   - Uses the lookup table `highrisk_mcc` to check if the `merchant_mcc` is in a pre-populated list of high-risk MCCs.
   - This evaluation is also weighted at 3, representing a strong risk indicator.

3. **Sufficient_Balance_Check**:
   - This evaluation checks if the `available_balance` in the **entity’s profile** (referenced as `profile.available_balance`) is greater than or equal to the **transaction amount** (`amount`).
   - Weighted at 4, this condition ensures that the entity has enough balance to cover the transaction, reducing the chance of authorization if funds are insufficient.

4. **High_Risk_Merchant_Logic**:
   - This logical evaluation combines the **merchant checks** (`Merchant_State_Check` and `High_Risk_MCC_Check`) with the **balance check** (`Sufficient_Balance_Check`).
   - The merchant checks are grouped with an **OR** condition, meaning only one of them needs to be true.
   - The `Sufficient_Balance_Check` is then combined with the merchant logic using an **AND** condition, ensuring the transaction will only proceed if there is sufficient balance and a high-risk merchant condition is met.
   - Weighted at 5, this final evaluation dictates whether the transaction should be flagged.

5. **Flag Transaction Action**:
   - If `High_Risk_Merchant_Logic` evaluates to true, the transaction is flagged due to the presence of a high-risk merchant and sufficient available balance.

---

### Purpose and Use Case

This rule ensures that **high-risk transactions are only flagged if the entity has sufficient balance**:
- The entity’s `available_balance` is checked, preventing unnecessary flags for transactions that would fail due to insufficient funds.
- High-risk merchants are detected based on state or MCC, targeting specific attributes often associated with fraud.
