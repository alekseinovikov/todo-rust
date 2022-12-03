use std::{error::Error};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

use crate::domain::todo::Todo;

use super::TodoRepository;

#[derive(Clone)]
pub struct TodoRepositoryImpl{
    db_pool: Pool<SqliteConnectionManager>,
}

pub fn new_todo_repository(db_pool: Pool<SqliteConnectionManager>) -> TodoRepositoryImpl {
    let repo = TodoRepositoryImpl{
        db_pool,
    };

    repo.init_schema().unwrap();
    repo
}

impl TodoRepositoryImpl {
    fn get_connection(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, Box<dyn Error>> {
        self.db_pool.get().map_err(|e| e.into())
    }

    fn init_schema(&self) -> Result<(), Box<dyn Error>> {
        let conn = self.get_connection()?;
        conn.execute("CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NULL,
            completed INTEGER NOT NULL
        )", [])?;
        Ok(())
    }

    fn map_row_to_todo(&self, row: &rusqlite::Row) -> Result<Todo, rusqlite::Error> {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            completed: row.get(3)?,
        })
    }

}

impl TodoRepository for TodoRepositoryImpl {
    fn get_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT \
                                                        id, \
                                                        title, \
                                                        description, \
                                                        completed \
                                                    FROM \
                                                        todo \
                                                    WHERE \
                                                        completed = 0 \
                                                    ORDER BY id DESC")?;
        let todo_iter = stmt
            .query_map([], |row| self.map_row_to_todo(row))?;
        todo_iter.collect::<Result<Vec<Todo>, rusqlite::Error>>().map_err(|e| e.into())
    }

    fn get_by_id(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT \
                                                        id, \
                                                        title, \
                                                        description, \
                                                        completed \
                                                  FROM \
                                                        todo \
                                                  WHERE \
                                                        id = ?1")?;
        stmt
            .query_row([id.to_string()], |row| self.map_row_to_todo(row))
            .optional()
            .map_err(|e| e.into())
    }

    fn create(&self, todo: crate::domain::todo::CreateTodo) -> Result<Todo, Box<dyn Error>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("INSERT INTO todo (title, description, completed) VALUES (?1, ?2, 0)")?;
        stmt.execute([todo.title, todo.description])?;

        let id = conn.last_insert_rowid();
        self.get_by_id(id).map(|todo| todo.unwrap())
    }

    fn update(&self, id: i64, todo: crate::domain::todo::UpdateTodo) -> Result<Option<Todo>, Box<dyn Error>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("UPDATE todo SET title = ?1, description = ?2 WHERE id = ?3")?;
        stmt.execute([todo.title, todo.description, id.to_string()])?;

        self.get_by_id(id)
    }

    fn mark_completed(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("UPDATE todo SET completed = 1 WHERE id = ?1")?;
        stmt.execute([id])?;

        self.get_by_id(id)
    }

    fn mark_uncompleted(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("UPDATE todo SET completed = 0 WHERE id = ?1")?;
        stmt.execute([id])?;

        self.get_by_id(id)
    }
}