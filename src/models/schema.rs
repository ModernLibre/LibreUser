use diesel::table;

table! {
    // user table
    user (uid) {
        uid -> Uuid,
        login -> Varchar,
        name -> Varchar,
        avatar -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        admin -> Bool,
        github_id -> Nullable<Int8>,
    }
}
