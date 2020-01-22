#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> String {
    "Hello, world!".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch()
        .unwrap();
}
