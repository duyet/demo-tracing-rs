# CLAUDE.md - Project Guiding Principles

> **This document defines the essence of demo-tracing-rs**
> Everything built for this project must align with these principles.

## Project Vision

**demo-tracing-rs is the definitive reference implementation for distributed tracing in Rust.**

This is not a toy demo. This is not a "hello world" example. This is the project that developers study when they want to understand how distributed tracing *should* be done in production Rust applications.

### Core Tenets

1. **Code as Education** - Every line teaches. Every pattern is a lesson. Every example is a masterclass.

2. **Production-Ready by Default** - No shortcuts. No "this would never work in production" patterns. Everything demonstrated here can and should be used in real systems.

3. **Elegance Through Simplicity** - The best code isn't clever—it's clear. Prefer obvious over obscure. Remove complexity without losing power.

4. **Documentation is Code** - If it's not documented, it doesn't exist. Documentation should inspire, teach, and guide—not just describe.

5. **Zero-Friction Experience** - From `git clone` to viewing traces should take 30 seconds. Every barrier removed is a victory.

## Project Philosophy

### Why This Exists

Most tracing examples teach you *how* to add instrumentation. This project teaches you *why* it matters and *how* to do it right.

**Target Audience:**
- Rust developers learning distributed tracing
- Teams building production observability systems
- Engineers who want to understand OpenTelemetry deeply
- Anyone who values craft over convenience

**Not For:**
- People who just want copy-paste solutions (though they can get them here)
- Those satisfied with surface-level understanding
- Projects that prioritize speed over quality

### Design Philosophy

#### 1. The Craft of Code

```rust
// ❌ This works
let tracer = pipeline.install_batch(Tokio).expect("failed");

// ✅ This teaches and guides
let tracer = pipeline
    .install_batch(opentelemetry::runtime::Tokio)
    .context("Failed to initialize Jaeger exporter. Is Jaeger running? Try: docker-compose up -d")?;
```

**Principle**: Error messages should solve problems, not just report them.

#### 2. Documentation that Inspires

Every doc comment should answer:
- **What** does this do? (brief)
- **Why** does it exist? (context)
- **How** should you use it? (example)
- **When** is this the right pattern? (guidance)

#### 3. Examples as Teaching Tools

Each example must:
- Be runnable with zero setup (beyond `docker-compose up`)
- Demonstrate ONE core concept deeply
- Include comments explaining the "why"
- Show production patterns, not toy code
- Build on previous examples progressively

#### 4. Architecture as Expression

The code structure itself should teach:

```
src/
├── lib.rs          # Reusable components (what you'd use in production)
└── main.rs         # Demo application (how you'd use them)

examples/
├── basic_tracing.rs      # Fundamentals (start here)
├── async_tracing.rs      # Concurrent operations
└── error_handling.rs     # Production error patterns

tests/
└── integration_test.rs   # Validation (how to test tracing)
```

**Principle**: File organization tells a story.

## Technical Standards

### Code Quality

#### Rust Idioms

1. **Error Handling**
   - Use `Result<T, E>` for all fallible operations
   - Use `anyhow::Context` to add meaningful error context
   - Never use `.unwrap()` or `.expect()` without justification
   - Document error cases in function docs

2. **Async Patterns**
   - Always use `#[tracing::instrument]` on async functions
   - Document span propagation behavior
   - Show how to handle spawned tasks
   - Demonstrate concurrent operations properly

3. **Type Safety**
   - Prefer strong types over primitives
   - Use type system to prevent misuse
   - Document invariants in type definitions

4. **Naming**
   - Functions: `verb_noun` (e.g., `process_data`, `fetch_user`)
   - Types: `PascalCase` (e.g., `PrintingLayer`, `UserProfile`)
   - Constants: `SCREAMING_SNAKE_CASE`
   - Be descriptive, not clever

#### Documentation Standards

