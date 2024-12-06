use diesel::table;

table! {
    // user table
    user (uid) {
        uid -> Uuid,
        login -> Varchar,
        username -> Varchar,
        avatar -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        admin -> Bool,
    }
}
