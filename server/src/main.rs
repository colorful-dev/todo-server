use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;

#[derive(Clone)]
struct AppData {
    todos: Vec<Todo>,
}

#[derive(Serialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[get("/")]
async fn index(todos: Data<Vec<Todo>>) -> impl Responder {
    HttpResponse::Ok().json(web::Json(todos))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = AppData {
        todos: vec![] as Vec<Todo>,
    };
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_data.todos.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
