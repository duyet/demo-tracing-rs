# Architecture & Design Decisions

## Philosophy

This project demonstrates **production-grade distributed tracing** in Rust. Every design decision prioritizes:

1. **Clarity** - Code should teach, not obscure
2. **Composability** - Components work together elegantly
3. **Production-readiness** - Patterns you can use in real systems
4. **Performance** - Minimal overhead, efficient batching

## Core Concepts

### 1. The Tracing Abstraction

```
Application Code → tracing macros → Subscriber → Multiple Layers
```

**Why `tracing` instead of direct OpenTelemetry?**

- **Decoupling**: Instrumentation code doesn't depend on any specific backend
- **Composability**: Multiple layers process the same events independently
- **Ergonomics**: Rust-native API feels natural with async/await
- **Zero cost**: When disabled, tracing compiles to nearly nothing

### 2. The Layer Pattern

```rust
tracing_subscriber::registry()
    .with(otel_layer)      // Export to distributed tracing backend
    .with(stdout_layer)    // Human-readable console output
    .with(PrintingLayer);  // Custom analysis
```

**Design rationale:**

Each layer is a **separate concern**:
- **OpenTelemetry Layer**: Distributed tracing (Jaeger, Tempo, etc.)
- **Stdout Layer**: Development debugging and structured logs
- **Custom Layer**: Application-specific logic (metrics, filtering, etc.)

This follows the **Open/Closed Principle**—you can add functionality without modifying existing code.

### 3. Span Lifecycle

```
span creation → enter → events → nested spans → exit → export
```

**Key insights:**

1. **Spans are lazy** - Only activated when entered
2. **Context is automatic** - Tokio's task-local storage propagates spans across await points
3. **Structured data** - Attributes are typed, not just string concatenation
4. **Hierarchical** - Spans form a tree, showing causal relationships

### 4. Error Handling Strategy

```rust
fn init_tracing() -> Result<()> {
    let tracer = /* ... */.context("Helpful error message")?;
    // ...
}
```

**Why `anyhow`?**

- **Context**: Each error layer adds meaning
- **Ergonomic**: `?` operator just works
- **Debugging**: Full error chain in production logs

**Tracing errors vs returning errors:**

- Use `error!()` for expected failures you want to observe
- Use `Result<T, E>` for failures that affect control flow
- Do both when you want visibility AND propagation

### 5. Async Instrumentation

```rust
#[tracing::instrument]
async fn fetch_and_transform(id: u64) -> Result<String> {
    // Span automatically propagates across await points
    tokio::time::sleep(/* ... */).await;
    // Still in the same span
}
```

**How it works:**

1. Tokio stores span context in task-local storage
2. When a future yields (`.await`), context is saved
3. When resumed, context is restored
4. No manual propagation needed

**Caveat**: Spawned tasks don't inherit spans automatically. Use:

```rust
let span = tracing::Span::current();
tokio::spawn(async move {
    let _enter = span.enter();
    // Your code
});
```

## Component Deep-Dive

### PrintingLayer

```rust
impl<S> Layer<S> for PrintingLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_event(&self, event: &Event, ctx: LayerContext<S>) {
        // Access parent span hierarchy
        let span = ctx.event_span(event);
        // Custom logic here
    }
}
```

**Why a custom layer?**

This demonstrates the **extension point** for production needs:

- **Metrics extraction** - Emit Prometheus metrics from span durations
- **Dynamic sampling** - Sample 100% of errors, 1% of success
- **Cross-cutting concerns** - Add request IDs, user context, etc.
- **Bridge systems** - Forward to legacy logging infrastructure

**When to use:**

- You need behavior that doesn't fit existing layers
- You're building observability infrastructure
- You need fine-grained control over span processing

### Subscriber Registry

```rust
let subscriber = tracing_subscriber::registry()
    .with(layer1)
    .with(layer2)
    .with(layer3);
```

**Registry pattern:**

- **Type-safe composition** - Compile-time verification of layer compatibility
- **Efficient dispatch** - Virtual dispatch only where needed
- **Extensible** - Add layers without modifying core logic

**Performance note**: Each layer adds minimal overhead (~nanoseconds per event). The batching in OpenTelemetry layer amortizes export cost.

## Data Flow

### From Code to Jaeger

```
1. Application: span!(Level::INFO, "operation")
2. Tracing: Create span context, call subscriber
3. Registry: Dispatch to all layers
4. OTel Layer: Convert to OpenTelemetry span
5. Jaeger Exporter: Batch and send via UDP/HTTP
6. Jaeger Backend: Store and index trace
7. Jaeger UI: Query and visualize
```

**Critical points:**

- **Buffering**: Spans are batched for efficiency (configurable batch size)
- **Async export**: Network I/O doesn't block application threads
- **Flush on shutdown**: `global::shutdown_tracer_provider()` ensures no data loss

### Span Propagation in Async Code

```
[Task 1: HTTP Handler]
  ├─ span: "handle_request"
  ├─ tokio::spawn([Task 2])  ← context NOT propagated
  └─ await database_call()   ← context IS propagated
       └─ span: "db_query"
```

**Key insight**: Only explicit `.await` in the same task preserves context. Spawned tasks need manual span passing.

