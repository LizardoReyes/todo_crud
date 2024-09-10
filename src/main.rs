#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::response::status;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: Option<i32>,
    description: String,
    #[serde(default)] // Esto establece `done` como false si no se envía en el JSON
    done: bool,
}

#[get("/tasks")]
async fn get_tasks() -> Json<Vec<Task>> {
    let conn = Connection::open("tasks.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, description, done FROM tasks").unwrap();
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: Some(row.get(0)?),
            description: row.get(1)?,
            done: row.get(2)?,
        })
    }).unwrap();

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task.unwrap());
    }

    Json(tasks)
}

#[post("/tasks", format = "json", data = "<task>")]
async fn add_task(task: Json<Task>) -> status::Created<Json<Task>> {
    let conn = Connection::open("tasks.db").unwrap();

    // Se guarda la tarea con 'done' por defecto como false
    conn.execute("INSERT INTO tasks (description, done) VALUES (?1, 0)",
                 params![task.description]).unwrap();

    let last_id = conn.last_insert_rowid();
    let new_task = Task {
        id: Some(last_id as i32),
        description: task.description.clone(),
        done: false,
    };

    status::Created::new("/tasks").body(Json(new_task))
}

#[put("/tasks/<id>/mark-done")]
async fn mark_done(id: i32) -> status::NoContent {
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute("UPDATE tasks SET done = 1 WHERE id = ?1", params![id]).unwrap();
    status::NoContent
}

#[put("/tasks/<id>/unmark-done")]
async fn unmark_done(id: i32) -> status::NoContent {
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute("UPDATE tasks SET done = 0 WHERE id = ?1", params![id]).unwrap();
    status::NoContent
}

#[delete("/tasks/<id>")]
async fn delete_task(id: i32) -> status::NoContent {
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id]).unwrap();
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    // Inicializa la base de datos
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    ).unwrap();

    rocket::build()
        .mount("/", routes![get_tasks, add_task, mark_done, unmark_done, delete_task])
}
