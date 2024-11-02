# Directory structure
ROOT_DIR := $(shell pwd)
CRATES_DIR := $(ROOT_DIR)/Crates
PARSER_DIR := $(CRATES_DIR)/lrol_parser
CLI_DIR := $(CRATES_DIR)/lrol_cli
BIN_DIR := $(ROOT_DIR)/bin
TARGET_DIR := $(ROOT_DIR)/target

# Binary name
BINARY_NAME := lrol

# Build type
BUILD_TYPE ?= release

# Cargo commands
CARGO := cargo
CARGO_BUILD_FLAGS := $(if $(findstring release,$(BUILD_TYPE)),--release,)

# Colors for output
CYAN := \033[36m
GREEN := \033[32m
RESET := \033[0m

.PHONY: all clean build test check fmt clippy install uninstall

# Default target
all: build

# Create necessary directories
$(BIN_DIR):
	@mkdir -p $(BIN_DIR)

# Build the project
build: $(BIN_DIR)
	@echo "$(CYAN)Building LROL parser library...$(RESET)"
	@cd $(PARSER_DIR) && $(CARGO) build $(CARGO_BUILD_FLAGS)
	
	@echo "$(CYAN)Building LROL CLI...$(RESET)"
	@cd $(CLI_DIR) && $(CARGO) build $(CARGO_BUILD_FLAGS)
	
	@echo "$(CYAN)Installing binary to bin directory...$(RESET)"
	@cp $(TARGET_DIR)/$(BUILD_TYPE)/lrol_cli $(BIN_DIR)/$(BINARY_NAME)
	@chmod +x $(BIN_DIR)/$(BINARY_NAME)
	@echo "$(GREEN)Build complete! Binary installed at $(BIN_DIR)/$(BINARY_NAME)$(RESET)"

# Run tests
test:
	@echo "$(CYAN)Testing LROL parser library...$(RESET)"
	@cd $(PARSER_DIR) && $(CARGO) test
	
	@echo "$(CYAN)Testing LROL CLI...$(RESET)"
	@cd $(CLI_DIR) && $(CARGO) test
	@echo "$(GREEN)All tests completed!$(RESET)"

# Check code formatting
fmt:
	@echo "$(CYAN)Checking code formatting...$(RESET)"
	@cd $(PARSER_DIR) && $(CARGO) fmt --all -- --check
	@cd $(CLI_DIR) && $(CARGO) fmt --all -- --check
	@echo "$(GREEN)Format check complete!$(RESET)"

# Run clippy lints
clippy:
	@echo "$(CYAN)Running clippy on parser...$(RESET)"
	@cd $(PARSER_DIR) && $(CARGO) clippy -- -D warnings
	
	@echo "$(CYAN)Running clippy on CLI...$(RESET)"
	@cd $(CLI_DIR) && $(CARGO) clippy -- -D warnings
	@echo "$(GREEN)Clippy check complete!$(RESET)"

# Run all checks
check: fmt clippy test
	@echo "$(GREEN)All checks completed successfully!$(RESET)"

# Clean the project
clean:
	@echo "$(CYAN)Cleaning project...$(RESET)"
	@cd $(PARSER_DIR) && $(CARGO) clean
	@cd $(CLI_DIR) && $(CARGO) clean
	@rm -rf $(BIN_DIR)
	@echo "$(GREEN)Clean complete!$(RESET)"

# Install the binary to the system (requires sudo)
install: build
	@echo "$(CYAN)Installing LROL binary to system...$(RESET)"
	@sudo cp $(BIN_DIR)/$(BINARY_NAME) /usr/local/bin/
	@echo "$(GREEN)Installation complete! LROL is now available system-wide.$(RESET)"

# Uninstall the binary from the system (requires sudo)
uninstall:
	@echo "$(CYAN)Uninstalling LROL binary from system...$(RESET)"
	@sudo rm -f /usr/local/bin/$(BINARY_NAME)
	@echo "$(GREEN)Uninstallation complete!$(RESET)"

# Development mode build (faster compilation)
dev: BUILD_TYPE=debug
dev: build

# Help target
help:
	@echo "LROL Build System"
	@echo "----------------"
	@echo "Available targets:"
	@echo "  all (default) - Build the project"
	@echo "  build        - Build the project"
	@echo "  dev          - Build in debug mode (faster compilation)"
	@echo "  test         - Run all tests"
	@echo "  check        - Run all checks (fmt, clippy, test)"
	@echo "  fmt          - Check code formatting"
	@echo "  clippy       - Run clippy lints"
	@echo "  clean        - Clean build artifacts"
	@echo "  install      - Install binary to system (requires sudo)"
	@echo "  uninstall    - Remove binary from system (requires sudo)"
	@echo ""
	@echo "Build types:"
	@echo "  BUILD_TYPE=release (default) - Optimized build"
	@echo "  BUILD_TYPE=debug            - Debug build"