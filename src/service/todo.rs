use std::error::Error;
use crate::{domain::todo::Todo, repository::{TodoRepository, todo::TodoRepositoryImpl}};
use super::TodoService;

#[derive(Clone)]
pub struct TodoServiceImpl{
    repository: TodoRepositoryImpl
}

pub fn new_todo_service(repository: TodoRepositoryImpl) -> TodoServiceImpl {
    TodoServiceImpl {
        repository
    }
}

impl TodoService for TodoServiceImpl {
    fn get_all(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        self.repository.get_all()
    }

    fn get_by_id(&self, id: i32) -> Result<Option<Todo>, Box<dyn Error>> {
        self.repository.get_by_id(id)
    }

    fn create(&self, todo: crate::domain::todo::CreateTodo) -> Result<Todo, Box<dyn Error>> {
        self.repository.create(todo)
    }

    fn update(&self, todo: crate::domain::todo::UpdateTodo) -> Result<Option<Todo>, Box<dyn Error>> {
        self.repository.update(todo)
    }

    fn mark_complemeted(&self, id: i32) -> Result<Option<Todo>, Box<dyn Error>> {
        self.repository.mark_complemeted(id)
    }

    fn mark_uncompleted(&self, id: i32) -> Result<Option<Todo>, Box<dyn Error>> {
        self.repository.mark_uncompleted(id)
    }
}