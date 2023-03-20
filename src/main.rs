use opentelemetry::{
    global,
    sdk::trace::{self, Sampler},
};
use tracing::{error, info, span};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    // Create a new tracer pipeline
    let jaeger_tracer = opentelemetry_jaeger::new_collector_pipeline()
        .with_endpoint("http://localhost:14268/api/traces")
        .with_service_name("data.transformation")
        .with_isahc()
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(jaeger_tracer);

    // Stdout
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // Use the tracing subscriber `Registry`
    tracing_subscriber::registry()
        .with(telemetry)
        .with(stdout_log)
        .init();

    // Spans will be sent to the configured OpenTelemetry exporter
    let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();

    error!(
        metric = "abc",
        "This error will be logged in the root span."
    );

    let items: Vec<_> = (0..5)
        .into_iter()
        .map(|i| {
            tokio::spawn(async move {
                let processing_file = span!(
                    tracing::Level::INFO,
                    "processing_file",
                    file = "file.json.gz",
                    step = i
                );
                let _enter_processing_file = processing_file.enter();
                info!(
                    file = "file.json.gz",
                    step = i,
                    "This event will be logged in the loop span."
                );
                error!("This error will be logged in the loop span.");

                my_func(i);
            })
        })
        .collect();

    futures::future::join_all(items).await;

    info!("This event will be logged in the root span.");

    // Shutdown trace pipeline
    global::shutdown_tracer_provider();
}

#[tracing::instrument]
fn my_func(val: i8) {
    info!("Calling my_func with argument {}", val);
}
