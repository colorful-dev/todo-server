use crate::schema::todos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Deserialize,)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
}
