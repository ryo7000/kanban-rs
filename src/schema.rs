table! {
    boards (id) {
        id -> Int8,
        name -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    cards (id) {
        id -> Int8,
        board_id -> Int8,
        description -> Text,
        status -> Status_enum,
        created_at -> Timestamptz,
    }
}

joinable!(cards -> boards (board_id));

allow_tables_to_appear_in_same_query!(
    boards,
    cards,
);
