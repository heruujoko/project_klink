// @generated automatically by Diesel CLI.

diesel::table! {
    vehicles (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}
