use rusqlite::{Connection, LoadExtensionGuard, Result};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn setup_database(db_path: &str, extension_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        unsafe {
            let _guard = LoadExtensionGuard::new(&conn)?;
            conn.load_extension(extension_path, None::<&str>)?;
        }

        conn.execute_batch(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS messages USING vss0(
                id INTEGER PRIMARY KEY,
                role TEXT,
                timestamp TEXT,
                content TEXT,
                embedding VECTOR(768)
            );"#,
        )?;
        conn.execute_batch(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS memories USING vss0(
                type TEXT,
                content TEXT,
                embedding VECTOR(768)
            );"#,
        )?;

        Ok(Database { connection: conn })
    }
}
