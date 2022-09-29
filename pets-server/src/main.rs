mod routes;

use axum::{
    routing::{get, post},
    *,
};
use axum_server::tls_rustls::RustlsConfig;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use clap::Parser;
use mongodb::{options::ClientOptions, Client};
use std::{net::SocketAddr, path::PathBuf};


mod db;
mod graphql;


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
    let config = Config::parse();

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(&config.db_uri).await?;

    // Manually set an option.
    client_options.app_name = Some("Go Walkies".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    DB_CLIENT.set(client).unwrap();
    CONFIG.set(config.clone()).unwrap();
    // // configure certificate and private key used by https
    // let tls_config = RustlsConfig::from_pem_file(
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("self_signed_certs")
    //         .join("cert.pem"),
    //     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //         .join("self_signed_certs")
    //         .join("key.pem"),
    // )
    // .await
    // .unwrap();

    let app = Router::new()
        .route("/login", post(graphql::login))
        .route("/signup", post(graphql::signup))
        .route("/pet/create", post(graphql::create_pet))
        .route("/pet/update", post(graphql::update_pet))
        .route("/ticket/create", post(graphql::create_ticket))
        .route("/ticket/fetch", post(graphql::fetch_tickets))
        ;

        
    Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
