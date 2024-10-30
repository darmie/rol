### 2. **Understanding LROL Rule Structure**

**Overview of Rule Components**  
LROL’s structure revolves around defining clear, structured rules using JSON. Each rule includes components like **evaluations**, **thresholds**, and **actions** that allow teams to specify conditions and responses for fraud detection scenarios.

**Key Components of an LROL Rule**:
- **Model ID**: A unique identifier for each rule, used for traceability.
- **Name and Description**: Human-readable labels to provide clarity on rule intent.
- **Threshold**: A numerical limit determining when the rule is triggered, commonly a confidence level or score.
- **Evaluations**: These are the core conditions that make up the rule, such as comparing transaction amounts, aggregating values over time, or checking for specific patterns. Evaluations may include:
  - **Comparison Evaluations**: Basic conditions like greater than, less than, or equal to.
  - **Aggregation Evaluations**: Used to evaluate cumulative transaction data, such as the sum or count of transactions within a timeframe.
  - **Logical Conditions**: Allow nesting of multiple evaluations using logical operators (AND/OR) to create complex conditions.
- **Actions**: Defines what happens when the rule is triggered, such as flagging a transaction, notifying a team, or blocking an action.

**JSON Schema and Syntax**  
The schema is designed to be **human-readable** and **consistent across platforms**. JSON’s structure ensures that rules are easy to read, maintain, and deploy. Fields within each rule are structured to ensure clarity and reusability, so they can be extended as fraud detection needs evolve.

---
**Sub Components**  
### **Model ID**
- **Purpose**: The `model_id` is a unique identifier for each rule, allowing teams to easily reference, trace, and audit individual rules within their systems. This ID becomes especially useful in platforms handling numerous fraud rules, as it helps categorize and manage rule versions.
- **Best Practices**: Use a standardized naming convention, such as `FRAUD-HIGHVAL-001`, to quickly recognize the rule’s purpose and type.

### **Name and Description**
- **Purpose**: These fields provide a human-readable label (`name`) and an optional explanatory text (`description`) to clarify the rule’s intent. A clear description is essential for documentation and for teams to understand each rule’s application in compliance processes.
- **Best Practices**: Write concise, informative names and descriptions. For example, `"High-Value Cross-Border Payment Detection"` with a description like `"Flags transactions over $10,000 from high-risk regions"` clearly conveys the rule’s function.

### **Threshold**
- **Purpose**: The `threshold` is a numeric score that defines the sensitivity or confidence level required to trigger the rule. It’s typically a value between 0 and 1 (e.g., `0.85`), representing a confidence percentage that must be met or exceeded for the rule to activate.
- **Usage**: In rules involving multiple evaluations, the threshold ensures that only transactions meeting a cumulative risk level are flagged. Setting the right threshold minimizes false positives and ensures relevant transactions are flagged.

### **Evaluations**
Evaluations define the specific conditions or criteria for identifying potential fraud or risk. These can be single checks or layered with multiple types:

   - **Comparison Evaluations**
     - **Description**: Basic evaluations that compare field values (e.g., transaction amount) against defined conditions (e.g., `>` or `<=`).
     - **Example**: `"transaction_amount": ">", 10000` would flag transactions exceeding $10,000.
     - **Common Fields**: `transaction_amount`, `origin_country`, `account_age`.

   - **Aggregation Evaluations**
     - **Description**: Evaluations that consider cumulative data, like sums, averages, or counts within a specific timeframe. Aggregation is ideal for rules checking transaction velocity or monitoring cumulative transaction values over time.
     - **Example**: Summing transaction amounts over a 24-hour period to detect unusually high activity.
     - **Best Practices**: Specify time windows and conditions clearly to avoid excessive data processing, especially in real-time systems.

   - **Logical Conditions**
     - **Description**: Logical conditions, using operators like AND/OR, combine multiple evaluations within a rule. These conditions allow for complex criteria where all or some of the evaluations must be met.
     - **Example**: A rule that flags transactions over $5,000 (Comparison) AND originates from a high-risk country (Comparison).
     - **Usage**: Logical conditions are useful for creating multi-layered rules, ensuring that only transactions meeting multiple criteria are flagged.

### **Actions**
Actions define what happens once the rule is triggered. The primary action types in LROL include:

   - **Flag Transaction**: Marks the transaction for further review, typically by compliance or risk teams.
   - **Block Transaction**: Prevents the transaction from processing, often used for high-risk scenarios where immediate prevention is required.
   - **Notify**: Sends a real-time alert to designated personnel or systems, useful for time-sensitive cases.
   
   **Example of Actions**:
   ```json
   {
     "type": "flag_transaction",
     "reason": "High-value transaction detected from high-risk country."
   }
   ```

Each action type provides a structured response to potential risks, aligning with operational protocols and regulatory requirements. By customizing actions per rule, compliance teams can ensure appropriate responses based on the severity and type of risk detected.

---

#### **Optional Fields**

1. **description**:
   - **Description**: Provides additional context about the rule’s purpose and application, helpful for documentation and team understanding.
   - **Example**: `"description": "Detects high-value transactions originating from high-risk countries"`.

2. **conditions**:
   - **Description**: Additional criteria or filters applied to evaluations, refining when evaluations are applied. These conditions allow for flexibility in how, when, and where evaluations are performed within a rule.
   - **Example**:
     ```json
     "conditions": [
       {
         "type": "comparison",
         "left": "origin_country",
         "operator": "IN",
         "right": ["Country_X", "Country_Y"]
       }
     ]
     ```

3. **weight**:
   - **Description**: An optional scoring value assigned to each evaluation, enabling prioritization of certain evaluations within a rule. Higher weights increase an evaluation's influence on the rule outcome, useful when combining multiple checks within a rule.
   - **Example**: `"weight": 3` assigns moderate importance to an evaluation within the overall rule.

4. **aggregation**:
   - **Description**: Used specifically in aggregation evaluations, this field defines the type of aggregation performed (e.g., SUM, COUNT, AVERAGE).
   - **Example**: `"aggregation": "SUM"` specifies summing transaction amounts.

---
