---
name: rust-best-practices
description: Rust development best practices, error handling, CLI tools and library design
version: 1.0.0
tags:
  - best-practices
  - rust
  - cli
  - systems
  - cargo
---

# rust-best-practices

Mejores practicas para desarrollo en Rust: estructura de proyecto, error handling, CLI tools, testing y patrones idiomaticos.

## Overview

Rust es ideal para:
- **CLI tools**: Binarios rapidos, distribuibles sin runtime (clap, indicatif)
- **Libraries**: Crates reutilizables con APIs seguras y ergonomicas
- **Systems programming**: Alto rendimiento, seguridad de memoria
- **WebAssembly**: Compilacion a wasm para frontend y edge
- **Networking**: Servidores async con tokio, axum

## Project Structure

### Binario (CLI tool)

```
mycli/
├── src/
│   ├── main.rs            # Entry point, CLI parsing
│   ├── lib.rs             # Core logic (testeable sin main)
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   └── run.rs
│   ├── config.rs
│   ├── error.rs           # Error types centralizados
│   └── utils.rs
├── tests/
│   └── integration_test.rs
├── Cargo.toml
├── Cargo.lock             # SIEMPRE commitear para binarios
├── rust-toolchain.toml
└── .clippy.toml
```

### Library

```
mylib/
├── src/
│   ├── lib.rs             # Public API, re-exports
│   ├── types.rs
│   ├── parser/
│   │   ├── mod.rs
│   │   └── lexer.rs
│   └── error.rs
├── examples/
│   └── basic.rs
├── benches/
│   └── parsing.rs
├── Cargo.toml
└── Cargo.lock             # NO commitear para libraries
```

### Workspace (multi-crate)

```
workspace/
├── Cargo.toml             # [workspace] definition
├── Cargo.lock
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── cli/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── server/
│       ├── Cargo.toml
│       └── src/main.rs
└── tests/
    └── integration.rs
```

## Cargo.toml

### Configuracion recomendada

```toml
[package]
name = "mycli"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"          # MSRV explicito
description = "A fast CLI tool"
license = "MIT"
repository = "https://github.com/user/mycli"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"             # Para binarios
thiserror = "2"           # Para libraries
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
assert_cmd = "2"          # Testing CLI
predicates = "3"          # Assertions expresivas
tempfile = "3"
insta = "1"               # Snapshot testing
criterion = "0.5"         # Benchmarks

[profile.release]
lto = true                # Link-Time Optimization
strip = true              # Remover debug symbols
codegen-units = 1         # Mejor optimizacion (build mas lento)
panic = "abort"           # Binario mas pequeno

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
unwrap_used = "warn"
expect_used = "warn"
```

### Workspace config

```toml
# Root Cargo.toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.package]
edition = "2021"
rust-version = "1.75"
license = "MIT"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
thiserror = "2"

# En cada crate:
# [dependencies]
# serde = { workspace = true }
```

## rust-toolchain.toml

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy", "rust-analyzer"]
```

## Error Handling

### En binarios: usar `anyhow`

```rust
use anyhow::{Context, Result, bail, ensure};

fn main() -> Result<()> {
    let config = load_config()
        .context("Failed to load configuration")?;

    let data = std::fs::read_to_string(&config.input_path)
        .with_context(|| format!("Failed to read file: {}", config.input_path))?;

    ensure!(!data.is_empty(), "Input file is empty");

    if data.len() > 1_000_000 {
        bail!("Input file too large: {} bytes", data.len());
    }

    process(&data)?;
    Ok(())
}
```

### En libraries: usar `thiserror`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid syntax at line {line}: {message}")]
    Syntax { line: usize, message: String },

    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    #[error("File not found: {path}")]
    FileNotFound {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

// Funcion que retorna el error custom
pub fn parse(input: &str) -> Result<Document, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Syntax {
            line: 0,
            message: "empty input".to_string(),
        });
    }
    // ...
    Ok(document)
}
```

### Patron: convertir entre error types

