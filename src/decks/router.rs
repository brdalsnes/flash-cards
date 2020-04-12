use rocket;

use crate::connection;
use crate::decks::handler;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/decks",
            routes![
                handler::all,
                handler::get,
                handler::post,
                handler::put,
                handler::delete
            ],
    ).launch();
}