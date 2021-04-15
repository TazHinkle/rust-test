use actix_files::{NamedFile, self as fs};
use actix_web::*;
use serde_derive::*;
use actix_web::{web, App, HttpResponse};

#[derive(Serialize, Debug)]
struct State {
    todo_items: Vec<String>,
}

#[get("/api/todo")]
async fn get_data(data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok()
        .json(data.todo_items.clone())
}

#[get("/")]
async fn page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}

// #[post("/api/todo")]
// async fn post_data(data: web::Data<State>) -> HttpRequest {
//     HttpResponse::Ok()
//         .json(data.todo_items.push("something".to_string()))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(State {
                todo_items: vec!["This".to_string(), "Is".to_string(), "Working!".to_string()],
            })
            .service(get_data)
            .service(page)
            .service(fs::Files::new("/static", "./pkg").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
