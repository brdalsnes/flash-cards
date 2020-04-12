#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use dotenv::dotenv;

mod schema;
mod decks;
mod connection;

fn main() {
    dotenv().ok();
    decks::router::create_routes();
}