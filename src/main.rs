use axum::prelude::*;
use std::net::SocketAddr;

mod db;
mod logger;
mod models;
mod routes;

type StdErr = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    logger::init()?;

    let db = db::Db::connect().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(routes::root(db).into_make_service())
        .await?;

    Ok(())
}
