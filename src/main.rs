use secrecy::ExposeSecret;
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
    // expose secret
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();

    let app = run(connection_pool).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
