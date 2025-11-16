# 🔬 demo-tracing-rs

> **Distributed tracing in Rust, done right.**

A production-grade demonstration of OpenTelemetry tracing in Rust. This isn't just another "hello world" example—it's a comprehensive guide to understanding, implementing, and mastering distributed tracing in modern Rust applications.

## ✨ Why This Exists

Most tracing examples show you *how* to instrument code. This project shows you *why* it matters and *how* to do it elegantly in production systems.

You'll learn:
- **How distributed tracing reveals system behavior** you can't see any other way
- **Patterns for instrumenting** async Rust, error propagation, and concurrent operations
- **Performance implications** and how to minimize overhead
- **Real-world integration** with Jaeger and OpenTelemetry

## 🚀 Quick Start

Get up and running in 30 seconds:

```bash
# 1. Start Jaeger (tracing backend + UI)
docker-compose up -d

# 2. Run the demo
cargo run

# 3. View traces in Jaeger UI
open http://localhost:16686
```

That's it. You're now collecting and visualizing distributed traces.

## 🎯 What You'll See

When you run this demo, you'll observe:

1. **Trace propagation** - How spans nest and form a complete trace
2. **Structured events** - Custom attributes and logging within spans
3. **Multiple layers** - OpenTelemetry export + structured stdout + custom analysis
4. **Async context** - How tracing works seamlessly with Tokio

Look for the service `data.transformation.agent` in Jaeger UI and explore the trace timeline.

## 🏗️ Architecture

This demo showcases a **layered tracing architecture**:

```
┌─────────────────────────────────────────┐
│         Your Application Code           │
└──────────────┬──────────────────────────┘
               │ tracing macros (span!, info!, etc.)
               ▼
┌─────────────────────────────────────────┐
│      tracing::Subscriber Registry       │
├─────────────────────────────────────────┤
│  ┌─────────────────────────────────┐   │
│  │   OpenTelemetry Layer           │───┼──→ Jaeger (distributed tracing)
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │   Stdout Layer (pretty)         │───┼──→ Console output
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │   PrintingLayer (custom)        │───┼──→ Custom analysis
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

### Key Components

- **`tracing`** - Rust's structured, composable diagnostic framework
- **`tracing-subscriber`** - Registry and layer implementations
- **`tracing-opentelemetry`** - Bridge to OpenTelemetry ecosystem
- **`opentelemetry-jaeger`** - Jaeger exporter for trace visualization

## 📚 Understanding the Code

### The Subscriber Setup

```rust
let subscriber = tracing_subscriber::registry()
    .with(otel_layer)      // Export to Jaeger
    .with(stdout_layer)    // Human-readable console output
    .with(PrintingLayer);  // Custom span analysis
```

**Why layers?** Each layer processes the same tracing events independently. You can mix observability backends, add filtering, or implement custom logic—all without touching instrumentation code.

### Instrumentation Patterns

```rust
// 1. Automatic instrumentation (simplest)
#[tracing::instrument]
fn my_func(val: i8) {
    info!("Processing value {}", val);
}

// 2. Manual span creation (more control)
let span = span!(tracing::Level::TRACE, "app_start", work_units = 2);
let _enter = span.enter();
```

### The PrintingLayer

A custom layer that demonstrates how to:
- Intercept tracing events
- Access parent span context
- Implement custom analysis logic

This is where you'd add features like:
- Span duration tracking
- Error rate monitoring
- Custom metric extraction
- Dynamic sampling decisions

## 🔧 Configuration

The demo uses sensible defaults, but you can customize:

### Jaeger Endpoint

By default, traces export to `localhost:6831` (UDP). To change:

```rust
opentelemetry_jaeger::new_agent_pipeline()
    .with_endpoint("your-jaeger-host:6831")
    .with_service_name("your-service")
    // ...
```

### Trace Sampling

Currently set to 100% sampling (all traces). For production:

```rust
// TODO: Add sampler configuration example
```

## 🎓 Learning Path

1. **Start here**: Run the demo and explore Jaeger UI
2. **Read the code**: `src/main.rs` - Only ~60 lines, every one matters
3. **Experiment**: Add your own spans, attributes, and events
4. **Explore examples**: Check `/examples` for real-world patterns (coming soon)

## 🛠️ Development

```bash
# Build
cargo build

# Run with detailed logging
RUST_LOG=trace cargo run

# Format code
cargo fmt

# Run lints
cargo clippy

# Run tests
cargo test
```

## 🐛 Troubleshooting

### "Can't connect to Jaeger"

Ensure Jaeger is running:
```bash
docker-compose ps
curl http://localhost:16686
```

### "No traces appearing in Jaeger UI"

1. Check the service name - look for `data.transformation.agent`
2. Adjust time range in UI (default: last 1 hour)
3. Verify `global::shutdown_tracer_provider()` is called (flushes buffered spans)

### "Port already in use"

Jaeger uses several ports. Check if another Jaeger instance is running:
```bash
docker ps | grep jaeger
lsof -i :16686
```

## 📖 Additional Resources

- [OpenTelemetry Rust Docs](https://docs.rs/opentelemetry/)
- [Tracing Crate Guide](https://docs.rs/tracing/)
- [Jaeger Documentation](https://www.jaegertracing.io/docs/)
- [Distributed Tracing Concepts](https://opentelemetry.io/docs/concepts/observability-primer/)

## 🤝 Contributing

Found a way to make this demo even better? Contributions welcome!

Areas for improvement:
- [ ] More real-world examples (HTTP services, databases, etc.)
- [ ] Performance benchmarks showing tracing overhead
- [ ] Integration with other backends (Tempo, Zipkin, etc.)
- [ ] Advanced sampling strategies
- [ ] Error propagation patterns
- [ ] Multi-service distributed tracing demo

## 📜 License

This project is meant for learning. Use it, break it, improve it.

---

**Made with ❤️ for the Rust community**

*"The best way to understand distributed systems is to observe them in action."*
