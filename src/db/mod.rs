use std::usize;

use tokio_rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Author {
    pub id: i32,
    pub created_at: String,
    pub name: String,
    pub bio: String,
}

pub async fn init(conn: &Connection) -> Result<usize> {
    conn.call(|conn| {
        Ok(conn.execute(
            "
            CREATE TABLE IF NOT EXISTS author (
                id INTEGER PRIMARY KEY,
                created_at TEXT NOT NULL,
                name TEXT NOT NULL,
                bio TEXT NOT NULL
            )
            ",
            [],
        )?)
    })
    .await
}
