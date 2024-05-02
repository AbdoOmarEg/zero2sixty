// use sqlx::PgPool;
use std::error::Error;
use zero2sixty::configuration::get_configuration;
// use zero2sixty::email_client::EmailClient;
use zero2sixty::startup::{Application /*run*/};
use zero2sixty::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = get_subscriber("zero2sixty".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
