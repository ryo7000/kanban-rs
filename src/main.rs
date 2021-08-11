#[macro_use]
extern crate diesel;

mod logger;
mod models;
mod schema;

type StdErr = Box<dyn std::error::Error>;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    logger::init()?;

    Ok(())
}
