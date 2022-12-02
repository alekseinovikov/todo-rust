use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}