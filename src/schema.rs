table! {
    decks (id) {
        id -> Int4,
        name -> Varchar,
        tags -> Array<Text>,
        cards -> Jsonb,
    }
}
