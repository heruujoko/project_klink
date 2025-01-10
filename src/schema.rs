// @generated automatically by Diesel CLI.

diesel::table! {
    passengers (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    passengers,
    vehicles,
);