## Performance Considerations

### Overhead Analysis

| Operation | Cost | When |
|-----------|------|------|
| Span creation (not entered) | ~5-10ns | Always |
| Span enter/exit | ~50-100ns | Per scope |
| Event with formatting | ~200-500ns | Per log |
| Batch export (1000 spans) | ~1-5ms | Background, amortized |

**Optimization strategies:**

1. **Sampling**: Only trace a percentage of requests
2. **Level filtering**: Disable DEBUG/TRACE in production
3. **Lazy evaluation**: Use closure-based formatting
4. **Batch sizing**: Tune `max_batch_size` for your throughput

### Memory Usage

- **Span storage**: ~100-200 bytes per active span
- **Batch buffer**: ~10-100KB depending on configuration
- **Subscriber registry**: ~1KB (negligible)

**Scaling**: With default settings, a service handling 1000 req/s uses <10MB for tracing infrastructure.

## Design Trade-offs

### Why Jaeger over Other Backends?

**Chosen**: Jaeger
**Alternatives**: Zipkin, Tempo, AWS X-Ray

**Rationale**:
- ✅ Native OpenTelemetry support
- ✅ Easy local development (all-in-one Docker image)
- ✅ Excellent UI for trace visualization
- ✅ Battle-tested in production (Uber, CNCF project)

**Trade-off**: Vendor-neutral OpenTelemetry means switching backends is ~10 lines of code.

### Why UDP Agent Protocol?

**Chosen**: UDP agent (`localhost:6831`)
**Alternative**: HTTP collector (`POST http://localhost:14268/api/traces`)

**Rationale**:
- ✅ Fire-and-forget (doesn't block on network)
- ✅ Lower latency
- ✅ Simpler error handling
- ⚠️ Risk of data loss under extreme load

**Production recommendation**: Use HTTP collector with retry logic for critical traces.

### Why Multiple Layers?

**Chosen**: OpenTelemetry + Stdout + Custom
**Alternative**: Single layer to Jaeger

**Rationale**:
- ✅ Development experience (stdout for debugging)
- ✅ Demonstrates composability
- ✅ Fallback if Jaeger is down
- ⚠️ Slightly more overhead

**Production**: Remove stdout layer, keep OpenTelemetry + custom metrics layer.

## Instrumentation Patterns

### Pattern 1: Automatic (Preferred for Most Cases)

```rust
#[tracing::instrument]
fn process(value: i32) -> Result<Output> {
    // Span created automatically
}
```

**When**: Functions with clear boundaries, meaningful arguments

### Pattern 2: Manual (When You Need Control)

```rust
fn complex_logic() {
    let span = span!(Level::INFO, "operation", custom_field = value);
    let _enter = span.enter();
    // ...
}
```

**When**: Need custom attributes, conditional span creation, non-function scopes

### Pattern 3: Async Context Passing

```rust
async fn handler() {
    let span = Span::current();
    tokio::spawn(async move {
        let _enter = span.enter();
        // Work in spawned task
    });
}
```

**When**: Spawning background tasks that should inherit trace context

### Pattern 4: Error Propagation

```rust
#[tracing::instrument(err)]
async fn fallible_op() -> Result<()> {
    // Errors automatically logged at ERROR level
}
```

**When**: You want automatic error visibility without manual logging

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_with_tracing() {
    let subscriber = tracing_subscriber::registry()
        .with(/* test layer */);

    tracing::subscriber::with_default(subscriber, || {
        // Your test code
    });
}
```

### Integration Tests

- Spin up Jaeger in CI (docker-compose)
- Verify spans appear in Jaeger API
- Assert on span attributes, timing, relationships

## Security Considerations

### Sensitive Data

**Risk**: Tracing might capture sensitive information (PII, secrets)

**Mitigations**:
1. Use custom layer to filter/redact sensitive fields
2. Never log passwords, tokens, or full credit cards
3. Hash or truncate user identifiers
4. Configure sampling to avoid capturing sensitive requests

### Network Security

**Risk**: Unencrypted traces sent to Jaeger

**Production recommendations**:
1. Use TLS for Jaeger collector (HTTPS endpoint)
2. Run Jaeger in same VPC/network as services
3. Implement authentication on Jaeger UI
4. Encrypt traces at rest in Jaeger storage

## Future Enhancements

Potential improvements (in priority order):

1. **Configuration management** - Environment variables, config file
2. **Sampling strategies** - Adaptive sampling, error-biased sampling
3. **Metrics integration** - Emit RED metrics (Rate, Errors, Duration)
4. **Multi-service demo** - Show distributed tracing across services
5. **Performance benchmarks** - Quantify overhead with different configurations
6. **Alternative backends** - Examples for Tempo, Zipkin, AWS X-Ray

## References

- [OpenTelemetry Tracing Spec](https://opentelemetry.io/docs/reference/specification/trace/)
- [Tokio Tracing Guide](https://tokio.rs/tokio/topics/tracing)
- [Distributed Tracing Patterns](https://microservices.io/patterns/observability/distributed-tracing.html)
- [Jaeger Architecture](https://www.jaegertracing.io/docs/1.51/architecture/)

---

**Last Updated**: 2025-11-16
**Maintainers**: Open to community contributions
