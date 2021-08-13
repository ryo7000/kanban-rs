use axum::{http::StatusCode, prelude::*, response::IntoResponse};
use std::net::SocketAddr;

mod db;
mod logger;
mod models;

type StdErr = Box<dyn std::error::Error>;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    logger::init()?;

    let app = route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
