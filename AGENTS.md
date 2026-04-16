# Agent Guidelines for dots-cli

This document provides guidance for agents working in this repository.

## Project Overview

`dots` is a Rust CLI application that helps manage dotfiles, their dependencies, and allows downloading and switching between dotfiles.

### Module Structure

```
src/
├── main.rs              # Entry point - parses CLI args and delegates to command .run()
├── cli/
│   ├── mod.rs        # Module declarations
│   ├── entrypoint.rs # CLI struct definitions (CliEntrypoint, CliEntrypointSubcommands)
│   ├── doctor.rs    # DoctorCommand struct with .run() method
│   └── version.rs  # VersionCommand struct with .run() method
└── doctor/
    ├── mod.rs      # Module declarations
    └── checks.rs  # Health checks (run_checks, HealthResult)
```

## Build/Lint/Test Commands

### Building the Project

```bash
cargo build                  # Build the project
cargo build --release      # Build in release mode
cargo run -- [args]      # Run the application
```

### Running Tests

```bash
cargo test                    # Run all tests
cargo test <test_name>         # Run a single test by name
cargo test -- --nocapture    # Run tests with output displayed
cargo test -- --show-output  # Run tests and show stdout
```

### Code Quality

```bash
cargo fmt                  # Format code (run before committing)
cargo fmt -- --check       # Check formatting without modifying
cargo clippy               # Run clippy for linting
cargo clippy -- -D warnings # Treat all warnings as errors
cargo check              # Check code without building
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
| Functions | snake_case | `get_config`, `run_checks` |
| Variables | snake_case | `user_name`, `file_path` |
| Structs | PascalCase | `CliEntrypoint`, `DoctorCommand` |
| Enums | PascalCase | `CliEntrypointSubcommands` |
| Enum variants | PascalCase | `Doctor`, `Version` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_RETRY_COUNT` |
| Modules | snake_case | `cli`, `doctor` |
| Command structs | <CommandName>Command | `DoctorCommand`, `VersionCommand` |

### Imports

Order imports by: 1) Standard library, 2) External crates, 3) Local modules (use `crate::`)

```rust
use std::path::PathBuf;
use clap::Parser;
use crate::cli::entrypoint::CliEntrypoint;
```

### Command Pattern (Important!)

All CLI commands should follow this pattern:

```rust
// In src/cli/<command>.rs
use clap::Args;

#[derive(Args, Debug)]
pub struct <CommandName>Command {
    // fields with clap attributes
}

impl <CommandName>Command {
    pub fn run(&self) {
        // command logic here
    }
}

// In main.rs
match args.command {
    CliEntrypointSubcommands::<CommandName>(cmd) => cmd.run(),
}
```

### Error Handling

- Use `Result<T, E>` for functions that can fail
- Use `anyhow::Result<T>` for application code (propagating errors)
- Use `thiserror` for library code (defining error types)
- Use `Option<T>` when a value may be absent
- Prefer `unwrap_or`, `unwrap_or_else`, or `?` over `unwrap()` in production code
- Never use `unwrap()` or `expect()` in library code or production paths

### Documentation

- Document public API with `///` doc comments
- Include examples in doc comments when useful

```rust
/// Runs system diagnostics and checks for required tools.
///
/// # Example
/// ```
/// let cmd = DoctorCommand { json: false, commands: None };
/// cmd.run();
/// ```
pub fn run(&self) {
    // ...
}
```

### Modules and Structure

- One module per file is preferred
- Keep modules focused (single responsibility)
- Use `pub(crate)` for items intended for internal use but public within crate
- Each command gets its own file in `src/cli/`

### Testing

- Write unit tests in the same file, below the implementation
- Use `#[cfg(test)]` module for integration tests
- Name test functions descriptively: `test_<what_is_being_tested>_<expected_behavior>`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_detects_git() {
        // test implementation
    }
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