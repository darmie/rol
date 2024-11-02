__High-Value Transactions with Conditional Risk__
By combining amount thresholds with a tiered risk assessment and entity-specific thresholds, the rule is robust and adaptable to varying transaction sizes and risk profiles.

---

### **R001: High-Value Transactions**
- **Purpose**: Flags high-value transactions and monitors for rapid withdrawals following large deposits, which could indicate layering or fraud.
- **Key Evaluations**:
  - Checks if the transaction amount is above a specified high-value threshold.
  - Monitors recent large deposits and calculates their cumulative value.
  - Evaluates standard deviation in transaction amounts to detect unusual variations.
- **Logic**: Uses `AND` to require all conditions to be met before flagging, ensuring precision.
- **Summary**: This rule identifies significant transactions that deviate from typical patterns or follow large deposits, reducing the risk of undetected large-value laundering.

---

### **R002: Cross-Border High-Risk Transactions**
- **Purpose**: Flags cross-border transactions involving politically exposed persons (PEPs) or jurisdictions with elevated risk levels.
- **Key Evaluations**:
  - Checks if the transaction is cross-border by comparing the geo-location with the entity’s home country.
  - Identifies transactions involving PEPs, who typically require higher scrutiny.
  - Cross-references the jurisdiction against a high-risk country list.
  - Detects new beneficiaries not previously associated with the entity.
- **Logic**: Uses `OR` to trigger if any high-risk condition is met, ensuring comprehensive coverage.
- **Summary**: This rule helps identify cross-border transactions with potentially high-risk elements like PEP involvement or high-risk jurisdictions.

---

### **R003: Velocity Increase and Volume Surge**
- **Purpose**: Detects a sudden surge in transaction volume or frequency, especially with smaller, frequent transactions, which may indicate structuring.
- **Key Evaluations**:
  - Counts the number of transactions in a short period to identify volume surges.
  - Aggregates small-value transactions to monitor structuring.
- **Logic**: Uses `OR` to flag based on either high frequency or numerous low-value transactions.
- **Summary**: This rule targets structuring behavior by monitoring both the frequency and volume of small transactions, which are common in evasion tactics.

---

### **R004: New Account with High-Risk Activity**
- **Purpose**: Flags transactions from newly created or recently reactivated accounts with high transaction volumes.
- **Key Evaluations**:
  - Checks if the account is new or recently reactivated.
  - Monitors the total transaction volume within the last day.
  - Compares the volume against entity-specific thresholds for high-value activity.
- **Logic**: Uses `AND` to confirm new account status and high transaction volume before flagging.
- **Summary**: This rule monitors new accounts for high-value transactions, a common pattern in fraud and money laundering.

---

### **R005: Entity-Specific Profile Limits and Thresholds**
- **Purpose**: Flags transactions exceeding entity-specific thresholds for daily volume or amount.
- **Key Evaluations**:
  - Compares the transaction amount to the entity-defined threshold.
  - Aggregates daily transaction volume to detect if it surpasses the profile’s limit.
- **Logic**: Uses `OR` for flexibility, flagging transactions that exceed either the amount or volume thresholds.
- **Summary**: This rule tailors monitoring to individual entities, ensuring alerts when activity surpasses their specific thresholds.

---

### **R006: Unusual Transaction Currency and Amount Patterns**
- **Purpose**: Detects transactions in uncommon currencies or unusual amounts that deviate significantly from recent activity.
- **Key Evaluations**:
  - Checks if the currency is not commonly used by the entity.
  - Calculates the standard deviation of recent transactions and flags large deviations.
- **Logic**: Uses `OR` to flag either unusual currency usage or deviations in amount.
- **Summary**: This rule helps detect unexpected transactions by monitoring for atypical currencies or amounts, improving sensitivity to potential risks.

---

### **R007: High Transaction Volume in High-Risk Areas**
- **Purpose**: Flags high transaction volumes occurring in high-risk jurisdictions.
- **Key Evaluations**:
  - Cross-references the transaction’s geo-location against a list of high-risk countries.
  - Counts the transactions in high-risk areas within the monitoring period.
- **Logic**: Uses `AND` to confirm both high-risk jurisdiction and volume thresholds are met.
- **Summary**: This rule targets suspicious patterns in high-risk areas, combining volume and location checks for robust detection.

---

### **R008: Rapid Withdrawals Following Large Deposits**
- **Purpose**: Detects large withdrawals occurring shortly after large deposits, a common pattern in layering for money laundering.
- **Key Evaluations**:
  - Aggregates recent deposit values as a reference.
  - Monitors withdrawal timing to identify rapid transfers post-deposit.
  - Compares withdrawal amounts against recent deposits to flag large transfers.
- **Logic**: Uses `AND` to require both large withdrawal and timing conditions, ensuring targeted detection.
- **Summary**: This rule identifies rapid fund movement following deposits, which may indicate layering.

---

### **R009: Multi-Currency and Rapid Transactions**
- **Purpose**: Flags rapid transactions involving multiple currencies within a short period, possibly for currency layering.
- **Key Evaluations**:
  - Counts distinct currencies used in recent transactions.
  - Monitors rapid transaction frequency.
