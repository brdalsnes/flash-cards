table! {
    decks (id) {
        id -> Int4,
        name -> Varchar,
        tags -> Array<Text>,
        cards -> Jsonb,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    decks,
    users,
);
