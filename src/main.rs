use actix_files::{NamedFile, self as fs};
use actix_web::*;
use serde_derive::*;

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