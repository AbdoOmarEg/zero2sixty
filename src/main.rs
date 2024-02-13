use std::error::Error;

use sqlx::PgPool;
use zero2sixty::configuration::get_configuration;
use zero2sixty::startup::run;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();

    let app = run(connection_pool).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
