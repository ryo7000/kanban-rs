// required for rocket macros to work
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod db;
mod logger;
mod models;
mod schema;

type StdErr = Box<dyn std::error::Error>;

#[rocket::get("/")]
fn hellow_world() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    logger::init()?;

    rocket::ignite()
        .mount("/", rocket::routes![hellow_world])
        .launch();

    Ok(())
}
