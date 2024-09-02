#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(FromForm, Serialize)]
struct Task {
    id: Option<i32>,
    description: String,
}

#[get("/")]
fn index() -> Template {
    let conn = Connection::open("tasks.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, description FROM tasks").unwrap();
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: Some(row.get(0)?),
            description: row.get(1)?,
        })
    }).unwrap();

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task.unwrap());
    }

    let context = serde_json::json!({ "tasks": tasks });
    Template::render("index", &context)
}

#[post("/add", data = "<task_form>")]
fn add(task_form: Form<Task>) -> Redirect {
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute("INSERT INTO tasks (description) VALUES (?1)",
                 params![task_form.description]).unwrap();
    Redirect::to(uri!(index))
}

#[post("/delete/<id>")]
fn delete(id: i32) -> Redirect {
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id]).unwrap();
    Redirect::to(uri!(index))
}

#[launch]
fn rocket() ->  _ {
    // Initialize the database
    let conn = Connection::open("tasks.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL
        )",
        [],
    ).unwrap();

    rocket::build()
        .mount("/", routes![index, add, delete])
        .attach(Template::fairing())
}
