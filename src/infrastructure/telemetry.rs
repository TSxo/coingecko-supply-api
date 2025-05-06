use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Sets up telemetry for the application.
pub fn setup_tracing<T, Sink>(app_name: T, sink: Sink)
where
    T: Into<String>,
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Create a formatting layer for development output.
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), sink);

    // Get log level from environment or use `info` as default.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Create the tracing subscriber.
    tracing_subscriber::registry()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();
}
