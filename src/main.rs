#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::form::{Form, Contextual, FromForm};
use rocket::fs::{FileServer, relative};
use rocket::response::{Redirect};
use std::sync::Mutex;

mod tasks;


#[derive(Debug, FromForm)]
#[allow(dead_code)]
struct TaskForm<'v> {
    task: &'v str
}


#[get("/")]
fn index(con: &rocket::State<Mutex<redis::Connection>>) -> Template {
    Template::render("index", context! {
        tasks: tasks::get_tasks(&mut *con.lock().expect("Err"))
    })
}


#[post("/", data = "<form>")]
fn add_task<'r>(con: &rocket::State<Mutex<redis::Connection>>, form: Form<Contextual<'r, TaskForm<'r>>>) -> Template {
    let task_text = form.value.as_ref().unwrap();

    tasks::add_task(&mut *con.lock().expect("Err"), task_text.task.to_string());

    Template::render("index", context! {
        tasks: tasks::get_tasks(&mut *con.lock().expect("Err"))
    })
}

#[get("/api/<action>?<id>")]
fn api(con: &rocket::State<Mutex<redis::Connection>>, action: &str, id: i32) -> Redirect{

    match action {
        "ok" => tasks::mark_ok(&mut *con.lock().expect("Err"), id),
        "undo" => tasks::mark_undo(&mut *con.lock().expect("Err"), id),
        "del" => tasks::del_task(&mut *con.lock().expect("Err"), id),
        _ => {}
    }

    Redirect::to("/")
}


#[launch]
fn rocket() -> _ {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    rocket::build()
        .manage(Mutex::new(client.get_connection().unwrap()) )
        .mount("/", routes![index])
        .mount("/", routes![add_task])
        .mount("/", routes![api])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
}