```rust
/// One-line summary of what this does.
///
/// Detailed explanation of why this exists and how it works.
///
/// # Examples
///
/// ```rust
/// let result = process_data(42, 2)?;
/// assert_eq!(result, 84);
/// ```
///
/// # Errors
///
/// Returns error if validation fails or processing errors occur.
///
/// # Panics
///
/// Never panics (or document when it does).
pub fn process_data(value: i32, factor: i32) -> Result<i32> {
    // Implementation
}
```

#### Testing Requirements

- Every public function must have tests
- Tests must use descriptive names: `test_process_data_with_negative_values`
- Include edge cases: zero, negative, max values
- Test error conditions, not just happy paths
- Integration tests should validate tracing behavior

### Tracing Patterns

#### When to Use What

1. **Automatic Instrumentation** (`#[tracing::instrument]`)
   - Functions with clear boundaries
   - When you want function arguments as span attributes
   - Most cases (prefer this)

2. **Manual Spans** (`span!(Level::INFO, "name")`)
   - Non-function scopes (loops, blocks)
   - Need custom attributes
   - Conditional span creation

3. **Events** (`info!`, `warn!`, `error!`)
   - Point-in-time occurrences
   - Structured logging within spans
   - Status updates

#### Span Hierarchy

Good tracing has a clear hierarchy:

```
app_lifecycle (root)
├── process_data (operation)
│   └── validate_input (sub-operation)
└── fetch_and_transform (async operation)
    └── transform (nested span)
```

**Principles:**
- Root spans represent the entire operation lifecycle
- Child spans represent logical sub-operations
- Depth should rarely exceed 5 levels
- Each level should add meaningful context

### Performance Considerations

Tracing adds overhead. We must be honest about this and minimize it:

1. **Sampling** - Not every request needs tracing
2. **Level filtering** - Disable DEBUG/TRACE in production
3. **Lazy evaluation** - Use closure-based formatting
4. **Batching** - Buffer and send in batches

Document performance implications in examples.

## Project Structure

### What Lives Where

```
/
├── CLAUDE.md                      # This file - the source of truth
├── CLAUDE_README.md               # What this project is
├── CLAUDE_ARCHITECTURE.md         # How it's designed
├── CLAUDE_CONTRIBUTING.md         # How to contribute
├── docker-compose.yml             # One-command Jaeger setup
├── Cargo.toml                     # Dependencies (no wildcards!)
├── .github/workflows/ci.yml       # Quality gates
├── src/
│   ├── lib.rs                     # Reusable tracing components
│   └── main.rs                    # Demo application
├── examples/                      # Teaching examples
│   ├── basic_tracing.rs           # Start here
│   ├── async_tracing.rs           # Async patterns
│   └── error_handling.rs          # Error propagation
└── tests/
    └── integration_test.rs        # Validation
```

### Adding New Files

**Before creating any new file, ask:**
1. Does this serve the educational mission?
2. Is this production-ready?
3. Does this fit the existing structure?
4. Is it documented?

**If adding an example:**
- Name clearly: `<concept>_tracing.rs`
- Add header doc explaining what it teaches
- Make it runnable standalone
- Reference it in CLAUDE_README.md

**If adding a module:**
- Document its purpose in module-level doc
- Export only what's needed
- Add comprehensive tests
- Update CLAUDE_ARCHITECTURE.md

## Development Workflow

### The Craft Process

1. **Understand** - Read CLAUDE_README.md, CLAUDE_ARCHITECTURE.md
2. **Plan** - Design before coding
3. **Implement** - Follow standards, document as you go
4. **Validate** - Test, lint, review
5. **Refine** - Iterate until elegant
6. **Document** - Update relevant CLAUDE_*.md files

### Quality Gates

Before any commit:

```bash
# Format
cargo fmt --all

# Lint (zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test

# Examples
cargo run --example basic_tracing
cargo run --example async_tracing
cargo run --example error_handling

# Main demo
cargo run
```

### Commit Standards

Format: `<type>: <description>`

**Types:**
- `feat:` - New feature or example
- `fix:` - Bug fix
- `refactor:` - Code restructuring
- `docs:` - Documentation updates
- `test:` - Test additions/changes
- `chore:` - Maintenance tasks

