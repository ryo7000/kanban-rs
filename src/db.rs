use crate::models::*;
use crate::StdErr;
use sqlx::{postgres::PgPoolOptions, Connection, PgConnection, Pool, Postgres};

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

struct Row {
    count: i64,
    status: Status,
}

impl From<Vec<Row>> for BoardSummary {
    fn from(rows: Vec<Row>) -> BoardSummary {
        let mut summary = BoardSummary::default();
        for row in rows {
            match row.status {
                Status::Todo => summary.todo += row.count,
                Status::Doing => summary.doing += row.count,
                Status::Done => summary.done += row.count,
            }
        }
        summary
    }
}

impl Db {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        Ok(Db { pool })
    }

    pub async fn boards(&self) -> Result<Vec<Board>, StdErr> {
        let boards = sqlx::query_as!(Board, "SELECT * FROM boards")
            .fetch_all(&self.pool)
            .await?;
        Ok(boards)
    }

    pub async fn board_summary(&self, board_id: i64) -> Result<BoardSummary, StdErr> {
        let counts = sqlx::query_as!(
            Row,
            r#"
                SELECT count(*) as "count!: _", status as "status: _" FROM cards WHERE board_id = $1 GROUP BY status
            "#,
            board_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(counts.into())
    }

    pub async fn create_board(&self, create_board: CreateBoard) -> Result<Board, StdErr> {
        let board = sqlx::query_as!(
            Board,
            "INSERT INTO boards (name) VALUES ($1) RETURNING *",
            create_board.name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(board)
    }

    pub async fn delete_board(&self, board_id: i64) -> Result<(), StdErr> {
        sqlx::query!("DELETE FROM boards WHERE id = $1", board_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn cards(&self, board_id: i64) -> Result<Vec<Card>, StdErr> {
        let cards = sqlx::query_as!(
            Card,
            r#"
                SELECT
                    id, board_id, description, status as "status: _", created_at
                FROM cards WHERE board_id = $1
            "#,
            board_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(cards)
    }

    pub async fn create_card(&self, create_card: CreateCard) -> Result<Card, StdErr> {
        let card = sqlx::query_as!(
            Card,
            r#"
                INSERT INTO cards (board_id, description) VALUES ($1, $2)
                RETURNING
                    id, board_id, description, status as "status: _", created_at
            "#,
            create_card.board_id,
            create_card.description,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(card)
    }

    pub async fn update_card(&self, card_id: i64, update_card: UpdateCard) -> Result<Card, StdErr> {
        let card = sqlx::query_as!(
            Card,
            r#"
                UPDATE cards SET description = $1, status = $2 WHERE id = $3
                RETURNING
                    id, board_id, description, status as "status: _", created_at
            "#,
            update_card.description,
            update_card.status as Status,
            card_id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(card)
    }

    pub async fn delete_card(&self, card_id: i64) -> Result<(), StdErr> {
        sqlx::query!("DELETE FROM cards WHERE id = $1", card_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
