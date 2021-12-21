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

    json!({"status": "ok", "id": id})
}


#[post("/", format="text")]
fn whatever() -> Value {
    json!({"status": "bad jack"})
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

#[options("/")]
fn good_jacky() -> Value {
    json!({"hours": "8", "rate": "150.00", "payment_owed": "1200.00"})
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![jacky, new, get, get_all, whatever, good_jacky])
        .manage(MessageList::new(vec![]))
}
