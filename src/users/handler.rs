use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status;

use crate::connection::DbConn;
use crate::users::repository;
use crate::users::model::{User, InsertableUser, InputUser};
use crate::utils::error_helpers::error_status;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<User>>, Status> {
    repository::all(&connection)
        .map(|users| Json(users))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<User>, Status> {
    repository::get(id, &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<user>")]
pub fn post(user: Json<InputUser>, connection: DbConn) -> Result<status::Created<Json<User>>, Status> {
    let input_user = user.into_inner();
    let insertable_user: InsertableUser = InsertableUser {
        username: &input_user.username,
        email: &input_user.email,
        created_at: chrono::Local::now().naive_local(),
    };
    repository::insert(insertable_user, &connection)
        .map(|user| status::Created("user".to_string(), Some(Json(user))))
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