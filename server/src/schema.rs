// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        content -> Varchar,
        complete -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
