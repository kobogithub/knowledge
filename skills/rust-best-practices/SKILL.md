---
name: rust-best-practices
description: Rust programming best practices for systems programming and CLI tools
version: 1.0.0
author: Knowledge Framework
tags: [rust, systems-programming, performance, safety]
---

# Rust Best Practices

Expert guidelines for writing idiomatic, safe, and performant Rust code.

## Core Principles

### 1. Ownership & Borrowing (Critical)

**Prefer Borrowing Over Cloning**
```rust
// ✅ GOOD: Borrow when possible
fn process_data(data: &Vec<String>) {
    for item in data {
        println!("{}", item);
    }
}

// ❌ BAD: Unnecessary clone
fn process_data(data: Vec<String>) {
    // Takes ownership, caller loses data
}
```

**Use Lifetimes Explicitly When Needed**
```rust
struct Parser<'a> {
    content: &'a str,
}

impl<'a> Parser<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }
}
```

### 2. Error Handling (Critical)

**Use Result and ? Operator**
```rust
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let contents = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    toml::from_str(&contents)
        .context("Failed to parse config")
}
```

**Custom Error Types**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}
```

### 3. Type Design (High)

**Use Newtype Pattern**
```rust
#[derive(Debug, Clone)]
pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
    
    pub fn value(&self) -> u64 {
        self.0
    }
}
```

**Builder Pattern**
```rust
#[derive(Default)]
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<Duration>,
}

impl ConfigBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn build(self) -> Result<Config> {
        Ok(Config {
            host: self.host.ok_or("host required")?,
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        })
    }
}
```

### 4. Collections & Iterators (High)

**Use Iterators Idiomatically**
```rust
// ✅ GOOD: Iterator chains
let result: Vec<_> = data
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.value)
    .collect();

// ❌ BAD: Manual loops
let mut result = Vec::new();
for item in &data {
    if item.is_valid() {
        result.push(item.value);
    }
}
```

**Preallocate Collections**
```rust
// ✅ GOOD: Known capacity
let mut vec = Vec::with_capacity(100);

// ⚠️ OK: Unknown capacity
let mut vec = Vec::new();
```

### 5. Concurrency (High)

**Use Channels for Communication**
```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello from thread").unwrap();
});

let msg = rx.recv().unwrap();
```

**Arc + Mutex for Shared State**
```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});
```

**Async/Await (Tokio)**
```rust
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let result = fetch_data().await?;
    process(result).await?;
    Ok(())
}

async fn fetch_data() -> Result<Data> {
    let response = reqwest::get("https://api.example.com")
        .await?
        .json()
        .await?;
    Ok(response)
}
```

### 6. CLI Development (High)

**Use Clap for Argument Parsing**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server
    Start {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Stop the server
    Stop,
}
```

### 7. Testing (High)

**Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_division_by_zero() {
        divide(10, 0);
    }
}
```

**Integration Tests**
```rust
// tests/integration_test.rs
use myapp::process;

#[test]
fn test_end_to_end() {
    let input = "test data";
    let result = process(input).unwrap();
    assert_eq!(result, "expected output");
}
```

### 8. Performance (Medium)

**Use &str Over String When Possible**
```rust
// ✅ GOOD: No allocation
fn process(s: &str) -> usize {
    s.len()
}

// ❌ BAD: Unnecessary allocation
fn process(s: String) -> usize {
    s.len()
}
```

**Avoid Unnecessary Allocations**
```rust
// ✅ GOOD: Reuse buffer
let mut buffer = String::with_capacity(1024);
for item in items {
    buffer.clear();
    write!(&mut buffer, "{}", item)?;
    send(&buffer);
}
```

**Use Cow for Conditional Ownership**
```rust
use std::borrow::Cow;

fn process<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains("bad") {
        Cow::Owned(input.replace("bad", "good"))
    } else {
        Cow::Borrowed(input)
    }
}
```

### 9. Cargo & Project Structure (High)

**Cargo.toml Best Practices**
```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"

[dependencies]
anyhow = "1.0"
clap = { version = "4.5", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

**Project Structure**
```
src/
  main.rs           # Binary entry point
  lib.rs            # Library entry point
  commands/         # Command modules
    mod.rs
    start.rs
    stop.rs
  models/           # Data structures
  error.rs          # Error types
  config.rs         # Configuration
tests/
  integration_test.rs
benches/
  benchmark.rs
```

### 10. Common Crates (Medium)

**Essential Crates**
- `anyhow` / `thiserror` - Error handling
- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `serde` - Serialization
- `reqwest` - HTTP client
- `tracing` - Logging
- `regex` - Regular expressions

## Common Pitfalls

❌ **Unnecessary `.clone()`**: Performance overhead  
✅ Use references and borrowing

❌ **Unwrapping everywhere**: Panics on errors  
✅ Use `?` operator and `Result`

❌ **String vs &str confusion**: Extra allocations  
✅ Prefer `&str` for function parameters

❌ **Fighting the borrow checker**: Overusing `Arc<Mutex<T>>`  
✅ Design with ownership in mind from the start

❌ **Ignoring Clippy warnings**: Missing optimization opportunities  
✅ Run `cargo clippy` and fix warnings

## Development Workflow

```bash
# Check code
cargo check

# Run with optimizations
cargo run --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Lint
cargo clippy

# Format
cargo fmt

# Check for outdated deps
cargo outdated

# Security audit
cargo audit

# Benchmark
cargo bench
```

## Performance Checklist

- [ ] Use `&str` instead of `String` for parameters
- [ ] Preallocate collections with `with_capacity()`
- [ ] Avoid unnecessary `.clone()` calls
- [ ] Use iterators instead of loops
- [ ] Profile with `cargo flamegraph`
- [ ] Run `cargo clippy` and fix warnings
- [ ] Enable LTO in release mode
- [ ] Use `#[inline]` for hot functions
- [ ] Avoid allocations in hot paths
- [ ] Benchmark critical sections

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Effective Rust](https://www.lurklurk.org/effective-rust/)
