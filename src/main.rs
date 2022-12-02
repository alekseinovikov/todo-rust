use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use repository::{todo::new_todo_repository, TodoRepository};
use service::{todo::{new_todo_service, TodoServiceImpl}, TodoService};
use crate::domain::todo::{Todo, CreateTodo};

mod service;
mod domain;
mod repository;


#[post("/")]
async fn post_todo(todo: web::Json<CreateTodo>, service: web::Data<TodoServiceImpl>) -> impl Responder {
    service.create(todo.into_inner())
        .map(|todo| HttpResponse::Ok().json(todo))
        .unwrap_or_else(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let repository = new_todo_repository();
    let service = new_todo_service(repository);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(service.clone()))
        .service(post_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
