#[macro_use] extern crate rocket;

use rocket::{Rocket, Build};
use rocket::tokio::time::{sleep, Duration};
use rocket::tokio::task::spawn_blocking;

use std::io;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
    Ok(vec)
}

// #[get("/jacky/<input>")]
// fn jacky(input: &RawStr) -> String {
//     format!("{}{}", input.as_str(), "y")
// }

#[get("/jacky/<name>")]
fn jacky(name: &str) -> String {
    format!("{}y", name)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, delay, blocking_task, jacky])
}

// #[rocket::main]
// async fn main() {
//     rocket::build()
//         .mount("/hello", routes![index])
//         .launch()
//         .await;
// }
