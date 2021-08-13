use crate::db;
use axum::{
    extract::Extension, prelude::*, response::IntoResponse, routing::BoxRoute, AddExtensionLayer,
};

async fn boards(Extension(db): Extension<db::Db>) -> impl IntoResponse {
    response::Json(db.boards().await.unwrap())
}

pub fn root(db: db::Db) -> BoxRoute<Body> {
    route("/boards", get(boards))
        .layer(AddExtensionLayer::new(db))
        .boxed()
}