**Examples:**
```
feat: Add HTTP server tracing example
fix: Correct span context in spawned tasks
refactor: Simplify PrintingLayer implementation
docs: Update CLAUDE_ARCHITECTURE.md with new patterns
test: Add concurrent operation tests
chore: Update dependencies
```

## What Good Looks Like

### Example: Good Function

```rust
/// Simulate a computational task with proper tracing and error handling.
///
/// This demonstrates how to:
/// - Instrument a function automatically
/// - Add structured attributes
/// - Handle errors within traced contexts
/// - Log progress for long operations
///
/// # Examples
///
/// ```rust
/// let result = process_data(42, 2)?;
/// assert_eq!(result, 84);
/// ```
///
/// # Errors
///
/// Returns error if the computation overflows or validation fails.
#[tracing::instrument]
pub fn process_data(value: i32, factor: i32) -> Result<i32> {
    info!(value, factor, "Starting data processing");

    // Validate inputs
    if value < 0 {
        warn!(value, "Received negative value, applying absolute");
    }

    // Perform computation
    let result = value.abs() * factor;

    info!(result, "Processing complete");
    Ok(result)
}
```

**Why this is good:**
- Doc comment teaches, not just describes
- Error handling is explicit
- Logging provides visibility
- Function is testable
- Follows Rust idioms

### Example: Good Example File

See `examples/error_handling.rs` - it demonstrates:
- Clear header documentation
- Progressive complexity
- Production patterns
- Comprehensive comments
- Runnable with `cargo run --example error_handling`

## Anti-Patterns to Avoid

### ❌ Bad Practices

1. **Wildcard dependencies**
   ```toml
   # ❌ Never do this
   opentelemetry = "*"

   # ✅ Always pin versions
   opentelemetry = "0.19.0"
   ```

2. **Poor error messages**
   ```rust
   // ❌ Unhelpful
   .expect("failed to initialize")

   // ✅ Actionable
   .context("Failed to initialize Jaeger. Is it running? Try: docker-compose up -d")?
   ```

3. **Missing documentation**
   ```rust
   // ❌ No context
   fn process(x: i32) -> i32 { x * 2 }

   // ✅ Clear purpose
   /// Doubles the input value for demonstration purposes.
   fn process(x: i32) -> i32 { x * 2 }
   ```

4. **Ignoring context in async**
   ```rust
   // ❌ Span not propagated
   tokio::spawn(async { /* work */ });

   // ✅ Span explicitly passed
   let span = Span::current();
   tokio::spawn(async move {
       let _enter = span.enter();
       /* work */
   });
   ```

5. **Testing without assertions**
   ```rust
   // ❌ Just running code
   #[test]
   fn test_process() {
       process_data(10, 2);
   }

   // ✅ Validating behavior
   #[test]
   fn test_process_doubles_positive_values() {
       let result = process_data(10, 2).unwrap();
       assert_eq!(result, 20);
   }
   ```

## Related Documents

This is the master document. For specific details:

- **CLAUDE_README.md** - Project overview, quick start, learning path
- **CLAUDE_ARCHITECTURE.md** - Technical design, patterns, trade-offs
- **CLAUDE_CONTRIBUTING.md** - Development workflow, PR process, standards

All four documents together define the complete picture of this project.

## Measuring Success

This project succeeds when:

1. **Developers learn** - People understand tracing deeply, not superficially
2. **Code ships** - Patterns from here appear in production systems
3. **Community grows** - Contributors add examples and improvements
4. **Standards rise** - Other projects use this as a reference
5. **Quality improves** - Every change makes the project better, not just bigger

## The Standard

Every contribution should be measured against this question:

> **"Is this something I'd be proud to show as a reference implementation?"**

If the answer is anything less than an enthusiastic "yes," it's not ready.

---

## Final Principle

> *"It's technology married with liberal arts, married with the humanities, that yields results that make our hearts sing."*

This project is about more than distributed tracing. It's about the craft of software engineering. It's about building things that work beautifully and teach elegantly.

**Make every line count.**

---

**Document Version**: 1.0
**Last Updated**: 2025-11-16
**Status**: Living document - evolves with the project
