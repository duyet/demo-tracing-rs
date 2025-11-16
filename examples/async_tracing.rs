/// Async tracing example demonstrating concurrent operations
///
/// Run with: cargo run --example async_tracing
///
/// This example shows:
/// - Tracing across async boundaries
/// - Context propagation in spawned tasks
/// - Concurrent operations with proper trace context
/// - Async instrumentation patterns
use tokio::time::{sleep, Duration};
use tracing::{info, instrument, span, warn, Level, Span};
use tracing_subscriber;

#[instrument]
async fn fetch_user(user_id: u64) -> Result<String, &'static str> {
    info!("Fetching user from database");
    sleep(Duration::from_millis(100)).await;

    if user_id == 0 {
        warn!("Invalid user ID");
        return Err("Invalid user ID");
    }

    Ok(format!("User_{}", user_id))
}

#[instrument]
async fn fetch_user_posts(user_id: u64) -> Vec<String> {
    info!("Fetching posts from database");
    sleep(Duration::from_millis(150)).await;

    vec![
        format!("Post 1 by user {}", user_id),
        format!("Post 2 by user {}", user_id),
    ]
}

#[instrument]
async fn fetch_user_profile(user_id: u64) -> Result<UserProfile, &'static str> {
    info!("Building complete user profile");

    // Fetch user and posts concurrently
    let (user_result, posts) = tokio::join!(fetch_user(user_id), fetch_user_posts(user_id));

    let username = user_result?;

    Ok(UserProfile {
        username,
        posts,
        followers: user_id * 10, // Simulate
    })
}

#[derive(Debug)]
struct UserProfile {
    username: String,
    posts: Vec<String>,
    followers: u64,
}

/// Demonstrates spawning tasks with proper trace context
async fn process_users_concurrently(user_ids: Vec<u64>) {
    let span = span!(
        Level::INFO,
        "process_users_concurrently",
        count = user_ids.len()
    );
    let _enter = span.enter();

    info!("Starting concurrent user processing");

    let mut handles = vec![];

    for user_id in user_ids {
        // IMPORTANT: Capture current span before spawning
        let span = Span::current();

        let handle = tokio::spawn(async move {
            // Enter the captured span in the spawned task
            let _enter = span.enter();

            match fetch_user_profile(user_id).await {
                Ok(profile) => {
                    info!(
                        username = %profile.username,
                        posts = profile.posts.len(),
                        followers = profile.followers,
                        "User profile fetched successfully"
                    );
                }
                Err(e) => {
                    warn!(user_id, error = e, "Failed to fetch user profile");
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let _ = handle.await;
    }

    info!("Concurrent processing complete");
}

/// Demonstrates select! with tracing
#[instrument]
async fn timeout_operation(duration_ms: u64) -> Result<String, &'static str> {
    info!("Starting operation with timeout");

    let operation = async {
        sleep(Duration::from_millis(duration_ms)).await;
        "Operation completed"
    };

    let timeout = sleep(Duration::from_millis(200));

    tokio::select! {
        result = operation => {
            info!("Operation completed successfully");
            Ok(result.to_string())
        }
        _ = timeout => {
            warn!("Operation timed out");
            Err("Timeout")
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .pretty()
        .init();

    info!("Async tracing example starting");

    // Example 1: Single async operation
    match fetch_user_profile(1).await {
        Ok(profile) => info!(?profile, "Profile fetched"),
        Err(e) => warn!(error = e, "Failed to fetch profile"),
    }

    // Example 2: Concurrent operations with proper context
    process_users_concurrently(vec![1, 2, 3, 4, 5]).await;

    // Example 3: Error handling
    match fetch_user_profile(0).await {
        Ok(_) => info!("Unexpected success"),
        Err(e) => info!(error = e, "Expected error occurred"),
    }

    // Example 4: Timeout operations
    let fast = timeout_operation(50).await;
    info!(?fast, "Fast operation result");

    let slow = timeout_operation(500).await;
    info!(?slow, "Slow operation result");

    info!("Example complete");
}
