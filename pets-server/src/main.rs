mod routes;
mod schema;

use axum::{
    routing::{get, post},
    *,
};
use clap::Parser;
use routes::*;
use std::net::SocketAddr;

/// Pets CLI -- Boots up pets server.
#[derive(clap::Parser)]
struct Config {
    /// Socket address to listen on
    #[clap(short, long, env = "PETS_ADDR", default_value = "127.0.0.1:8080")]
    addr: SocketAddr,
    /// MongoDB uri
    #[clap(
        short,
        long,
        env = "PETS_DB",
        default_value = "mongodb://localhost:27017"
    )]
    db: String,
}

#[tokio::main]
async fn main() {
    let config = Config::parse();

    // build our application with a route
    let app = Router::new()
        .route("/rank", get(rank))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/pet/create", post(create_pet))
        .route("/pet/update", post(update_pet))
        .route("/pet/delete", post(delete_pet));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::debug!("listening on {}", config.addr);
    axum::Server::bind(&config.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
