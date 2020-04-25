#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use dotenv::dotenv;

mod schema;
mod users;
mod decks;
mod connection;
mod utils;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/users", users::router::routes())
        .mount("/decks", decks::router::routes())
        .launch();
}