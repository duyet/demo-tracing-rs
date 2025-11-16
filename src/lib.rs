use anyhow::{Context, Result};
use opentelemetry::global;
use tracing::{error, info, span, warn, Event, Level, Subscriber};
use tracing_subscriber::{
    layer::{Context as LayerContext, Layer},
    prelude::*,
    registry::LookupSpan,
};

/// A custom layer that demonstrates span context analysis.
///
/// In production, you might use this pattern to:
/// - Track span durations and emit metrics
/// - Implement custom sampling logic
/// - Filter or enrich spans based on context
/// - Bridge to other observability systems
pub struct PrintingLayer;

impl<S> Layer<S> for PrintingLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_event(&self, event: &Event, ctx: LayerContext<S>) {
        // Access the current span context
        let span = ctx.event_span(event);

        if let Some(span) = span {
            let name = span.name();
            let metadata = event.metadata();

            // Demonstrate accessing span hierarchy
            let scope: Vec<&str> = span.scope().map(|s| s.name()).collect();

            println!(
                "[PrintingLayer] Event '{}' in span '{}' (path: {})",
                metadata.name(),
                name,
                scope.join(" > ")
            );
        }
    }
}

/// Initialize the OpenTelemetry tracing pipeline.
///
/// This sets up:
/// 1. Jaeger exporter for distributed tracing
/// 2. Structured stdout logging
/// 3. Custom layer for span analysis
pub fn init_tracing() -> Result<()> {
    // Configure Jaeger exporter with error handling
    let otel_tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("data.transformation.agent")
        .with_endpoint("localhost:6831")
        .install_batch(opentelemetry::runtime::Tokio)
        .context(
            "Failed to initialize Jaeger exporter. Is Jaeger running? Try: docker-compose up -d",
        )?;

    // Create OpenTelemetry layer for distributed tracing
    let otel_layer = tracing_opentelemetry::layer().with_tracer(otel_tracer);

    // Create pretty-printed stdout layer for human-readable logs
    let stdout_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_target(true)
        .with_thread_ids(true);

    // Combine all layers into a single subscriber
    let subscriber = tracing_subscriber::registry()
        .with(otel_layer)
        .with(stdout_layer)
        .with(PrintingLayer);

    // Set as global default subscriber
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global tracing subscriber")?;

    info!("Tracing initialized successfully");
    Ok(())
}

/// Simulate a computational task with tracing.
///
/// The #[tracing::instrument] macro automatically:
/// - Creates a span with the function name
/// - Adds function arguments as span attributes
/// - Tracks entry and exit
#[tracing::instrument]
pub fn process_data(value: i32, factor: i32) -> Result<i32> {
    info!(value, factor, "Starting data processing");

    // Simulate validation
    if value < 0 {
        warn!(value, "Received negative value, applying absolute");
    }

    // Simulate processing
    let result = value.abs() * factor;

    info!(result, "Processing complete");
    Ok(result)
}

/// Simulate an async operation with nested spans.
#[tracing::instrument]
pub async fn fetch_and_transform(id: u64) -> Result<String> {
    info!(id, "Fetching data");

    // Simulate async I/O (e.g., database query)
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Create nested span for transformation logic
    let transform_span = span!(Level::DEBUG, "transform", id);
    let _enter = transform_span.enter();

    info!("Transforming data");
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;

    Ok(format!("transformed_{}", id))
}

/// Demonstrate error handling with tracing.
#[tracing::instrument]
pub async fn operation_that_may_fail(should_fail: bool) -> Result<()> {
    if should_fail {
        error!("Operation failed as requested");
        anyhow::bail!("Simulated failure for tracing demonstration");
    }

    info!("Operation succeeded");
    Ok(())
}

/// Gracefully shutdown the tracing provider, flushing all pending spans.
pub fn shutdown_tracing() {
    global::shutdown_tracer_provider();
}