```rust
// En el binario, anyhow envuelve cualquier error automaticamente
use anyhow::Result;
use mylib::ParseError;

fn run() -> Result<()> {
    let doc = mylib::parse(&input)?;  // ParseError -> anyhow::Error
    Ok(())
}

// En una lib, definir conversiones explicitas con From
impl From<ParseError> for AppError {
    fn from(err: ParseError) -> Self {
        AppError::Parse(err)
    }
}
```

## CLI con clap

### Derive API

```rust
use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser)]
#[command(name = "mycli", version, about = "A fast CLI tool")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Config file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init(InitArgs),

    /// Run the processor
    Run {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output format
        #[arg(short, long, default_value_t = Format::Json)]
        format: Format,
    },

    /// List all items
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,

        /// Maximum items to show
        #[arg(short, long, default_value_t = 50)]
        limit: usize,
    },
}

#[derive(Args)]
struct InitArgs {
    /// Project name
    name: String,

    /// Template to use
    #[arg(short, long, default_value = "default")]
    template: String,
}

#[derive(Clone, ValueEnum)]
enum Format {
    Json,
    Csv,
    Table,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => cmd_init(args),
        Commands::Run { input, format } => cmd_run(&input, format),
        Commands::List { status, limit } => cmd_list(status.as_deref(), limit),
    }
}
```

## Structured Logging con tracing

```rust
use tracing::{info, warn, error, debug, instrument, Level};
use tracing_subscriber::{fmt, EnvFilter};

fn setup_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

// Instrument automaticamente agrega span con parametros
#[instrument(skip(content), fields(path = %path.display()))]
fn process_file(path: &std::path::Path, content: &str) -> anyhow::Result<()> {
    info!(lines = content.lines().count(), "Processing file");

    if content.is_empty() {
        warn!("File is empty, skipping");
        return Ok(());
    }

    debug!(bytes = content.len(), "Parsing content");

    match parse(content) {
        Ok(result) => {
            info!(items = result.len(), "Parse complete");
            Ok(())
        }
        Err(e) => {
            error!(error = %e, "Parse failed");
            Err(e.into())
        }
    }
}
```

## Serde Patterns

### Serialization / Deserialization

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub project_name: String,
    pub version: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

fn default_port() -> u16 {
    8080
}

// Deserializar desde multiples formatos
impl Config {
    pub fn from_toml(input: &str) -> anyhow::Result<Self> {
        Ok(toml::from_str(input)?)
    }

    pub fn from_json(input: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(input)?)
    }

    pub fn from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => Self::from_toml(&content),
            Some("json") => Self::from_json(&content),
            _ => anyhow::bail!("Unsupported format: {}", path.display()),
        }
    }
}
```

### Enums con tags

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    Create { name: String, path: String },
    Delete { id: u64 },
    Update { id: u64, fields: Vec<String> },
}

// Se serializa como: {"type": "create", "name": "...", "path": "..."}
```

## Iterator Patterns

```rust
// Encadenar operaciones con iterators (zero-cost abstractions)
fn process_items(items: &[Item]) -> Vec<ProcessedItem> {
    items
        .iter()
        .filter(|item| item.is_active())
        .map(|item| item.transform())
        .collect()
}

// Collect en distintos tipos
let names: Vec<String> = items.iter().map(|i| i.name.clone()).collect();
let name_set: HashSet<String> = items.iter().map(|i| i.name.clone()).collect();
let by_id: HashMap<u64, &Item> = items.iter().map(|i| (i.id, i)).collect();

// Result con collect — falla al primer error
let results: Result<Vec<Output>, Error> = items
    .iter()
    .map(|item| process(item))
    .collect();

// Iterators custom
struct Chunks<'a> {
    data: &'a [u8],
    size: usize,
    pos: usize,
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let end = (self.pos + self.size).min(self.data.len());
        let chunk = &self.data[self.pos..end];
        self.pos = end;
        Some(chunk)
    }
}
```

## Async con Tokio

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

