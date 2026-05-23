use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("c2.db")?;

    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            agent_id TEXT NOT NULL,
            last_seen TEXT NOT NULL
            )
        ")?;

    Ok(())
}