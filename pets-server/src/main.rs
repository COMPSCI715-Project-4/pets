mod routes;

use axum::{
    routing::{get, post},
    *,
};
use clap::Parser;
use mongodb::{options::ClientOptions, Client};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod handlers;

static CONFIG: once_cell::sync::OnceCell<Config> = once_cell::sync::OnceCell::new();
static DB_CLIENT: once_cell::sync::OnceCell<Client> = once_cell::sync::OnceCell::new();

/// Go walkies CLI -- Boots up Go walkies server.
#[derive(clap::Parser, Clone, Debug)]
struct Config {
    /// Socket address to listen on
    #[clap(short, long, env = "GO_WALKIES_ADDR", default_value = "127.0.0.1:8080")]
    addr: SocketAddr,
    /// MongoDB uri
    #[clap(
        short,
        long,
        env = "GO_WALKIES_DB_URI",
        default_value = "mongodb://localhost:27017"
    )]
    db_uri: String,

    #[clap(long, env = "GO_WALKIES_DB_NAME", default_value = "go_walkies")]
    db_name: String,

    /// secret used for signing jwt token
    #[clap(short, long, env = "GO_WALKIES_SECRET", default_value = "Go Walkies")]
    secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
    tracing_subscriber::fmt::fmt()
        .with_env_filter(filter)
        .with_file(false)
        .with_line_number(true)
        .with_target(true)
        .with_ansi(true)
        .init();

    let config = Config::parse();

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(&config.db_uri).await?;

    // Manually set an option.
    client_options.app_name = Some("Go Walkies".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    DB_CLIENT.set(client).unwrap();
    CONFIG.set(config.clone()).unwrap();

    let app = Router::new()
        .route("/login", post(handlers::login))
        .route("/signup", post(handlers::signup))
        .route("/pet/update", post(handlers::update_pet))
        .route("/ticket/create", post(handlers::create_ticket))
        .route("/ticket/fetch", post(handlers::fetch_tickets))
        .route("/rank", get(handlers::rank));

    tracing::debug!("listening on {}", config.addr);
    Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
