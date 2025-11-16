use anyhow::Result;
use demo_tracing::{
    fetch_and_transform, init_tracing, operation_that_may_fail, process_data, shutdown_tracing,
};
use tracing::{error, info, span, warn, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing (with proper error handling)
    init_tracing()?;

    // Create root span for the entire application lifecycle
    let root_span = span!(
        Level::INFO,
        "app_lifecycle",
        version = env!("CARGO_PKG_VERSION"),
        environment = "demo"
    );
    let _root_guard = root_span.enter();

    info!("Application starting");

    // Example 1: Simple function instrumentation
    match process_data(42, 2) {
        Ok(result) => info!(result, "Data processed successfully"),
        Err(e) => error!(error = %e, "Data processing failed"),
    }

    // Example 2: Negative value handling
    process_data(-10, 5)?;

    // Example 3: Async operations with nested spans
    let items = vec![1, 2, 3, 4, 5];
    let futures: Vec<_> = items
        .into_iter()
        .map(|id| fetch_and_transform(id))
        .collect();

    let results = futures::future::join_all(futures).await;
    info!(
        success_count = results.iter().filter(|r| r.is_ok()).count(),
        total_count = results.len(),
        "Batch processing complete"
    );

    // Example 4: Error propagation through spans
    if let Err(e) = operation_that_may_fail(false).await {
        error!(error = %e, "Unexpected failure in success case");
    }

    if let Err(e) = operation_that_may_fail(true).await {
        warn!(error = %e, "Expected failure occurred (this is OK for demo)");
    }

    info!("Application shutting down gracefully");

    // CRITICAL: Flush all pending spans to Jaeger before exit
    // Without this, buffered spans may be lost
    shutdown_tracing();

    info!("Tracing shutdown complete");
    Ok(())
}
