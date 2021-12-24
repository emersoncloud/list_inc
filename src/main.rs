#[macro_use]
extern crate rocket;

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

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![jacky, new, get, get_all, good_jack, sendit, test_sendit],
        )
        .manage(ContactList::new(vec![]))
}

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

#[post("/send_email", format = "json", data = "<contact>")]
async fn sendit(contact: Json<Contact>) -> Value {
    dotenv::dotenv().expect("Failed to read .env file");

    let sg = SGClient::new(env::var("SENDGRID_API_KEY").unwrap());

    let x_smtpapi = String::from(r#"{"unique_args":{"test":7}}"#);

    let logan_address = env::var("TO_ADDRESS").unwrap();

    let from = contact.name.to_string();
    let subject = format!("New LIST Inc contact from {}", contact.name);
    let message = format!("Name: {}<br>Email: {}<br>Message: {}", contact.name, contact.email, contact.message);
    println!("{}", message);

    let mail_info = Mail::new()
        .add_to(Destination {
            address: logan_address.as_str(),
            name: "Logan List",
        })
        .add_from("emerson@emersoncloud.net")
        .add_from_name(from.as_str())
        .add_subject(subject.as_str())
        .add_html(message.as_str())
        .add_header("x-cool".to_string(), "indeed")
        .add_x_smtpapi(&x_smtpapi);

    let response = sg.send(mail_info).await.unwrap();

    println!("{:?}", response);
    json!(response.status().as_str())
}

#[post("/test_send_email", format = "json", data = "<_contacts>")]
async fn test_sendit(_contacts: Json<Contact>) -> Value {
    json!("200")
}
