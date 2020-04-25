use rocket;
use crate::users::handler;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        handler::all,
        handler::get,
        handler::post,
        handler::delete,
    ]
}