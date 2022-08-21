mod routes;

use axum::{
    routing::{get, post},
    *,
};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use clap::Parser;
use std::net::SocketAddr;

use crate::graphql::{graphql_handler, graphql_playground};

mod db;
mod graphql;

/// Go walkies CLI -- Boots up Go walkies server.
#[derive(clap::Parser, Clone)]
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

    use mongodb::{options::ClientOptions, Client};

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(&config.db_uri).await?;

    // Manually set an option.
    client_options.app_name = Some("Go Walkies".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    let schema = Schema::build(graphql::Graphql, EmptyMutation, EmptySubscription)
        .data(client)
        .data(config.clone())
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
