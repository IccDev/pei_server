// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        create_at -> Timestamptz,
    }
}
