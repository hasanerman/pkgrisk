use rusqlite::{params, Connection};
use std::path::PathBuf;

pub struct SqliteCache {
    conn: Connection,
}

impl SqliteCache {
    pub fn new(db_path: PathBuf) -> anyhow::Result<Self> {
        let conn = Connection::open(&db_path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cache (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn get(&self, key: &str, ttl_secs: u64) -> anyhow::Result<Option<String>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let mut stmt = self.conn.prepare("SELECT value, timestamp FROM cache WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;

        if let Some(row) = rows.next()? {
            let timestamp: i64 = row.get(1)?;
            if now as i64 - timestamp <= ttl_secs as i64 {
                let value: String = row.get(0)?;
                return Ok(Some(value));
            } else {
                // Expired, delete it
                self.conn.execute("DELETE FROM cache WHERE key = ?1", params![key])?;
            }
        }
        
        Ok(None)
    }

    pub fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        self.conn.execute(
            "INSERT OR REPLACE INTO cache (key, value, timestamp) VALUES (?1, ?2, ?3)",
            params![key, value, now as i64],
        )?;

        Ok(())
    }
}
