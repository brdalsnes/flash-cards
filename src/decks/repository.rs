use crate::diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::{PgConnection, QueryResult};
use crate::schema::decks;
use crate::decks::model::{Deck, InsertableDeck};

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Deck>> {
    decks::table.load::<Deck>(connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Deck> {
    decks::table.find(id).get_result::<Deck>(connection)
}

pub fn insert(deck: InsertableDeck, connection: &PgConnection) -> QueryResult<Deck> {
    diesel::insert_into(decks::table)
        .values(&deck)
        .get_result(connection)
}

pub fn update(id: i32, deck: Deck, connection: &PgConnection) -> QueryResult<Deck> {
    diesel::update(decks::table.find(id))
        .set(&deck)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(decks::table.find(id))
        .execute(connection)
}

pub fn update_name(id: i32, name: String, connection: &PgConnection) -> QueryResult<Deck> {
    diesel::update(decks::table.find(id))
        .set(decks::name.eq(name))
        .get_result(connection)
}