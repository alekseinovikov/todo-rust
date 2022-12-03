use std::error::Error;

use crate::domain::todo::{Todo, CreateTodo, UpdateTodo};

pub mod todo;

pub trait TodoRepository {
    fn get_all(&self) -> Result<Vec<Todo>, Box<dyn Error>>;
    fn get_by_id(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>>;
    fn create(&self, todo: CreateTodo) -> Result<Todo, Box<dyn Error>>;
    fn update(&self, id: i64, todo: UpdateTodo) -> Result<Option<Todo>, Box<dyn Error>>;
    fn mark_completed(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>>;
    fn mark_uncompleted(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>>;
}