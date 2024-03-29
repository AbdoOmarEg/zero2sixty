use sqlx::PgPool;
use std::error::Error;
use zero2sixty::configuration::get_configuration;
use zero2sixty::startup::run;
use zero2sixty::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = get_subscriber("zero2sixty".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(address).await?;

    let app = run(connection_pool).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
