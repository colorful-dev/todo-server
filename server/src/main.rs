use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[get("/")]
async fn index() -> impl Responder {
    let obj = Todo {
        id: 1,
        title: "1".to_string(),
        completed: false,
    };
    HttpResponse::Ok().json(web::Json(obj))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
