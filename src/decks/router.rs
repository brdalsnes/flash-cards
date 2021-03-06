use rocket;
use crate::decks::handler;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        handler::all,
        handler::get,
        handler::post,
        handler::put,
        handler::delete,
        handler::patch_name,
    ]
}