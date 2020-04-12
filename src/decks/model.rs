use crate::schema::decks;
use serde_derive::{Serialize, Deserialize};
use serde_json::{Value};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "decks"]
pub struct Deck {
    pub id: i32,
    pub name: String,
    pub tags: Vec<String>,
    pub cards: Value,
}

#[derive(Insertable, Deserialize)]
#[table_name = "decks"]
pub struct InsertableDeck {
    pub name: String,
    pub tags: Vec<String>,
    pub cards: Value,
}