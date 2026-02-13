use high_concurrency_api::config::get_configuration;
use high_concurrency_api::telemetry::{get_subscriber, init_subscriber};
use high_concurrency_api::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. telemetría (logs estructurados en json)
    let subscriber = get_subscriber(
        "high_concurrency_api".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // 2. cargar configuración desde yaml y variables de entorno
    let configuration = get_configuration().expect("Falló la lectura de la configuración");

    // 3. construir la app (pools de db, listeners, cache)
    let application = Application::build(configuration).await?;

    // 4. Correr la Aplicación
    tracing::info!("Iniciando aplicación en el puerto {}", application.port());
    application.run_until_stopped().await?;

    Ok(())
}
