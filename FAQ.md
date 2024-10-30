Here’s a **FAQ/Glossary** to clarify terms and address common questions for LROL users:

---

### **Glossary**

1. **LROL (Loci Risk Orchestration Language)**: A JSON-based language for defining fraud detection and risk management rules, designed to standardize rule structure and enable cross-team collaboration.

2. **Model ID**: A unique identifier for each rule, aiding in tracking, auditing, and distinguishing between rules.

3. **Threshold**: A numeric confidence level that determines when a rule should trigger. Typically ranges from 0 to 1, with higher values indicating stricter conditions.

4. **Evaluation**: A condition or set of conditions within a rule. Evaluations can include comparison checks, aggregations, or logical conditions that assess transaction attributes or behavior.

5. **Action**: The response triggered when a rule meets the specified threshold. Common actions include flagging a transaction, blocking it, or notifying the appropriate team.

6. **Comparison Evaluation**: Evaluates whether a transaction attribute (e.g., amount, location) meets a defined condition, like greater than or within a specific range.

7. **Aggregation Evaluation**: Evaluates cumulative values over a set timeframe, such as summing transactions within the past hour to detect high transaction volumes.

8. **Conditional Case**: A branching evaluation that allows for “if-else” logic, triggering different actions based on defined conditions.

9. **Logical Condition**: Combines multiple evaluations using logical operators like AND/OR, enabling more complex rule conditions.

10. **Flag Transaction**: An action type that marks a transaction for review without blocking it.

---

### **Frequently Asked Questions (FAQ)**

1. **What is LROL used for?**  
   LROL is used to define rules for fraud detection and risk management across platforms, allowing organizations to standardize and centralize their rule definitions.

2. **How does LROL differ from a rule engine?**  
   LROL is a rule definition language, not a processing engine. It provides the structure for rules but requires an execution platform (e.g., Apache Flink, Spark, Loci) to evaluate and enforce them.

3. **Can LROL handle complex fraud scenarios?**  
   Yes, LROL supports multi-layered evaluations, conditional cases, and logical conditions, making it adaptable for complex fraud scenarios and evolving risks.

4. **What platforms can LROL work with?**  
   LROL is platform-agnostic, meaning it can theoretically work with various rule-processing systems, including streaming platforms like Apache Flink, Spark, Kafka, and custom in-house systems. However, LROL itself is a rule definition language, not an execution engine.
   
   To function across these platforms, adapters or connectors are needed to interpret and transform LROL's JSON-based rule structure into formats understood by the target system. These adapters bridge LROL with specific processing engines, ensuring smooth integration and functionality without implying that LROL directly handles execution.

5. **How is “threshold” used in LROL rules?**  
   The threshold sets a confidence level for triggering actions within a rule. If the evaluations meet or exceed this level, the defined action is triggered.

6. **Does LROL support machine learning integration?**  
   While LROL itself is focused on rule definitions, it can incorporate ML-derived metrics (like anomaly scores) as inputs, allowing organizations to enhance rules with machine learning insights.

7. **Can I customize actions in LROL?**  
   Yes, LROL allows custom action definitions, including flagging, blocking, or notifying based on rule outcomes. Users can tailor actions to meet organizational workflows.

