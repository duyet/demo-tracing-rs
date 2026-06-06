# Contributing to demo-tracing-rs

Thank you for your interest in improving this project! This guide will help you contribute effectively.

## Philosophy

This project exists to **teach distributed tracing in Rust**. Every contribution should make the project:
- More educational
- More production-ready
- More elegant
- Easier to understand

## How to Contribute

### Reporting Issues

Found a bug or have a suggestion?

1. Check [existing issues](https://github.com/duyet/demo-tracing-rs/issues) first
2. Create a new issue with:
   - Clear, descriptive title
   - Steps to reproduce (for bugs)
   - Expected vs actual behavior
   - Your environment (OS, Rust version)

### Suggesting Enhancements

Ideas for new examples or features?

1. Open an issue tagged `enhancement`
2. Explain the use case and why it matters
3. Propose the implementation approach
4. Reference real-world scenarios

### Pull Requests

#### Before You Start

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Set up your development environment:
   ```bash
   # Install Rust (if needed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install development tools
   rustup component add rustfmt clippy

   # Start Jaeger
   docker-compose up -d
   ```

#### Development Workflow

1. **Make your changes**
   ```bash
   # Edit code
   vim src/main.rs
   ```

2. **Format code**
   ```bash
   cargo fmt
   ```

3. **Run linter**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

5. **Test examples**
   ```bash
   cargo run --example basic_tracing
   cargo run --example async_tracing
   cargo run --example error_handling
   ```

6. **Verify with Jaeger**
   ```bash
   cargo run
   # Visit http://localhost:16686 and check traces
   ```

#### Code Standards

**Style:**
- Follow `rustfmt` defaults (enforced by CI)
- Use `clippy` recommendations (enforced by CI)
- Keep functions focused and small
- Prefer explicit over clever

**Documentation:**
- Every public function needs a doc comment
- Examples should have inline comments explaining **why**, not just what
- Update README.md if adding major features
- Update ARCHITECTURE.md if changing design patterns

**Testing:**
- Add tests for new functionality
- Ensure all tests pass: `cargo test`
- Examples should be runnable and demonstrate clear concepts

**Commits:**
- Use clear, descriptive commit messages
- Format: `<type>: <description>`
  - `feat: Add HTTP server tracing example`
  - `fix: Correct span context propagation in async code`
  - `docs: Update README with new examples`
  - `test: Add integration test for error handling`
  - `refactor: Simplify PrintingLayer implementation`

#### Submitting

1. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request**
   - Clear title describing the change
   - Reference related issues
   - Describe what you changed and why
   - Include screenshots/traces if relevant

3. **Respond to feedback**
   - Address review comments
   - Update code and tests as needed
   - Keep the conversation focused and constructive

## What We're Looking For

### High-Priority Contributions

1. **More real-world examples**
   - HTTP server with middleware
   - Database query tracing
   - Multi-service distributed tracing
   - gRPC service instrumentation
   - WebSocket tracing

2. **Advanced patterns**
   - Custom sampling strategies
   - Metrics extraction from traces
   - Performance benchmarks
   - Integration with other backends (Tempo, Zipkin)

3. **Documentation improvements**
   - Clearer explanations of concepts
   - Visual diagrams of trace flow
   - Troubleshooting guides
   - Video tutorials or blog posts (link them!)

4. **Testing enhancements**
   - More comprehensive integration tests
   - Performance regression tests
   - Chaos testing for error scenarios

### Guidelines for Examples

When adding new examples:

1. **Name clearly**: `examples/http_server_tracing.rs`
2. **Document purpose**: Top-level comment explaining what it demonstrates
3. **Self-contained**: Should run with `cargo run --example name`
4. **Commented**: Explain the "why" behind patterns
5. **Realistic**: Based on actual production use cases

Example template:

```rust
/// <One-line description>
///
/// Run with: cargo run --example <name>
///
/// This example shows:
/// - <Pattern 1>
/// - <Pattern 2>
/// - <Pattern 3>

use tracing::{info, instrument};

#[instrument]
fn example_function() {
    info!("Example with explanatory comments");
}

fn main() {
    // Initialize tracing
    // ... setup code ...

    // Demonstrate pattern 1
    // ... example code with comments ...

    // Demonstrate pattern 2
    // ... more examples ...
}
```

## Code Review Process

1. **Automated checks** run first (format, lint, tests)
2. **Maintainer review** focuses on:
   - Code quality and clarity
   - Educational value
   - Alignment with project goals
   - Documentation completeness
3. **Feedback iteration** until ready to merge
4. **Merge** when approved and CI passes

## Development Tips

### Local Testing

```bash
# Run all checks locally before pushing
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release

# Test all examples
for example in examples/*.rs; do
    name=$(basename "$example" .rs)
    cargo run --example "$name"
done
```

### Debugging Traces

```bash
# Enable verbose tracing
RUST_LOG=trace cargo run

# Check Jaeger for your traces
open http://localhost:16686

# Look for service: data.transformation.agent
```

### Performance Testing

```bash
# Build optimized
cargo build --release

# Run with timing
time ./target/release/demo_tracing

# Profile with flamegraph (optional)
cargo install flamegraph
cargo flamegraph
```

## Questions?

- Open a [GitHub Discussion](https://github.com/duyet/demo-tracing-rs/discussions)
- Tag issues with `question`
- Check existing issues and documentation first

## Recognition

Contributors will be recognized in:
- GitHub contributors page
- Release notes
- Special mention for significant contributions

Thank you for helping make distributed tracing in Rust more accessible!

---

**Remember**: The goal isn't just working code—it's code that teaches.
