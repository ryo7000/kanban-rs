use crate::db;
use crate::models;
use axum::{
    extract::{Extension, Path},
    handler::{delete, get, patch, post},
    http::StatusCode,
    response::IntoResponse,
    routing::{BoxRoute, Router},
    AddExtensionLayer, Json,
};

// board routes

async fn boards(Extension(db): Extension<db::Db>) -> impl IntoResponse {
    Json(db.boards().await.unwrap())
}

async fn create_board(
    Json(input): Json<models::CreateBoard>,
    Extension(db): Extension<db::Db>,
) -> impl IntoResponse {
    Json(db.create_board(input).await.unwrap())
}

async fn board_summary(Path(id): Path<i64>, Extension(db): Extension<db::Db>) -> impl IntoResponse {
    Json(db.board_summary(id).await.unwrap())
}

async fn delete_board(Path(id): Path<i64>, Extension(db): Extension<db::Db>) -> impl IntoResponse {
    db.delete_board(id).await.unwrap();

    StatusCode::NO_CONTENT
}

// card routes

async fn cards(Path(id): Path<i64>, Extension(db): Extension<db::Db>) -> impl IntoResponse {
    Json(db.cards(id).await.unwrap())
}

async fn create_card(
    Json(input): Json<models::CreateCard>,
    Extension(db): Extension<db::Db>,
) -> impl IntoResponse {
    Json(db.create_card(input).await.unwrap())
}

async fn update_card(
    Path(id): Path<i64>,
    Json(input): Json<models::UpdateCard>,
    Extension(db): Extension<db::Db>,
) -> impl IntoResponse {
    Json(db.update_card(id, input).await.unwrap())
}

async fn delete_card(Path(id): Path<i64>, Extension(db): Extension<db::Db>) -> impl IntoResponse {
    db.delete_card(id).await.unwrap();

    StatusCode::NO_CONTENT
}

pub fn root(db: db::Db) -> Router<BoxRoute> {
    Router::new()
        .route("/boards", get(boards).post(create_board))
        .route("/boards/:id", delete(delete_board))
        .route("/boards/:id/summary", get(board_summary))
        .route("/boards/:id/cards", get(cards))
        .route("/cards", post(create_card))
        .route("/cards/:id", patch(update_card).delete(delete_card))
        .layer(AddExtensionLayer::new(db))
        .boxed()
}
