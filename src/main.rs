use tracing::{error, info, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() {
    // Create a new tracer pipeline
    let tracer = opentelemetry_jaeger::new_collector_pipeline()
        .with_endpoint("http://localhost:14268/api/traces")
        .with_service_name("my_app")
        .with_isahc()
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        for i in 0..20 {
            let span_loop = span!(tracing::Level::TRACE, "loop", iteration = i);
            let _enter_span_loop = span_loop.enter();

            tokio::spawn(async move {
                let processing_file = span!(
                    tracing::Level::TRACE,
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
            });
        }

        info!("This event will be logged in the root span.");
    });
}
