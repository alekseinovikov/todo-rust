use std::error::Error;

use crate::domain::todo::Todo;
use crate::domain::todo::CreateTodo;
use crate::domain::todo::UpdateTodo;

pub mod todo;

pub trait TodoService {
    fn get_all(&self) ->Result<Vec<Todo>, Box<dyn Error>>;
    fn get_by_id(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>>;
    fn create(&self, todo: CreateTodo) -> Result<Todo, Box<dyn Error>>;
    fn update(&self, id: i64, todo: UpdateTodo) -> Result<Option<Todo>, Box<dyn Error>>;
    fn mark_completed(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>>;
    fn mark_uncompleted(&self, id: i64) -> Result<Option<Todo>, Box<dyn Error>>;
}
