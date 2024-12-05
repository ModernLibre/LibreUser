use diesel::table;

table! {
    // user table
    user (uid) {
        id -> Int4,
        uid -> Uuid,
        username -> Varchar,
        avatar -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        admin -> Bool,
    }
}
