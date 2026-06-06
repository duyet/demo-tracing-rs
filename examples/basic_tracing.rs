/// Basic tracing example demonstrating fundamental patterns
///
/// Run with: `cargo run --example basic_tracing`
///
/// This example shows:
/// - Manual span creation
/// - Automatic instrumentation with `#[instrument]`
/// - Adding custom attributes to spans
/// - Nested spans and hierarchy
use tracing::{info, span, Level};
use tracing_subscriber;

#[tracing::instrument]
fn calculate_fibonacci(n: u32) -> u64 {
    info!(n, "Calculating Fibonacci number");

    if n <= 1 {
        return n as u64;
    }

    let mut a = 0u64;
    let mut b = 1u64;

    for i in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;

        // Log progress for large numbers
        if i % 10 == 0 {
            info!(iteration = i, current = b, "Progress update");
        }
    }

    info!(result = b, "Fibonacci calculation complete");
    b
}

fn process_numbers(numbers: Vec<u32>) {
    // Create a manual span with custom attributes
    let span = span!(
        Level::INFO,
        "process_numbers",
        count = numbers.len(),
        operation = "batch_fibonacci"
    );
    let _enter = span.enter();

    info!("Starting batch processing");

    for num in numbers {
        let result = calculate_fibonacci(num);
        info!(input = num, output = result, "Processed number");
    }

    info!("Batch processing complete");
}

fn main() {
    // Initialize simple console tracing (no Jaeger needed)
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .init();

    info!("Basic tracing example starting");

    // Example 1: Single calculation
    let result = calculate_fibonacci(10);
    info!(result, "Single calculation result");

    // Example 2: Batch processing with manual span
    process_numbers(vec![5, 8, 12, 15]);

    // Example 3: Nested spans
    let outer_span = span!(Level::INFO, "outer_operation");
    let _outer_guard = outer_span.enter();

    info!("In outer span");

    {
        let inner_span = span!(Level::INFO, "inner_operation", detail = "nested");
        let _inner_guard = inner_span.enter();

        info!("In inner span");
        let _ = calculate_fibonacci(7);
    }

    info!("Back in outer span");

    info!("Example complete");
}
