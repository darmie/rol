# LROL CLI Tool

The LROL (Loci Risk Orchestration Language) CLI Tool provides utilities for validating, analyzing, and parsing LROL rules. It helps teams ensure their risk orchestration rules are well-formed and follow best practices.

## Prerequisites

## Install Rust
First, you'll need to install Rust and Cargo (Rust's package manager). 

For Unix-based systems (Linux/macOS):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For Windows:
- Download and run [rustup-init.exe](https://rustup.rs/)
- Follow the installation prompts

Verify your installation:
```bash
rustc --version
cargo --version
```

## Installation

```bash

# Build from source
git clone https://github.com/runloci/rol
cd rol
make build && make install
```

## Commands

The CLI tool provides three main commands:
- `validate`: Check LROL rule syntax and structure
- `analyze`: Perform deep analysis of rule complexity and relationships
- `parse`: Parse and display LROL rule contents

### Validate Command

Use the validate command to check if your LROL rules are syntactically correct and follow the schema requirements.

```bash
# Basic validation
lrol validate -f rules/my-rule.json

# Validation with detailed output
lrol validate -f rules/my-rule.json -v
```

#### Validation Output Example
```
Validating LROL file...
✓ File is valid LROL

Model Summary:
  Model ID: RISK-001
  Name: High Value Transaction Rule
  Description: Detects suspicious high-value transactions
  Threshold: 0.9
  Evaluations: 3
  Actions: 1

Metadata:
  Created by: john.doe
  Created at: 2024-01-01T10:00:00Z
  Last updated: 2024-01-01T15:30:00Z
```

### Analyze Command

The analyze command provides deeper insights into your LROL rules, including complexity analysis, dependency checking, and best practice recommendations.

```bash
# Basic analysis
lrol analyze -f rules/my-rule.json

# Detailed analysis
lrol analyze -f rules/my-rule.json -v

# JSON output
lrol analyze -f rules/my-rule.json -o json
```

#### Analysis Output Example
```
Analyzing LROL file...

Analysis Summary:
  Total Evaluations: 5
  Evaluation Types:
    comparison: 3
    logical: 2
  Complexity Score: 4.5

Warnings:
  MEDIUM [Complexity] High number of evaluations may impact maintainability
  LOW [Performance] Deep dependency chain detected

Suggestions:
  • Consider adding a description to improve rule documentation
  • Consider normalizing evaluation weights to improve rule balance

Detailed Analysis:
  Dependency Graph:
    high_risk_check → amount_check, frequency_check
    final_decision → high_risk_check, location_check

  DateTime Expressions:
    • datetime(now, '-2 hours')
    • datetime(now, '-24 hours')
```

### Parse Command

The parse command displays the contents of LROL rules in a readable format.

```bash
# Basic parsing
lrol parse -f rules/my-rule.json

# Parse with detailed output
lrol parse -f rules/my-rule.json -v

# JSON output
lrol parse -f rules/my-rule.json -o json
```

## Common Options

All commands support these common flags:
- `-f, --file`: Specify the input LROL file (required)
- `-v, --verbose`: Enable detailed output
- `-o, --output`: Specify output format (text/json) [analyze/parse only]

## Error Handling

When validation or analysis fails, the tool provides clear error messages:

```
✗ Validation failed

Parser Errors:
  Line 5, Column 10: Expected ',' but found '}'

Analyzer Errors:
  1. Invalid threshold 1.5: Value must be between 0 and 1
  2. Missing operand reference in eval1: non_existent
```

## Best Practices

1. Always validate rules before deploying:
   ```bash
   lrol validate -f rule.json
   ```

2. Use analysis for complex rules:
   ```bash
   lrol analyze -f rule.json -v
   ```

3. Export analysis results for documentation:
   ```bash
   lrol analyze -f rule.json -o json > analysis.json
   ```

4. Check dependencies and complexity:
   ```bash
   lrol analyze -f rule.json -v | grep "Dependency"
   ```

## Examples

### Validating Multiple Rules
```bash
# Validate all rules in a directory
for file in rules/*.json; do
    echo "Validating $file..."
    lrol validate -f "$file"
done
```

### Complex Analysis Workflow
```bash
# Validate and analyze if valid
if lrol validate -f rule.json; then
    echo "Rule is valid, performing analysis..."
    lrol analyze -f rule.json -v
else
    echo "Validation failed, fixing required"
fi
```

### Generating Documentation
```bash
# Create comprehensive rule documentation
lrol analyze -f rule.json -o json > analysis.json
lrol parse -f rule.json -o json > rule_details.json
```

## Error Codes

The CLI tool uses the following exit codes:
- `0`: Success
- `1`: Validation/Analysis failure
- `2`: File system error
- `3`: Invalid arguments

## Contributing

We welcome contributions! Please see our contribution guidelines for more details.