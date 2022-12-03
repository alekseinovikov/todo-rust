use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct UpdateTodo {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}