- **Logic**: Uses `AND` to combine multi-currency and frequency checks.
- **Summary**: This rule monitors unusual currency use and rapid transactions, a behavior often linked to layering schemes.

---

### **R010: Suspicious IP or Geo-Location Changes**
- **Purpose**: Flags transactions from unusual IPs or geo-locations, especially outside business hours.
- **Key Evaluations**:
  - Checks if the IP or location differs from the usual.
  - Monitors transaction timing for unusual hours.
- **Logic**: Uses `OR` to detect any suspicious factor like location, IP, or timing.
- **Summary**: This rule enhances detection for potentially unauthorized transactions occurring from unexpected locations or times.

---

### **R011: Unverified or High-Risk Beneficiary**
- **Purpose**: Flags transactions to unverified or high-risk beneficiaries.
- **Key Evaluations**:
  - Checks if the beneficiary is unverified in the entity’s profile.
  - Cross-references against high-risk beneficiaries.
- **Logic**: Uses `OR` to trigger for unverified or high-risk beneficiaries.
- **Summary**: This rule improves detection by monitoring for risky or unknown recipients in transactions.

---

### **R012: High Transaction Volume for New or Recently Inactive Accounts**
- **Purpose**: Monitors new or recently reactivated accounts for high transaction volumes.
- **Key Evaluations**:
  - Checks if the account is new or was recently inactive.
  - Aggregates transaction volume for the last day.
- **Logic**: Uses `AND` to confirm both high volume and new or inactive account status.
- **Summary**: This rule helps detect sudden high-volume activity in new or reactivated accounts.

---

### **R013: Known Fraud and Structuring Patterns**
- **Purpose**: Detects structuring patterns, including split transactions and frequent low-value payments.
- **Key Evaluations**:
  - Checks for transactions just below reporting thresholds.
  - Aggregates frequent small payments.
- **Logic**: Uses `AND` to confirm structuring patterns through both thresholds and frequency.
- **Summary**: This rule targets common fraud patterns by identifying multiple small or split transactions.

---

### **R014: Beneficiary Country Outside Typical Profile**
- **Purpose**: Flags transactions to beneficiaries in countries outside the entity’s usual transaction profile.
- **Key Evaluations**:
  - Checks if the transaction is in a new or high-risk country.
- **Logic**: Uses `OR` to detect either unusual or high-risk beneficiary countries.
- **Summary**: This rule identifies transactions to unfamiliar or high-risk countries, which could indicate suspicious international activity.

---

### **R015: High Transaction Count or Value in Specific Industries**
- **Purpose**: Flags transactions with high counts or cumulative values in high-risk industries.
- **Key Evaluations**:
  - Monitors transaction counts and cumulative value in high-risk sectors.
- **Logic**: Uses `OR` for flexibility if either count or value exceeds thresholds.
- **Summary**: This rule targets high-risk industries like gambling or crypto, detecting potential risks through volume and value.

---

### **R016: Frequent Cross-Border Transactions**
- **Purpose**: Flags frequent cross-border transactions within a set timeframe.
- **Key Evaluations**:
  - Confirms cross-border status.
  - Counts transactions to monitor for high frequency.
- **Logic**: Uses `AND` to combine both cross-border and frequency checks.
- **Summary**: This rule monitors for unusual patterns in international transactions, helping detect potential money laundering.

---

### **R017: Account with History of Failed Transactions**
- **Purpose**: Detects accounts with repeated failed transactions, potentially indicating fraud attempts.
- **Key Evaluations**:
  - Monitors transaction status and counts failures.
- **Logic**: Uses `AND` to confirm failure count exceeds a threshold.
- **Summary**: This rule detects accounts with multiple failures, which may signal testing or misuse.

---

### **R018: Large Transactions Outside Business Hours**
- **Purpose**: Flags large transactions outside business hours, potentially indicating unauthorized access.
- **Key Evaluations**:
  - Checks transaction amount.
  - Monitors timing for outside business hours.
- **Logic**: Uses `AND` to confirm both high amount and timing criteria.
- **Summary**: This rule targets unusual, high-value transactions at odd hours, enhancing detection of unauthorized activity.

---

### **R019: Rapid Successive Payments to Different Beneficiaries

**
- **Purpose**: Detects rapid payments to multiple beneficiaries within a short period, common in layering.
- **Key Evaluations**:
  - Checks payment type and frequency.
  - Monitors unique beneficiaries in recent transactions.
- **Logic**: Uses `AND` to confirm payment frequency and distinct beneficiaries.
- **Summary**: This rule identifies rapid, successive payments to different beneficiaries, indicating potential layering.

---

### **R020: Unusual Increase in Daily Transaction Volume**
- **Purpose**: Flags a sudden increase in daily transaction volume relative to recent averages.
- **Key Evaluations**:
  - Aggregates current day’s transaction volume.
  - Compares it to recent daily averages to detect unusual spikes.
- **Logic**: Uses `AND` to confirm volume surpasses recent average by a threshold.
- **Summary**: This rule detects unusual spikes in transaction volume, indicating potential suspicious account activity.

---
