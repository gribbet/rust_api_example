#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;
use std::process;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[get("/")]
fn get() -> Json<User> {
    let user = User {
        id: 1,
        name: String::from("Testy"),
    };
    Json(user)
}

#[post("/", format = "json", data = "<user>")]
fn post(user: Json<User>) -> Json<User> {
    user
}

fn main() {
    if let Err(error) = rocket::ignite().mount("/", routes![get, post]).launch() {
        println!("Error: {}", error);
        process::exit(1)
    }
}
