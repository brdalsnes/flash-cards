use crate::diesel::{RunQueryDsl, QueryDsl};
use diesel::{PgConnection, QueryResult};
use crate::schema::users;
use crate::users::model::{User, InsertableUser};

pub fn all(connection: &PgConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(connection)
}

pub fn insert(user: InsertableUser, connection: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id))
        .execute(connection)
}