use dirs;
use rusqlite::Connection;
use std::{fs, path, process};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        assert!(create_tables().is_ok())
    }
}

fn get_db_path() -> String {
    let path = match dirs::home_dir() {
        Some(val) => val.join(".todo_cli"),
        None => path::PathBuf::from("./"),
    };
    match fs::read_dir(&path) {
        Ok(_val) => {}
        Err(_err) => {
            fs::create_dir(&path).unwrap_or_else(|err| {
                eprintln!("error: Error creating data directory ({})", err);
                process::exit(1);
            });
        }
    };
    path.join("todo_db.sqlite3")
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn get_db() -> Connection {
    let conn = match Connection::open(get_db_path()) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    };
    conn
}

pub fn create_tables() -> rusqlite::Result<()> {
    let conn = get_db();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            task_name	TEXT NOT NULL,
            task_date	TEXT,
            task_done	INTEGER NOT NULL
            )",
        [],
    )?;
    Ok(())
}
