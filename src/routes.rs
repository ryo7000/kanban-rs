use axum::{prelude::*, routing::BoxRoute};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub fn root() -> BoxRoute<Body> {
    route("/", get(hello_world)).boxed()
}
