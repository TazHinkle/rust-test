use actix_files::{NamedFile, self as fs};
use actix_web::*;
use serde_derive::*;
use actix_web::{web, App, HttpResponse};
use std::sync::Mutex;

#[derive(Serialize, Debug)]
struct State {
    todo_items: Mutex<Vec<String>>,
}

#[derive(Deserialize)]
struct PostTodo {
    name: String,
}

#[derive(Deserialize)]
struct DeleteTodo {
    index: usize,
}

#[get("/api/todo")]
async fn get_data(data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok()
        .json(data.todo_items.lock().unwrap().clone())
}

#[post("/api/todo")]
async fn post_data(data: web::Data<State>, json: web::Json<PostTodo>) -> HttpResponse {
    let mut todo_items = data.todo_items.lock().unwrap();
    (*todo_items).push(json.name.to_owned());
    HttpResponse::Ok()
        .json(todo_items.clone())
}

#[delete("/api/todo/{index}")]
async fn delete_item(data: web::Data<State>, path: web::Path<DeleteTodo>) -> HttpResponse {
    let mut todo_items = data.todo_items.lock().unwrap();
    (*todo_items).remove(path.index);
    HttpResponse::Ok()
        .json(todo_items.clone())
}

#[get("/")]
async fn page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State {
        todo_items: Mutex::new(vec![
            "This".to_string(), 
            "Is".to_string(), 
            "Working!".to_string()
        ]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_data)
            .service(post_data)
            .service(delete_item)
            .service(page)
            .service(fs::Files::new("/static", "./pkg").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
