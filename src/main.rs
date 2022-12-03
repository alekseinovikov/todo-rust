#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger, get, put};
use repository::{todo::new_todo_repository};
use service::{todo::{new_todo_service, TodoServiceImpl}, TodoService};
use crate::domain::todo::{CreateTodo, UpdateTodo};
use dotenv::dotenv;
use r2d2_sqlite::SqliteConnectionManager;
use crate::responses::*;

mod service;
mod domain;
mod repository;
mod responses;



#[get("/{id}")]
async fn get_todo(id: web::Path<i64>, service: web::Data<TodoServiceImpl>) -> impl Responder {
    service.get_by_id(id.into_inner())
        .unwrap_or_404()
}

#[get("/")]
async fn get_todos(service: web::Data<TodoServiceImpl>) -> impl Responder {
    service.get_all()
    .map(|todos| HttpResponse::Ok().json(todos))
    .unwrap_or_else(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[post("/")]
async fn post_todo(todo: web::Json<CreateTodo>, service: web::Data<TodoServiceImpl>) -> impl Responder {
    service.create(todo.into_inner())
        .map(|todo| HttpResponse::Ok().json(todo))
        .unwrap_or_else(|e| HttpResponse::InternalServerError().body(e.to_string()))
}

#[put["/{id}"]]
async fn update_todo(id: web::Path<i64>, todo: web::Json<UpdateTodo>, service: web::Data<TodoServiceImpl>) -> impl Responder {
    service.update(id.into_inner(), todo.into_inner())
        .unwrap_or_404()
}

#[put["/{id}/completed"]]
async fn mark_completed(id: web::Path<i64>, service: web::Data<TodoServiceImpl>) -> impl Responder {
    service.mark_completed(id.into_inner())
        .unwrap_or_404()
}

#[put["/{id}/uncompleted"]]
async fn mark_uncompleted(id: web::Path<i64>, service: web::Data<TodoServiceImpl>) -> impl Responder {
    service.mark_uncompleted(id.into_inner())
        .unwrap_or_404()
}

fn get_home_dir() -> String {
    match home::home_dir() {
        Some(path) => path.display().to_string(),
        None => panic!("Impossible to get home directory"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    let db_file_name = dotenv!("DB_FILE_NAME");
    let home_dir = get_home_dir();
    let db_path = format!("{}/{}", home_dir, db_file_name);
    let db_manager = SqliteConnectionManager::file(db_path);
    let db_pool = r2d2::Pool::new(db_manager).unwrap();

    let repository = new_todo_repository(db_pool);
    let service = new_todo_service(repository);

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(service.clone()))
        .service(post_todo)
        .service(get_todo)
        .service(get_todos)
        .service(update_todo)
        .service(mark_completed)
        .service(mark_uncompleted)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
