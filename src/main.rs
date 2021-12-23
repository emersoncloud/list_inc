#[macro_use] extern crate rocket;

use rocket::State;
use rocket::tokio::sync::Mutex;
use rocket::{Rocket, Build};
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use dotenv;
use openapi::apis::{configuration::Configuration, default_api as twilio_api};
use std::env;

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

#[get("/twil")]
async fn twilio() {
    dotenv::dotenv().expect("Failed to read .env file");
    let account_sid = env::var("TWILIO_ACCOUNT_SID").unwrap();
    let api_key = env::var("TWILIO_API_KEY").unwrap();
    let api_key_sercret = env::var("TWILIO_API_KEY_SECRET").unwrap();
    let from = env::var("TWILIO_PHONE_NUMBER").unwrap();
    let to = env::var("TO_NUMBER").unwrap();

    let mut twilio_config = Configuration::default();
    twilio_config.basic_auth = Some((api_key, Some(api_key_sercret)));

    let message = twilio_api::create_message(
        &twilio_config,
        &account_sid,
        &to,
        None,
        None,
        None,
        Some("Ahoy, Rustacean! 🦀"),
        None,
        None,
        Some(&from),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
        .await;

    let result = match message {
        Ok(result) => result,
        Err(error) => panic!("Something went wrong {:?}", error),
    };
    println!("{:?}", result.sid);
}

#[options("/")]
fn good_jack() -> Value {
    json!({"hours": "8", "rate": "150.00", "payment_owed": "1200.00"})
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![jacky, new, get, get_all, good_jack, twilio])
        .manage(ContactList::new(vec![]))
}
