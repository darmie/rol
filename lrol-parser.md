# LROL Parser

A Rust parser for the Loci Risk Orchestration Language (LROL), built using the nom parsing framework. This parser handles JSON-based risk evaluation rules with support for comparison, logical, and aggregation operations.

## Prerequisites

### 1. Install Rust
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

### 2. IDE Setup
We recommend using VS Code with the following extensions:

1. [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
   - Provides advanced Rust language support
   - Code completion, go to definition, and real-time error checking

2. [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
   - Debugging support for Rust
   - Breakpoint setting and variable inspection

3. [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
   - TOML file support for Cargo.toml

Alternative IDEs:
- CLion with Rust plugin
- IntelliJ IDEA with Rust plugin
- Vim/Neovim with rust.vim and rust-analyzer

### 3. Project Setup
The root repository directory is a Cargo workspace that is capable of having multiple sub-crates. 
The `lrol_parser` crate can be found in the `Crates` directory. 

## Running Tests

### Basic Test Running
Run all tests:
```bash
cargo test
```

Run specific test:
```bash
cargo test parser::tests::test_parse_logical_evaluation
```

Run tests with output:
```bash
cargo test -- --nocapture
```

### Test Organization

Tests are categorized into:
- Basic parsing tests
- Logical operation tests
- Error handling tests

### VS Code Test Integration

1. Open the Command Palette (Ctrl+Shift+P / Cmd+Shift+P)
2. Type "Rust: Run Test" to run individual tests
3. Use the Testing sidebar to see all available tests

## Development

### Code Structure

- `Crates/lrol_parser/src/parser.rs`: Main parser implementation
- `Crates/lrol_parser/src/types.rs`: Type definitions
- `Crates/lrol_parser/src/error.rs`: Error handling

### Running Clippy (Rust Linter)
```bash
cargo clippy
```

### Formatting Code
```bash
cargo fmt
```

## Common Issues

1. **Rust Installation Issues**
   - For Windows: Ensure you have Microsoft Visual Studio C++ Build Tools
   - For Linux: Install build-essential package
   ```bash
   sudo apt install build-essential
   ```

2. **VS Code rust-analyzer Not Working**
   - Try reloading VS Code
   - Check if rust-analyzer is properly installed
   - Verify your Rust toolchain is up to date:
   ```bash
   rustup update
   ```

3. **Test Discovery Issues**
   - Clear target directory: `cargo clean`
   - Rebuild project: `cargo build`
   - Check file permissions

## Contributing

1. Ensure all tests pass: `cargo test`
2. Run clippy: `cargo clippy`
3. Format code: `cargo fmt`
4. Submit PR with detailed description

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [nom Documentation](https://docs.rs/nom/latest/nom/)
- [rust-analyzer Manual](https://rust-analyzer.github.io/manual.html)
- [VS Code Rust](https://code.visualstudio.com/docs/languages/rust)