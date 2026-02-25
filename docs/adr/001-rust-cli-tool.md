# ADR-001: Rust as Implementation Language for CLI Tool

## Status

Accepted

## Date

2024-12-15

## Context

The Knowledge Framework (`kn`) is a meta-framework CLI that needs to be distributed to developers across multiple platforms (Linux, macOS, Windows). The tool must:

- Initialize projects with agent configurations
- Manage skills (download, install, sync)
- Generate issue templates for Beads
- Configure MCP servers
- Perform self-updates

Key requirements for the implementation language:
1. **Single binary distribution** - No runtime dependencies or interpreter installation
2. **Cross-platform support** - Linux x86_64, macOS (Intel/ARM), Windows
3. **Fast execution** - CLI commands should respond in milliseconds
4. **Type safety** - Prevent bugs during file operations and configuration management
5. **Ecosystem compatibility** - Strong HTTP client, JSON/TOML/YAML parsing, filesystem operations

## Decision

We will implement the `kn` CLI tool in **Rust** using the following libraries:
- **clap** (v4) - Command-line argument parsing with derive macros
- **reqwest::blocking** - HTTP client for skill downloads
- **serde** - Serialization/deserialization (JSON, YAML, TOML)
- **anyhow** - Error handling with context propagation
- **colored** - Terminal output formatting

We will use **blocking I/O** (not async) for simplicity, as CLI operations are short-lived.

## Alternatives Considered

### Alternative 1: Go
- **Pros**: Single binary, fast compilation, excellent cross-platform support, simpler than Rust
- **Cons**: Less type safety (no enums with associated data), weaker error handling, larger binaries
- **Why rejected**: Rust's type system prevents entire classes of bugs (especially important for filesystem operations). Error handling with `Result<T, E>` and `anyhow::Context` provides better UX for CLI error messages.

### Alternative 2: Python
- **Pros**: Rapid development, rich ecosystem, easier to learn
- **Cons**: Requires Python runtime, slower execution, distribution complexity (PyInstaller/cx_Freeze add bloat), harder to ensure reproducible installations
- **Why rejected**: Distribution is the killer - requiring users to have Python 3.11+ installed is a non-starter for a developer tool that should "just work".

### Alternative 3: Node.js (TypeScript)
- **Pros**: Strong ecosystem, good cross-platform support, familiar to frontend developers
- **Cons**: Requires Node.js runtime, larger installation footprint, slower startup times
- **Why rejected**: Runtime dependency is a dealbreaker. We want `curl | bash` installs that download a single binary, not `npm install -g`.

### Alternative 4: Bash script
- **Pros**: No compilation, works everywhere, familiar
- **Cons**: No type safety, terrible error handling, fragile across shell versions, complex logic is unmaintainable
- **Why rejected**: Fine for `install.sh` wrapper, but not for the main CLI with configuration management, HTTP downloads, and complex file operations.

## Consequences

### Positive

- **Zero runtime dependencies**: Users get a single binary that works immediately
- **Fast execution**: Typical commands complete in 10-50ms
- **Excellent error messages**: Rust's `Result` + `anyhow` provides contextual error chains
- **Type safety**: Compiler catches bugs before deployment (e.g., TOML schema mismatches)
- **Cross-compilation**: `cargo build --target` makes multi-platform releases straightforward
- **Strong ecosystem**: `clap`, `reqwest`, `serde` are production-grade libraries
- **Memory safety**: No segfaults or memory leaks in production

### Negative

- **Steeper learning curve**: Contributing requires Rust knowledge (mitigated by good docs)
- **Slower compile times**: Full release build takes ~2 minutes (vs. seconds for Go)
- **Larger team ramp-up**: Fewer developers know Rust than Python/JS
- **Async complexity avoided**: We use blocking I/O, which is simpler but limits future concurrency

### Risks

- **Rust version compatibility**: Requires `rustc 1.70+`
  - **Mitigation**: Document minimum version, provide binaries for users without Rust
- **Binary size**: Release builds are ~5-8 MB (larger than Go's ~2-3 MB)
  - **Mitigation**: Acceptable for developer tool, can strip symbols and UPX if needed
- **Windows cross-compilation**: More complex than Unix targets
  - **Mitigation**: Use GitHub Actions runners for native Windows builds

## References

- [Commit 09b6834](https://github.com/kobogithub/knowledge/commit/09b6834) - Initial Rust CLI implementation
- [Cargo.toml](../../cli/Cargo.toml) - Rust dependency specification
- [ARCHITECTURE.md](../ARCHITECTURE.md#1-cli-tool-rust) - CLI architecture documentation
- [Rust 2021 Edition](https://doc.rust-lang.org/edition-guide/rust-2021/) - Language version used
