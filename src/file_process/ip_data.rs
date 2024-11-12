use rusqlite::{Connection,params};

pub struct ConnectionData{
    pub ip : String,
    pub port : u16,
    pub region : Option<String>,
    pub can_connected : Option<bool>,
    pub is_high_speed : Option<bool>,
}

pub fn create_table(conn: &Connection) -> Result<(),rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS connections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip TEXT NOT NULL,
            port INTEGER NOT NULL,
            region TEXT,
            can_connected BOOLEAN,
            is_high_speed BOOLEAN,
            UNIQUE(ip, port)  -- 添加唯一约束
        )",
        [],
    )?;
    Ok(())
}
/// 插入数据
pub fn insert_connection(conn: &Connection, data: &ConnectionData) -> Result<(),rusqlite::Error> {
    conn.execute(
        "INSERT INTO connections (ip, port, region, can_connected, is_high_speed) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            data.ip,
            data.port,
            data.region.as_ref().map(|s| s as &str), // 如果存在则插入，否则为 NULL
            data.can_connected.unwrap_or(false), // 默认值为 false
            data.is_high_speed.unwrap_or(false), // 默认值为 false
        ],
    )?;
    Ok(())
}

/// 清空数据库
pub fn clear_database(conn: &Connection) -> Result<(),rusqlite::Error> {
    // 删除表并重新创建
    conn.execute("DROP TABLE connections", [])?;
    create_table(conn)?;
    Ok(())
}

/// 改变可连接的布尔值
pub fn change_connected(conn: &Connection, ip: &str, port: u16, can_connected: bool) -> Result<(),rusqlite::Error> {
    conn.execute("UPDATE connections SET can_connected = ?1 WHERE ip = ?2 AND port = ?3", params![can_connected, ip, port])?;
    Ok(())
}

