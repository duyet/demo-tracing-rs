use anyhow::Result;
use demo_tracing::{fetch_and_transform, operation_that_may_fail, process_data};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Helper to initialize a test subscriber that captures events
fn init_test_tracing() {
    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::TRACE,
        ))
        .try_init();
}

#[test]
fn test_process_data_positive() {
    init_test_tracing();

    let result = process_data(10, 5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 50);
}

#[test]
fn test_process_data_negative() {
    init_test_tracing();

    // Negative values should be converted to absolute
    let result = process_data(-10, 5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 50);
}

#[test]
fn test_process_data_zero() {
    init_test_tracing();

    let result = process_data(0, 100);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[tokio::test]
async fn test_fetch_and_transform() {
    init_test_tracing();

    let result = fetch_and_transform(42).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "transformed_42");
}

#[tokio::test]
async fn test_fetch_and_transform_multiple() {
    init_test_tracing();

    let ids = vec![1, 2, 3, 4, 5];
    let futures: Vec<_> = ids.into_iter().map(|id| fetch_and_transform(id)).collect();

    let results = futures::future::join_all(futures).await;

    // All should succeed
    assert_eq!(results.len(), 5);
    for (idx, result) in results.iter().enumerate() {
        assert!(result.is_ok());
        assert_eq!(
            result.as_ref().unwrap(),
            &format!("transformed_{}", idx + 1)
        );
    }
}

#[tokio::test]
async fn test_operation_success() {
    init_test_tracing();

    let result = operation_that_may_fail(false).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_operation_failure() {
    init_test_tracing();

    let result = operation_that_may_fail(true).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.to_string().contains("Simulated failure"));
}

#[test]
fn test_process_data_with_span_context() {
    init_test_tracing();

    // Create a parent span
    let span = tracing::span!(Level::INFO, "test_parent_span");
    let _enter = span.enter();

    // This call should be nested within the parent span
    let result = process_data(100, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 200);
}

#[tokio::test]
async fn test_concurrent_operations() {
    init_test_tracing();

    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                let _ = fetch_and_transform(i).await;
                let _ = process_data(i as i32, 2);
            })
        })
        .collect();

    // All should complete successfully
    for handle in handles {
        assert!(handle.await.is_ok());
    }
}
