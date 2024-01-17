#[tracing::instrument]
fn hello_world() {
    tracing::info!("Hello, world!");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("OTEL_SERVICE_NAME", "basic");
    std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:4317");
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()
        .expect("init subscribers");

    hello_world();

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
