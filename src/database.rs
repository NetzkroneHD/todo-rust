use crate::model;
use chrono::DateTime;
use rusqlite;
use rusqlite::{params, Connection};
use std::path::Path;

pub fn get_database_file_path() -> &'static Path {
    Path::new("test.db")
}

pub fn connect() -> Result<Connection, rusqlite::Error> {
    let path = get_database_file_path();
    Connection::open(path).map_or_else(|e| Err(e), |conn| Ok(conn))
}

pub fn create_tables(connection: &Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id       INTEGER PRIMARY KEY AUTOINCREMENT,
            name     VARCHAR(256) NOT NULL,
            done     BOOLEAN NOT NULL,
            deadline DATETIME
        );
    ", params![])?;
    Ok(())

}

pub fn create_task(connection: &Connection, name: String, done: bool, deadline: Option<DateTime<chrono::FixedOffset>>) -> Result<model::Task, rusqlite::Error> {
    let query = "INSERT INTO tasks (name, done, deadline) VALUES (?1, ?2, ?3)";

    connection.execute(query, params![name, done, deadline.map(|dt| dt.to_rfc3339())])?;
    Ok(model::Task::new(connection.last_insert_rowid(), name.to_string(), done, deadline))
}


pub fn update_task(connection: &Connection, task: &model::Task) -> Result<(), rusqlite::Error> {
    if exists_task(connection, task.get_id())? {
        connection.execute("UPDATE tasks SET name =?1, done = ?2, deadline = ?3 WHERE id = ?4",
                           params![task.name, task.done, task.deadline.map(|d| d.to_rfc3339()), task.get_id()])?;
        Ok(())
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn delete_task(connection: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    connection.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn exists_task(connection: &Connection, id: i64) -> Result<bool, rusqlite::Error> {
    connection.query_row("SELECT id FROM tasks WHERE id = ?1", params![id], |_| { Ok(true) }).or_else(|_| { Ok(false) })
}

pub fn get_all_tasks(connection: &Connection) -> Result<Vec<model::Task>, Box<dyn std::error::Error>> {
    let mut ps = connection.prepare("SELECT id, name, done, deadline FROM tasks")?;
    let rows = ps.query_map(params![], |row| {
        Ok(model::Task::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3).map_or_else(|_| None, |d: String| DateTime::parse_from_rfc3339(d.as_str()).map_or_else(|_| None, |d| Some(d)))))
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}