// for GET requests
use crate::schema::*;

#[derive(serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, diesel::Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, diesel_derive_enum::DbEnum, Debug)]
#[serde(rename_all = "camelCase")]
#[DieselType = "Status_enum"]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(Default, serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}

// this will be the result of out diesel::sql_query query
#[derive(diesel::QueryableByName)]
pub struct StatusCount {
    #[sql_type = "diesel::sql_types::BigInt"]
    pub count: i64,
    #[sql_type = "Status_enum"]
    pub status: Status,
}

// converting from a list of StatusCount to a BoardSummary
impl From<Vec<StatusCount>> for BoardSummary {
    fn from(counts: Vec<StatusCount>) -> BoardSummary {
        let mut summary = BoardSummary::default();
        for StatusCount { count, status } in counts {
            match status {
                Status::Todo => summary.todo += count,
                Status::Doing => summary.doing += count,
                Status::Done => summary.done += count,
            }
        }
        summary
    }
}

// for POST requests

#[derive(serde::Deserialize, diesel::Insertable)]
#[table_name = "boards"]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize, diesel::Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "cards"]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

// for PATCH requests

#[derive(serde::Deserialize, diesel::AsChangeset)]
#[table_name = "cards"]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}
