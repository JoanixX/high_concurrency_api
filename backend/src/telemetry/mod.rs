use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub mod metrics;

// compone multiples capas en un subscriber de tracing
// usamos el impl subscriber como retorno para no escribir el tipo completo
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // filtro basado en la variable RUST_LOG
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Capa de formato compatible con bunyan en json
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// registramos el subscriber como global y solo se llama una vez
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirige logs estándar a tracing
    LogTracer::init().expect("Falló al setear el logger");

    // setea el subscriber global
    set_global_default(subscriber).expect("Falló al setear el subscriber");
}
