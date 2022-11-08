use actix_web::{
    delete, get, post, put,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

struct AppData {
    current_id: Mutex<i32>,
    todos: Mutex<Vec<Todo>>,
}

#[derive(Serialize, Clone)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct AddTodo {
    title: String,
}

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
async fn index(data: Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(web::Json(&data.todos))
}

#[post("/")]
async fn insert(data: web::Data<AppData>, todo: web::Json<AddTodo>) -> impl Responder {
    let mut current_id = data.current_id.lock().unwrap();
    let mut todos = data.todos.lock().unwrap();
    let todo = Todo {
        id: current_id.clone(),
        title: todo.title.clone(),
        completed: false,
    };
    todos.push(todo.clone());
    *current_id += 1;
    HttpResponse::Created().json(web::Json(todo))
}

#[put("/{id}")]
async fn update(
    data: web::Data<AppData>,
    params: web::Path<UpdateParams>,
    update_data: web::Json<UpdateTodo>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = params.id;
    let exist = exist_by_id(id, &todos);
    match exist {
        Some(i) => {
            let todo = todos.get_mut(i).unwrap();
            let updated_todo = Todo {
                id: todo.id.clone(),
                title: update_data.title.clone(),
                completed: update_data.completed.clone(),
            };
            *todo = updated_todo.clone();
            HttpResponse::Ok().json(web::Json(updated_todo))
        }
        _ => HttpResponse::NotFound().finish(),
    }
}

#[delete("/{id}")]
async fn remove(data: web::Data<AppData>, params: web::Path<UpdateParams>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = params.id;
    let exist = exist_by_id(id, &todos);
    match exist {
        Some(i) => {
            todos.remove(i);
            HttpResponse::Ok().body("Todo deleted")
        }
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

fn exist_by_id(id: i32, state: &MutexGuard<Vec<Todo>>) -> Option<usize> {
    state.iter().position(|model| model.id == id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = Data::new(AppData {
        todos: Mutex::new(vec![]) as Mutex<Vec<Todo>>,
        current_id: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .service(insert)
            .service(update)
            .service(remove)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
