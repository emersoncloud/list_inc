#[macro_use]
extern crate rocket;

use dotenv;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;
use rocket::{Build, Rocket};
use std::env;

use sendgrid::SGClient;
use sendgrid::{Destination, Mail};

type Id = usize;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]

struct Contact {
    id: Option<Id>,
    name: String,
    email: String,
    phone: String,
    message: String,
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
        .mount(
            "/",
            routes![jacky, new, get, get_all, good_jack, sendit],
        )
        .manage(ContactList::new(vec![]))
}

#[post("/send_email")]
async fn sendit() -> Value {
    dotenv::dotenv().expect("Failed to read .env file");
    let api_key = env::var("SENDGRID_API_KEY").unwrap();
    let sg = SGClient::new(api_key);
    let mut x_smtpapi = String::new();
    x_smtpapi.push_str(r#"{"unique_args":{"test":7}}"#);
    let mail_info = Mail::new()
        .add_to(Destination {
            address: "emersoncloud@gmail.com",
            name: "you there",
        })
        .add_from("emerson@emersoncloud.net")
        .add_subject("Rust is rad")
        .add_html("<h1>Hello jack!</h1>")
        .add_from_name("test")
        .add_header("x-cool".to_string(), "indeed")
        .add_x_smtpapi(&x_smtpapi);

    let response = sg.send(mail_info).await.unwrap();

    json!(response.status().as_str())
}