// Limitar concurrencia
async fn fetch_all(urls: Vec<String>) -> Vec<anyhow::Result<String>> {
    let semaphore = Arc::new(Semaphore::new(10)); // max 10 concurrent
    let mut handles = Vec::new();

    for url in urls {
        let permit = semaphore.clone();
        handles.push(tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            reqwest::get(&url).await?.text().await.map_err(Into::into)
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    results
}

// Channels para comunicacion entre tasks
use tokio::sync::mpsc;

async fn producer_consumer() -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::channel::<String>(100);

    let producer = tokio::spawn(async move {
        for i in 0..100 {
            tx.send(format!("item-{i}")).await.unwrap();
        }
    });

    let consumer = tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            println!("Got: {item}");
        }
    });

    producer.await?;
    consumer.await?;
    Ok(())
}
```

## Testing

### Unit tests

```rust
// En el mismo archivo que el codigo
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_input() {
        let result = parse("hello world").unwrap();
        assert_eq!(result.words(), 2);
    }

    #[test]
    fn parse_empty_input_fails() {
        let err = parse("").unwrap_err();
        assert!(matches!(err, ParseError::Syntax { line: 0, .. }));
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn index_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }
}
```

### Integration tests

```rust
// tests/integration_test.rs
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn cli_init_creates_project() {
    let tmp = TempDir::new().unwrap();

    Command::cargo_bin("mycli")
        .unwrap()
        .args(["init", "myproject"])
        .current_dir(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Created project"));

    assert!(tmp.path().join("myproject").exists());
    assert!(tmp.path().join("myproject/config.toml").exists());
}

#[test]
fn cli_run_with_invalid_input() {
    Command::cargo_bin("mycli")
        .unwrap()
        .args(["run", "--input", "nonexistent.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read file"));
}
```

### Snapshot testing con insta

```rust
use insta::assert_snapshot;
use insta::assert_json_snapshot;

#[test]
fn test_format_output() {
    let result = format_report(&sample_data());
    assert_snapshot!(result);
}

#[test]
fn test_json_output() {
    let config = Config::default();
    assert_json_snapshot!(config, @r###"
    {
      "projectName": "default",
      "version": "0.1.0",
      "port": 8080
    }
    "###);
}
```

### Async tests

```rust
#[tokio::test]
async fn test_fetch_data() {
    let result = fetch_data("http://example.com").await;
    assert!(result.is_ok());
}

// Con setup/teardown
#[tokio::test]
async fn test_with_server() {
    let server = start_test_server().await;
    let client = Client::new(&server.url());

    let response = client.get("/health").await.unwrap();
    assert_eq!(response.status(), 200);

    server.shutdown().await;
}
```

## Clippy y Formatting

### clippy

```bash
# Ejecutar con pedantic
cargo clippy -- -W clippy::pedantic

# Fix automatico
cargo clippy --fix --allow-dirty

# En CI — fail on warnings
cargo clippy -- -D warnings

# Permitir un lint especifico
#[allow(clippy::too_many_arguments)]
fn complex_function(/* ... */) {}

# A nivel de crate
#![allow(clippy::module_name_repetitions)]
```

### rustfmt

```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4
edition = "2021"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
use_field_init_shorthand = true
```

```bash
# Formatear
cargo fmt

# Check sin modificar (para CI)
cargo fmt -- --check
```

## Patrones Comunes

### Builder pattern

```rust
#[derive(Debug)]
pub struct Server {
    host: String,
    port: u16,
    workers: usize,
    tls: bool,
}

#[derive(Default)]
pub struct ServerBuilder {
    host: String,
    port: u16,
    workers: usize,
    tls: bool,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: num_cpus::get(),
            tls: false,
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    pub fn tls(mut self, tls: bool) -> Self {
        self.tls = tls;
        self
    }

    pub fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            workers: self.workers,
            tls: self.tls,
        }
    }
}

// Uso
let server = ServerBuilder::new()
    .host("0.0.0.0")
    .port(3000)
    .tls(true)
    .build();
```

### Newtype pattern

```rust
// Evita confundir tipos primitivos del mismo tipo
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

