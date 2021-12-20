#[macro_use] extern crate rocket;

use std::borrow::Cow;

use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::{Rocket, Build};
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use std::io;

type Id = usize;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[serde(crate = "rocket::serde")]
struct Jacky<'r> {
    id: Option<Id>,
    name: Cow<'r, str>,
    email: Cow<'r, str>,
    phone: Cow<'r, str>,
    message: Cow<'r, str>
}

type MessageList<'r> = Mutex<Vec<String>>;
type Jackies<'r> = &'r State<MessageList<'r>>;

#[post("/", format = "json", data = "<message>")]
async fn new<'x>(message: Json<Jacky<'_>>, list: Jackies<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    list.push(message.name.to_string());
    // list.push(Jacky {
    //     id: None,
    //     name: message.name.clone(),
    //     email: message.email.clone(),
    //     phone: message.phone.clone(),
    //     message: message.message.clone(),
    //
    // });

    json!({"status": "ok", "id": id})
}

#[get("/<id>", format = "json")]
async fn get(id: Id, list: Jackies<'_>) -> Option<Json<String>> {
    let list = list.lock().await;
    let local_jack = list.get(id);
    if let Some(j) = local_jack {
        Some(Json(j.into()))
    } else {
        None
    }
}

#[get("/all", format = "json")]
async fn get_all(list: Jackies<'_>) -> Json<Vec<String>> {
    let list = list.lock().await;
    Json(list.to_vec())
}

#[get("/jacky/<name>")]
fn jacky(name: &str) -> String {
    format!("{}y", name)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![jacky, new, get, get_all])
        .manage(MessageList::new(vec![]))
}
