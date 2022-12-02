use std::error::Error;

use crate::domain::todo::Todo;

use super::TodoRepository;

#[derive(Clone)]
pub struct TodoRepositoryImpl;

pub fn new_todo_repository() -> TodoRepositoryImpl {
    TodoRepositoryImpl
}

impl TodoRepository for TodoRepositoryImpl {
    fn get_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        todo!()
    }

    fn get_by_id(&self, id: i32) -> Result<Option<Todo>, Box<dyn Error>> {
        todo!()
    }

    fn create(&self, todo: crate::domain::todo::CreateTodo) -> Result<Todo, Box<dyn Error>> {
        Ok(Todo { id: 1, title: todo.title, description: todo.description, completed: false })
    }

    fn update(&self, todo: crate::domain::todo::UpdateTodo) -> Result<Option<Todo>, Box<dyn Error>> {
        todo!()
    }

    fn mark_complemeted(&self, id: i32) -> Result<Option<Todo>, Box<dyn Error>> {
        todo!()
    }

    fn mark_uncompleted(&self, id: i32) -> Result<Option<Todo>, Box<dyn Error>> {
        todo!()
    }
}