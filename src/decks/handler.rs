use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status;
use diesel::result::Error;

use crate::connection::DbConn;
use crate::decks::repository;
use crate::decks::model::{Deck, InsertableDeck};

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Deck>>, Status> {
    repository::all(&connection)
        .map(|decks| Json(decks))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Deck>, Status> {
    repository::get(id, &connection)
        .map(|deck| Json(deck))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<deck>")]
pub fn post(deck: Json<InsertableDeck>, connection: DbConn) -> Result<status::Created<Json<Deck>>, Status> {
    repository::insert(deck.into_inner(), &connection)
        .map(|deck| status::Created("deck".to_string(), Some(Json(deck))))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<deck>")]
pub fn put(id: i32, deck: Json<Deck>, connection: DbConn) -> Result<Json<Deck>, Status> {
    repository::update(id, deck.into_inner(), &connection)
        .map(|deck| Json(deck))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match repository::get(id, &connection) {
        Ok(_) => repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}

#[patch("/<id>/name", format = "plain", data = "<name>")]
pub fn patch_name(id: i32, name: String, connection: DbConn) -> Result<Json<Deck>, Status> {
    repository::update_name(id, name, &connection)
        .map(|deck| Json(deck))
        .map_err(|error| error_status(error))
}