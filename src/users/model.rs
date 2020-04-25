use crate::schema::users;
use serde_derive::{Serialize, Deserialize};
use chrono;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct InputUser {
    pub username: String,
    pub email: String,
}