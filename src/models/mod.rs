use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::database::user;

#[derive(Default, Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone)]
#[diesel(table_name = user)]
pub struct User {
    pub uid: uuid::Uuid,
    pub login: String,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub admin: bool,
}
