use crate::database;
use chrono::{offset::Local, DateTime, NaiveDateTime};
use rusqlite::params;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        assert_eq!(
            new_task("New Task".to_string(), "2022-02-01 20:00:00".to_string()).unwrap(),
            "New Task".to_string()
        )
    }

    #[test]
    #[should_panic]
    fn test_new_task_fail() {
        assert!(new_task("New Task".to_string(), "invalid date".to_string()).is_ok())
    }

    #[test]
    fn test_get_tasks() {
        assert!(get_all_tasks().is_ok())
    }

    #[test]
    fn test_delete_task() {
        assert!(delete_task(&1).is_ok())
    }
}

fn parse_date(date: &String) -> Result<(), String> {
    match NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S") {
        Ok(_) => Ok(()),
        Err(_) => return Err("not a valid datetime.".to_string()),
    }
}

fn is_late(date: &String) -> bool {
    if date != "" {
        let current_date = Local::now().to_rfc2822();
        let len = current_date.len();
        let task_date = DateTime::parse_from_str(
            &format!("{} {}", date, &current_date[len - 5..]),
            "%Y-%m-%d %H:%M:%S %z",
        )
        .unwrap();
        if Local::now() >= task_date {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

struct Task {
    name: String,
    date: String,
    done: bool,
}

impl Task {
    fn new(name: String, date: String) -> Task {
        Task {
            name: name,
            date: date,
            done: false,
        }
    }
}

pub fn new_task(name: String, date: String) -> Result<String, Box<dyn Error>> {
    if date != "" {
        parse_date(&date)?
    }
    let conn = database::get_db();
    let task = Task::new(name, date);
    conn.execute(
        "INSERT INTO tasks (task_name, task_date,task_done) VALUES (?1, ?2, ?3)",
        params![task.name, task.date, task.done],
    )?;
    Ok(task.name)
}

pub fn get_all_tasks() -> Result<(), Box<dyn Error>> {
    let conn = database::get_db();
    let mut stmt = conn.prepare("SELECT task_name,task_date, task_done FROM tasks;")?;
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            name: row.get(0)?,
            date: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    for (index, task_item) in tasks.enumerate() {
        let task = task_item.unwrap();
        let warn = if is_late(&task.date) == true {
            "⚠"
        } else {
            ""
        };
        let sign = if task.done == true { "✓" } else { "⨯" };

        println!(
            "[{}] {}. {} ({}) {}",
            sign,
            index + 1,
            task.name,
            task.date,
            warn
        );
    }
    Ok(())
}

pub fn delete_task(id: &i32) -> Result<(), Box<dyn Error>> {
    let conn = database::get_db();
    conn.execute(
        "DELETE FROM tasks WHERE id in (SELECT id FROM tasks LIMIT 1 OFFSET ?1)",
        params![id - 1],
    )?;
    println!("Task deleted successfully.");
    Ok(())
}
