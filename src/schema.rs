// @generated automatically by Diesel CLI.

diesel::table! {
    crates (id) {
        id -> Int4,
        rustacean_id -> Int4,
        code -> Varchar,
        name -> Varchar,
        version -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rustaceans (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(crates -> rustaceans (rustacean_id));

diesel::allow_tables_to_appear_in_same_query!(
    crates,
    rustaceans,
);
