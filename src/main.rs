#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate easy_error;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use easy_error::ResultExt;

mod database;
mod error;
mod model;
mod route;
mod schema;
mod service;

use crate::{database::Database, route::user};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let database_url =
        env::var("DATABASE_URL").context("DATABASE_URL is required")?;
    let database = Database::connect(&database_url)?;

    rocket::ignite()
        .manage(database)
        .mount("/", routes![user::list, user::get, user::create])
        .launch()?;

    Ok(())
}
