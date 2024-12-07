use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, Selectable};
use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

mod schema;
pub use schema::user;

#[derive(Default, Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Clone)]
#[diesel(table_name = user)]
pub struct User {
    pub uid: uuid::Uuid,
    pub login: String,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub admin: bool,
    pub github_id: Option<i64>,
}

impl User {
    pub async fn create(
        &self,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<Self, Error> {
        diesel::insert_into(user::table)
            .values(self)
            .get_result(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn find_by_login(
        login: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<Self, Error> {
        user::table
            .filter(user::login.eq(login))
            .first(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn find_by_github_id(
        github_id: u64,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<Self, Error> {
        user::table
            .filter(user::github_id.eq(github_id as i64))
            .first(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn update_login(
        login: &str,
        new_login: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<usize, Error> {
        let existing_user: Result<User, diesel::result::Error> = user::dsl::user
            .filter(user::dsl::login.eq(login.to_owned()))
            .first(conn)
            .await;

        if existing_user.is_ok() {
            return Err(Error::Conflict);
        }

        diesel::update(user::table.filter(user::login.eq(login)))
            .set(user::login.eq(new_login))
            .execute(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn update_username(
        login: &str,
        new_username: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<usize, Error> {
        diesel::update(user::table.filter(user::login.eq(login)))
            .set(user::name.eq(new_username))
            .execute(conn)
            .await
            .map_err(Error::from)
    }

    pub async fn update_email(
        login: &str,
        new_email: &str,
        conn: &mut PooledConnection<'_, AsyncPgConnection>,
    ) -> Result<usize, Error> {
        diesel::update(user::table.filter(user::login.eq(login)))
            .set(user::email.eq(new_email))
            .execute(conn)
            .await
            .map_err(Error::from)
    }
}

pub enum Error {
    NotFound,
    InternalServerError,
    Conflict,
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Error::NotFound,
            _ => Error::InternalServerError,
        }
    }
}

impl From<Error> for actix_web::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => actix_web::error::ErrorNotFound("User not found"),
            Error::InternalServerError => {
                actix_web::error::ErrorInternalServerError("Internal server error")
            }
            Error::Conflict => actix_web::error::ErrorConflict("Conflict"),
        }
    }
}
