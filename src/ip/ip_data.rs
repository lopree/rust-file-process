use rusqlite::{params, Connection, Result};

pub struct ConnectionData {
    pub ip: String,
    pub port: u16,
    pub region: Option<String>,         // 可选字段
    pub is_connected: Option<bool>,     // 可选字段
    pub is_high_speed: Option<bool>,    // 可选字段
}

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS connections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip TEXT NOT NULL,
            port INTEGER NOT NULL,
            region TEXT NOT NULL,
            is_connected BOOLEAN NOT NULL,
            is_high_speed BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(())
}
/// 插入数据
pub fn insert_connection(conn: &Connection, data: &ConnectionData) -> Result<()> {
    conn.execute(
        "INSERT INTO connections (ip, port, region, is_connected, is_high_speed) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            data.ip,
            data.port,
            data.region.as_ref().map(|s| s as &str), // 如果存在则插入，否则为 NULL
            data.is_connected.unwrap_or(false), // 默认值为 false
            data.is_high_speed.unwrap_or(false), // 默认值为 false
        ],
    )?;
    Ok(())
}


/// 更新数据库中的is_connected
pub fn update_connection(conn: &Connection, ip: &str, port: u16, is_connected: bool) -> Result<()> {
    conn.execute(
        "UPDATE connections SET is_connected = ?1 WHERE ip = ?2 AND port = ?3",
        params![is_connected, ip, port],
    )?;
    Ok(())
}
