# AGENTS.md - DG-LAB Rust Project Guide for Agentic Coding Assistants

## Project Overview

This is a Rust workspace project for DG-LAB device control with BLE and WiFi support. The project is organized as a workspace with multiple crates for protocol implementation, core business logic, CLI, and GUI.

**Language**: Rust 2021 Edition  
**Workspace Structure**:
- `dglab-protocol` - BLE/WiFi communication protocol library
- `dglab-core` - Core business logic (device abstraction, session management, waveforms)
- `dglab-cli` - Command-line interface with TUI support
- `dglab-gui` - GUI (currently disabled, being replaced with Tauri + React)

---

## Build, Test, and Lint Commands

### Build Commands
```bash
# Build entire workspace
cargo build

# Build specific crate
cargo build -p dglab-protocol
cargo build -p dglab-core
cargo build -p dglab-cli

# Build with release optimization
cargo build --release

# Build CLI binary
cargo build --bin dglab
```

### Test Commands
```bash
# Run all tests in workspace
cargo test

# Run tests for specific crate
cargo test -p dglab-protocol
cargo test -p dglab-core
cargo test -p dglab-cli

# Run a single test by name
cargo test test_name

# Run tests with test name filter
cargo test session_manager

# Run tests in specific file (by module path)
cargo test --lib session::tests

# Run doc tests only
cargo test --doc

# Run tests with output
cargo test -- --nocapture

# Run tests with specific features
cargo test --features "feature_name"
```

### Lint and Format Commands
```bash
# Check code with clippy (linter)
cargo clippy

# Check specific crate
cargo clippy -p dglab-core

# Fix clippy warnings automatically
cargo clippy --fix

# Format code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check

# Check code without building
cargo check

# Check all targets
cargo check --all-targets
```

### Documentation Commands
```bash
# Build documentation
cargo doc

# Build and open documentation
cargo doc --open

# Build documentation for specific crate
cargo doc -p dglab-protocol --open
```

### Run Commands
```bash
# Run CLI (scan devices)
cargo run --bin dglab -- scan

# Run CLI with debug logging
cargo run --bin dglab -- --debug scan

# Run specific CLI commands
cargo run --bin dglab -- connect
cargo run --bin dglab -- control --power 50
cargo run --bin dglab -- tui
```

---

## Code Style Guidelines

### Module Organization and Imports

**Import Order**: Group imports in the following order with blank lines between groups:
1. Standard library imports (`std::*`)
2. External crate imports (sorted alphabetically)
3. Internal crate imports (`crate::*`)
4. Parent/sibling module imports (`super::*`, `self::*`)

**Example**:
```rust
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info, warn};

use crate::device::{Device, DeviceEvent, DeviceState};
use crate::error::{CoreError, Result};
```

### Documentation

- **Module docs**: Every module (file) should have a module-level doc comment (`//!`)
- **Public items**: All public functions, structs, enums, and traits must have doc comments (`///`)
- Enable `#![warn(missing_docs)]` in library crates
- Use triple-slash doc comments with proper grammar and punctuation

**Example**:
```rust
//! 会话管理器

/// 会话管理器
pub struct SessionManager {
    /// 会话 ID
    session_id: String,
}

/// 创建新的会话管理器
pub fn new() -> Self {
    // ...
}
```

### Naming Conventions

- **Crates**: kebab-case (`dglab-protocol`, `dglab-core`)
- **Modules**: snake_case (`session_manager`, `device_traits`)
- **Types** (structs, enums, traits): PascalCase (`SessionManager`, `DeviceState`, `Device`)
- **Functions/methods**: snake_case (`add_device`, `session_info`)
- **Constants**: SCREAMING_SNAKE_CASE (`SERVICE_UUID`, `MAX_POWER`)
- **Type aliases**: PascalCase with descriptive suffix (`DeviceBox`, `DeviceMap`)

### Types and Error Handling

**Error Handling**:
- Use `thiserror` for custom error types
- Define crate-specific error enums (e.g., `ProtocolError`, `CoreError`)
- Use `Result<T>` type alias: `pub type Result<T> = std::result::Result<T, CoreError>;`
- Convert errors from dependencies using `#[from]` attribute

**Example**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    /// 协议错误
    #[error("Protocol error: {0}")]
    ProtocolError(#[from] dglab_protocol::error::ProtocolError),
    
    /// 设备未连接
    #[error("Device not connected")]
    DeviceNotConnected,
}

pub type Result<T> = std::result::Result<T, CoreError>;
```

**Async Traits**:
- Use `async-trait` crate for async trait methods
- Annotate with `#[async_trait]` macro

```rust
use async_trait::async_trait;

#[async_trait]
pub trait Device: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
}
```

### Formatting

- Use rustfmt defaults (4-space indentation)
- Maximum line length: 100 characters (default)
- Use trailing commas in multi-line expressions
- Place opening braces on the same line

### Serialization

- Use `serde` with derive macros for serialization
- Enable features in workspace dependencies: `serde = { version = "1.0", features = ["derive"] }`
- Derive `Serialize` and `Deserialize` for data structures that need to be persisted or transmitted

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
}
```

### Async and Concurrency

- Use `tokio` as the async runtime with `full` features
- Use `Arc<RwLock<T>>` for shared mutable state across async tasks
- Use `broadcast::channel` for event distribution
- Always specify `Send + Sync` bounds for trait objects used across threads

```rust
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

type DeviceBox = Box<dyn Device>;
type DeviceMap = HashMap<String, Arc<RwLock<DeviceBox>>>;
```

### Logging

- Use `tracing` crate for structured logging
- Import specific macros: `use tracing::{debug, info, warn, error};`
- Log levels: `debug!`, `info!`, `warn!`, `error!`
- Initialize with `tracing-subscriber` in binary crates

---

## Workspace Configuration

### Dependency Management

- All dependencies should be declared in workspace `Cargo.toml` under `[workspace.dependencies]`
- Crates reference workspace dependencies: `tokio.workspace = true`
- Shared metadata (version, authors, license) should use workspace inheritance

### Lints

The workspace enables these lints:
```toml
[workspace.lints.rust]
unused_crate_dependencies = "warn"
unused_qualifications = "warn"
unused_results = "warn"
```

---

## Git Workflow

Ignored files (see `.gitignore`):
- `/target/`, `Cargo.lock` (for libraries)
- IDE files (`.idea/`, `.vscode/`)
- Build artifacts (`*.so`, `*.dll`, `*.exe`)
- Environment files (`*.env`)
- Logs (`*.log`, `logs/`)

---

## Notes for Agents

1. **Always run tests**: After making changes, run `cargo test -p <crate>` to ensure nothing breaks
2. **Check formatting**: Run `cargo fmt` before committing
3. **Run clippy**: Use `cargo clippy` to catch common mistakes and suggest improvements
4. **Documentation**: Update doc comments when adding/modifying public APIs
5. **Error handling**: Never use `unwrap()` or `expect()` in library code; propagate errors with `?`
6. **Chinese comments**: The project uses Chinese for documentation and comments, maintain consistency
7. **Module exports**: Use `pub use` in `mod.rs` files to re-export commonly used types for convenience

