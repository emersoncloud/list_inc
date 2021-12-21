#[macro_use] extern crate rocket;

use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::{Rocket, Build};
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

type Id = usize;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[serde(crate = "rocket::serde")]

struct Contact {
    id: Option<Id>,
    name: String,
    email: String,
    phone: String,
    message: String
}

type ContactList = Mutex<Vec<Contact>>;
type Contacts<'r> = &'r State<ContactList>;

#[post("/", format = "json", data = "<message>")]
async fn new(message: Json<Contact>, list: Contacts<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();

    let mut with_id = message.0;
    with_id.id = Some(id);

    list.push(with_id);
    json!({"status": "ok", "id": id})
}

#[get("/<id>", format = "json")]
async fn get(id: Id, list: Contacts<'_>) -> Value {
    let list = list.lock().await;
    json!(list.get(id))
}

#[get("/all", format = "json")]
async fn get_all(list: Contacts<'_>) -> Json<Vec<Contact>> {
    let list = list.lock().await;
    Json(list.to_vec())
}

#[get("/jacky/<name>")]
fn jacky(name: &str) -> String {
    format!("{}y", name)
}

#[options("/")]
fn good_jack() -> Value {
    json!({"hours": "8", "rate": "150.00", "payment_owed": "1200.00"})
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![jacky, new, get, get_all, good_jack])
        .manage(ContactList::new(vec![]))
}