// Ahora es imposible pasar un UserId donde se espera OrderId
fn get_order(order_id: OrderId) -> Order {
    // ...
}

// Implementar Display y FromStr para ergonomia
impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserId {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
```

### From / Into conversiones

```rust
// Implementar From para conversiones naturales
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

// Into se obtiene gratis
let response: UserResponse = user.into();

// TryFrom para conversiones que pueden fallar
impl TryFrom<&str> for Config {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value).map_err(|e| ParseError::Syntax {
            line: 0,
            message: e.to_string(),
        })
    }
}
```

## Performance

### Benchmarks con criterion

```rust
// benches/parsing.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mylib::parse;

fn bench_parse(c: &mut Criterion) {
    let input = include_str!("../testdata/large_input.txt");

    c.bench_function("parse large file", |b| {
        b.iter(|| parse(black_box(input)))
    });
}

fn bench_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    let data = sample_data();

    group.bench_function("json", |b| {
        b.iter(|| serde_json::to_string(black_box(&data)))
    });

    group.bench_function("toml", |b| {
        b.iter(|| toml::to_string(black_box(&data)))
    });

    group.finish();
}

criterion_group!(benches, bench_parse, bench_formats);
criterion_main!(benches);
```

```bash
# Ejecutar benchmarks
cargo bench

# Comparar con baseline
cargo bench -- --save-baseline main
# ... hacer cambios ...
cargo bench -- --baseline main
```

### Tips de performance

```rust
// Preferir &str sobre String cuando no necesitas ownership
fn process(data: &str) -> &str { /* ... */ }

// Usar Cow para evitar clones innecesarios
use std::borrow::Cow;
fn normalize(input: &str) -> Cow<'_, str> {
    if input.contains('\t') {
        Cow::Owned(input.replace('\t', "    "))
    } else {
        Cow::Borrowed(input)
    }
}

// Pre-allocar con capacity
let mut results = Vec::with_capacity(items.len());
let mut map = HashMap::with_capacity(expected_size);

// Usar entry API para evitar double lookup
use std::collections::HashMap;
let count = map.entry(key).or_insert(0);
*count += 1;
```

## CI con GitHub Actions

```yaml
name: CI
on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - uses: Swatinem/rust-cache@v2

      - name: Format check
        run: cargo fmt -- --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Tests
        run: cargo test

      - name: Build release
        run: cargo build --release
```

## Mejores Practicas

### DO

- Usar `thiserror` en libraries, `anyhow` en binarios
- Commitear `Cargo.lock` para binarios, NO para libraries
- Usar `clippy::pedantic` y `cargo fmt` en CI
- Preferir `&str` sobre `String` en parametros de funcion
- Usar `#[derive(Debug)]` en todos los types
- Implementar `Display` para error types
- Usar `#[cfg(test)]` para modulos de test inline
- Pre-allocar vectors con `Vec::with_capacity`
- Usar `instrument` de tracing para logging automatico
- Definir MSRV en `Cargo.toml` (`rust-version = "1.75"`)
- Usar workspace dependencies para monorepos
- Preferir `impl Into<String>` en builders para ergonomia

### DON'T

- Usar `.unwrap()` en codigo de produccion — usar `?` o `expect("reason")`
- Usar `unsafe` sin justificacion documentada
- Ignorar warnings de clippy — arreglarlos o `allow` con razon
- Clonar innecesariamente — usar references y lifetimes
- Usar `String` donde `&str` es suficiente
- Hacer `println!` en libraries — usar `tracing` o retornar datos
- Crear enums de error sin implementar `std::error::Error`
- Ignorar `Cargo.lock` en binarios — debe estar en git
- Usar `Box<dyn Error>` en libraries — usar tipos concretos
- Retornar `impl Trait` en trait definitions (usar associated types)
- Hacer I/O en funciones puras — separar logica de I/O

## Recursos

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Error Handling in Rust](https://nick.groenen.me/posts/rust-error-handling/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Blessed.rs - Curated Crates](https://blessed.rs/)
- [This Week in Rust](https://this-week-in-rust.org/)
