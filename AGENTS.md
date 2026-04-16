# Agent Guidelines for dots-cli

This document provides guidance for agents working in this repository.

## Project Overview

`dots` is a Rust CLI application that helps manage dotfiles, their dependencies, and allows downloading and switching between dotfiles.

## Build/Lint/Test Commands

### Building the Project

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the application
cargo run -- [args]
```

### Running Tests

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test <test_name>

# Run tests with output displayed
cargo test -- --nocapture <test_name>

# Run tests and show stdout
cargo test -- --show-output
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting (without modifying)
cargo fmt -- --check

# Run clippy for linting
cargo clippy

# Run clippy with all warnings
cargo clippy -- -D warnings
```

### Other Commands

```bash
# Check code without building
cargo check

# Build documentation
cargo doc

# Clean build artifacts
cargo clean
```

## Code Style Guidelines

### Formatting

- Use 4 spaces for indentation
- Follow `rustfmt` defaults (run `cargo fmt` before committing)
- Maximum line length: 100 characters (default)
- Use `?` operator for error propagation instead of `match` where appropriate

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Functions | snake_case | `get_config`, `process_file` |
| Variables | snake_case | `user_name`, `file_path` |
| Structs | PascalCase | `ConfigManager`, `FileHandler` |
| Enums | PascalCase | `FileStatus`, `ErrorType` |
| Enum variants | PascalCase | `Ok`, `Err`, `NotFound` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_RETRY_COUNT` |
| Modules | snake_case | `config`, `file_handler` |

### Imports

```rust
// Order imports by:
// 1. Standard library
// 2. External crates
// 3. Local modules (use `crate::` for internal)
use std::path::PathBuf;
use clap::Parser;
use crate::cli::Cli;
```

### Error Handling

- Use `Result<T, E>` for functions that can fail
- Use `anyhow::Result<T>` for application code ( propagating errors)
- Use `thiserror` for library code (defining error types)
- Use `Option<T>` when a value may be absent
- Prefer `unwrap_or`, `unwrap_or_else`, or `?` over `unwrap()` in production code
- Never use `unwrap()` or `expect()` in library code or production paths

### Documentation

- Document public API with `///` doc comments
- Include examples in doc comments when useful
- Add `#[allow(missing_docs)]` only when intentionally omitting docs

```rust
/// Retrieves the configuration from the given path.
///
/// # Arguments
/// * `path` - The path to the configuration file
///
/// # Example
/// ```
/// let config = get_config("/path/to/config.toml")?;
/// ```
pub fn get_config(path: &str) -> Result<Config, Error> {
    // ...
}
```

### Modules and Structure

- One module per file is preferred; group related code in a `mod.rs`
- Keep modules focused (single responsibility)
- Use `pub(crate)` for items intended for internal use but public within crate

### Performance Considerations

- Clone only when necessary; prefer references
- Use `&str` over `String` for function parameters when possible
- Use `Cargo.lock` to pin dependency versions (do not edit directly)

### Testing

- Write unit tests in the same file, below the implementation
- Use `#[cfg(test)]` module for integration tests
- Name test functions descriptively: `test_<what_is_being_tested>_<expected_behavior>`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_with_valid_file() {
        // test implementation
    }
}
```

### Common Patterns

```rust
// Option handling
let value = optional_value.unwrap_or(default_value);

// Result handling with ?
fn process() -> Result<Output, Error> {
    let config = read_config()?;  // propagates Error
    Ok(transform(config))
}

// Early returns
fn find_item(items: &[Item], id: u32) -> Option<Item> {
    for item in items {
        if item.id == id {
            return Some(item.clone());
        }
    }
    None
}
```

## Dependencies

- **clap**: CLI argument parsing with derive macros
- Add dependencies to `Cargo.toml` under `[dependencies]`
- Pin exact versions for reproducible builds

## Git Workflow

- Commit messages should be descriptive but concise
- Run `cargo fmt` and `cargo clippy` before committing
- Ensure `cargo test` passes before submitting changes
