use anyhow::Result;
use std::net::SocketAddr;

mod db;
mod logger;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    // loads env variables from .env
    dotenv::dotenv()?;
    let _guard = logger::init()?;

    let db = db::Db::connect().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(
            routes::root(db)
                .layer(logger::create_trace_layer())
                .into_make_service(),
        )
        .await?;

    Ok(())
}
