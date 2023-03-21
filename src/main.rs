use opentelemetry::global;
use tracing::{info, span, Event, Subscriber};
use tracing_subscriber::{
    layer::{Context, Layer},
    prelude::*,
    registry::LookupSpan,
};

struct PrintingLayer;
impl<S> Layer<S> for PrintingLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_event(&self, event: &Event, ctx: Context<S>) {
        let span = ctx.event_span(event);
        println!("Event in span: {:?}", span.map(|s| s.name()));
    }
}

#[tokio::main]
async fn main() {
    let otel_tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("data.transformation.agent")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Error initializing Jaeger exporter");

    // Create a tracing layer with the configured tracer
    let otel_layer = tracing_opentelemetry::layer().with_tracer(otel_tracer);

    // Stdout
    let stdout_layer = tracing_subscriber::fmt::layer().pretty();

    // Use the tracing subscriber `Registry`
    let s = tracing_subscriber::registry()
        .with(otel_layer)
        .with(stdout_layer)
        .with(PrintingLayer);

    tracing::subscriber::with_default(s, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        my_func(12);

        info!(
            metric = "abc",
            "This error will be logged in the root span."
        );
    });

    global::shutdown_tracer_provider();
}

#[tracing::instrument]
fn my_func(val: i8) {
    info!("Calling my_func with argument {}", val);
}
