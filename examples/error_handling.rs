/// Error handling and tracing example
///
/// Run with: `cargo run --example error_handling`
///
/// This example shows:
/// - Error propagation through spans
/// - Using `#[instrument(err)]` for automatic error logging
/// - Custom error types with tracing
/// - Recovering from errors while maintaining trace context
use std::fmt;
use tracing::{error, info, instrument, warn, Level};
use tracing_subscriber;

#[derive(Debug)]
enum AppError {
    ValidationError(String),
    DatabaseError(String),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Demonstrates automatic error logging with `#[instrument(err)]`
#[instrument(err)]
fn validate_user_input(input: &str) -> Result<String, AppError> {
    info!("Validating user input");

    if input.is_empty() {
        // Error will be automatically logged by the instrument macro
        return Err(AppError::ValidationError("Input cannot be empty".into()));
    }

    if input.len() > 100 {
        return Err(AppError::ValidationError("Input too long".into()));
    }

    Ok(input.to_uppercase())
}

/// Demonstrates layered error handling
#[instrument(err, skip(validated))]
fn save_to_database(validated: String) -> Result<u64, AppError> {
    info!("Saving to database");

    // Simulate database error
    if validated.contains("ERROR") {
        return Err(AppError::DatabaseError("Connection failed".into()));
    }

    // Simulate successful save
    let id = validated.len() as u64;
    info!(record_id = id, "Successfully saved to database");
    Ok(id)
}

/// Demonstrates error recovery
#[instrument]
fn process_with_retry(input: &str, max_retries: u32) -> Result<u64, AppError> {
    info!(max_retries, "Processing with retry logic");

    let mut attempts = 0;

    loop {
        attempts += 1;

        match validate_user_input(input) {
            Ok(validated) => match save_to_database(validated) {
                Ok(id) => {
                    info!(attempts, id, "Processing succeeded");
                    return Ok(id);
                }
                Err(e) if attempts < max_retries => {
                    warn!(
                        attempts,
                        max_retries,
                        error = %e,
                        "Retry after error"
                    );
                    continue;
                }
                Err(e) => {
                    error!(attempts, error = %e, "Max retries exceeded");
                    return Err(e);
                }
            },
            Err(e) => {
                // Validation errors don't retry
                error!(error = %e, "Validation failed");
                return Err(e);
            }
        }
    }
}

/// Demonstrates bulk processing with partial failures
#[instrument]
fn process_batch(inputs: Vec<&str>) -> (Vec<u64>, Vec<AppError>) {
    info!(count = inputs.len(), "Processing batch");

    let mut successes = Vec::new();
    let mut failures = Vec::new();

    for (idx, input) in inputs.iter().enumerate() {
        let span = tracing::span!(Level::INFO, "process_item", index = idx, input);
        let _enter = span.enter();

        match process_with_retry(input, 2) {
            Ok(id) => {
                info!(id, "Item processed successfully");
                successes.push(id);
            }
            Err(e) => {
                warn!(error = %e, "Item processing failed");
                failures.push(e);
            }
        }
    }

    info!(
        success_count = successes.len(),
        failure_count = failures.len(),
        "Batch processing complete"
    );

    (successes, failures)
}

/// Demonstrates custom error context
#[instrument(err)]
fn lookup_user(user_id: u64) -> Result<String, AppError> {
    info!(user_id, "Looking up user");

    if user_id == 0 {
        return Err(AppError::ValidationError("Invalid user ID".into()));
    }

    if user_id > 1000 {
        return Err(AppError::NotFound(format!("User {} not found", user_id)));
    }

    Ok(format!("User_{}", user_id))
}

fn main() {
    // Initialize tracing with error level visible
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .pretty()
        .init();

    info!("Error handling example starting");

    // Example 1: Successful validation
    match validate_user_input("Hello World") {
        Ok(result) => info!(result, "Validation succeeded"),
        Err(e) => error!(error = %e, "Validation failed"),
    }

    // Example 2: Validation error
    match validate_user_input("") {
        Ok(_) => info!("Unexpected success"),
        Err(e) => info!(error = %e, "Expected validation error"),
    }

    // Example 3: Database error with retry
    match process_with_retry("ERROR_TRIGGER", 3) {
        Ok(id) => info!(id, "Unexpected success"),
        Err(e) => info!(error = %e, "Expected database error after retries"),
    }

    // Example 4: Batch processing with partial failures
    let inputs = vec!["valid1", "", "valid2", "ERROR_TRIGGER", "valid3"];
    let (successes, failures) = process_batch(inputs);

    info!(
        success_count = successes.len(),
        failure_count = failures.len(),
        "Batch results"
    );

    // Example 5: Different error types
    let _ = lookup_user(0);
    let _ = lookup_user(9999);
    match lookup_user(42) {
        Ok(user) => info!(user, "User found"),
        Err(e) => error!(error = %e, "User lookup failed"),
    }

    info!("Example complete");
}
