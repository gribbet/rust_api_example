#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod database;
mod error;
mod model;
mod route;
mod schema;
mod service;

use crate::database::Database;
use crate::route::user;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let database_url =
        env::var("DATABASE_URL").map_err(|_| "DATABASE_URL required")?;
    let database = Database::connect(&database_url)?;

    rocket::ignite()
        .manage(database)
        .mount("/", routes![user::list, user::get, user::create])
        .launch()?;

    Ok(())
}
