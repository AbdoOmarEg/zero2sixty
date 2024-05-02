use crate::email_client::EmailClient;
use crate::routes::{greet, health_check, subscribe};
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::routing::post;
use axum::{serve, Router};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing;

use sqlx::PgPool;

// / [...]
// You’ll have to add a #[derive(Clone)] to all the structs in src/configuration.rs to make the
// compiler happy, but we are done with the database connection pool.
//
//the compiler is happy without them

pub struct Application {
    port: u16,
    // the book actix's Server type that I don't have in axum
    // server: Server,
    server: serve::Serve<Router, Router>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let sender = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url.clone(),
            sender,
            configuration.email_client.authorization_token.clone(),
            timeout,
        );

        let address = format!(
            "{}{}",
            configuration.application.host, configuration.application.port
        );

        let listener = std::net::TcpListener::bind(&address)?;
        // let listener = tokio::net::TcpListener::bind(&address).await?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    // idk how to do it..for now
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// #[derive(Clone)]
// struct AppState {
//     db_pool: PgPool,
// }

// New imports!
use crate::configuration::{DatabaseSettings, Settings};
use sqlx::postgres::PgPoolOptions;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        // .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

#[allow(unused)]
#[derive(Clone)]
pub struct AppState {
    // db_pool: PgPool,
    email_client: Arc<EmailClient>,
    base_url: String,
}

pub fn run(
    listener: std::net::TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<serve::Serve<Router, Router>, std::io::Error> {
    // wrap it in an Arc pointer, if that fails maybe just derive clone on it's struct
    let email_client = Arc::new(email_client);
    let state = AppState {
        // db_pool: db_pool.clone(),
        email_client,
        base_url,
    };
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            // thanks to https://github.com/tokio-rs/axum/discussions/2273
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let request_id = uuid::Uuid::new_v4();
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str)
                    .unwrap_or("no matched_path");
                tracing::span!(
                    tracing::Level::INFO,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(request_id),
                    matched_path = tracing::field::display(matched_path)
                )
            }),
        )
        // it's better off to do the state like this
        // // ref: axum: https://github.com/tokio-rs/axum/blob/main/examples/oauth/src/main.rs#L78
        // #[derive(Clone)]
        // struct AppState {
        //     database: PgPool,
        //     email_client: EmailClient,
        //     base_url: ApplicationBaseUrl,
        // }
        //
        .with_state(db_pool.clone())
        .with_state(state);

    listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(listener)?;

    let server = axum::serve(listener, app);
    Ok(server)
}
