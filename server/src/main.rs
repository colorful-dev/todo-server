use actix_web::{
    delete, get, post, put,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use diesel::{RunQueryDsl, QueryDsl};
use diesel::prelude::*;

use serde::Deserialize;

mod models;
use models::todo::*;

mod db;

mod schema;
// use schema::todos::*;

// use uuid::Uuid;

#[derive(Deserialize)]
struct UpdateParams {
    id: i32,
}

#[derive(Deserialize)]
struct UpdateTodo {
    #[serde(default)]
    title: String,
    #[serde(default)]
    completed: bool,
}

#[get("/")]
async fn index(db: web::Data<db::Pool>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let result = schema::todos::table
        .load::<Todo>(&mut conn)
        .expect("Error loading todos");
    HttpResponse::Ok().json(web::Json(result))
}

#[post("/")]
async fn insert(db: web::Data<db::Pool>, todo: web::Json<NewTodo>) -> impl Responder {
    let mut conn = db.get().unwrap();
    // let uuid = Uuid::new_v4().to_string();
    let new_todo = NewTodo {
        title: todo.title.clone(),
    };
    let todo = diesel::insert_into(schema::todos::dsl::todos)
        .values(&new_todo)
        .get_result::<Todo>(&mut conn)
        .expect("Error saving new todo");
    HttpResponse::Created().json(web::Json(todo))
}

#[put("/{id}")]
async fn update(
    db: web::Data<db::Pool>,
    params: web::Path<UpdateParams>,
    update_data: web::Json<UpdateTodo>,
) -> impl Responder {
    use schema::todos::dsl::{todos, title, completed};
    let mut conn = db.get().unwrap();
    let id = params.id;
    let todo = diesel::update(todos.find(id))
        .set((
            title.eq(update_data.title.to_string()),
            completed.eq(update_data.completed),
        ))
        .get_result::<Todo>(&mut conn)
        .expect("Error updating todo");
    HttpResponse::Ok().json(web::Json(todo))
}

#[delete("/{id}")]
async fn remove(db: web::Data<db::Pool>, params: web::Path<UpdateParams>) -> impl Responder {
    let mut conn = db.get().unwrap();
    let id = params.id;
    let num_deleted = diesel::delete(schema::todos::dsl::todos.find(id))
        .execute(&mut conn)
        .expect("Error deleting todo");
    HttpResponse::Ok().json(web::Json(num_deleted))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::sql_connect();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(index)
            .service(insert)
            .service(update)
            .service(remove)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
