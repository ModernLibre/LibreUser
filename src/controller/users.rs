use std::borrow::Cow;

use actix_web::{
    get, put, web, HttpMessage, HttpRequest, HttpResponse,
};

use diesel::prelude::*;
use diesel::ExpressionMethods;

use diesel_async::RunQueryDsl;

use crate::jwt;
use crate::{database, models};

#[get("")]
async fn get_users(
    req: HttpRequest,
    pool: web::Data<database::PostgresPool>,
) -> Result<HttpResponse, actix_web::Error> {
    {
        let ext = req.extensions();
        let request_user = ext
            .get::<jwt::Claims>()
            .ok_or(actix_web::error::ErrorUnauthorized(
                "User not authenticated",
            ))?;
        if !request_user.admin {
            return Err(actix_web::error::ErrorUnauthorized("User is not an admin"));
        }
    }

    let mut conn = pool.get().await?;

    let users = models::user::dsl::user
        .load::<models::User>(&mut conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[derive(serde::Deserialize, PartialEq)]
struct UserLogin {
    login: Cow<'static, str>,
}

/// Get user by login
///
/// ## Parameters
/// - `login`: The login of the user to retrieve (in path, required)
///
/// ## Security
/// - `bearerAuth`: Bearer authentication required
///
/// ## Responses
/// - `200`: Successful operation
/// - `401`: Unauthorized
/// - `404`: User not found
#[get("/{login}")]
async fn get_user_with_login(
    // req: HttpRequest,
    pool: web::Data<database::PostgresPool>,
    param: web::Path<UserLogin>,
) -> Result<HttpResponse, actix_web::Error> {
    // let ext = req.extensions();
    // let _request_user = ext
    //     .get::<jwt::Claims>()
    //     .ok_or(actix_web::error::ErrorUnauthorized("User not found"))?;

    let mut conn = pool.get().await?;

    let query_result: Result<Vec<models::User>, diesel::result::Error> = models::user::dsl::user
        .filter(models::user::dsl::login.eq(&param.login))
        .load(&mut conn)
        .await;

    let user = query_result.map_err(|err| match err {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("User not found"),
        _ => actix_web::error::ErrorInternalServerError(err),
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[derive(serde::Deserialize)]
struct UpdateBuilder {
    pub username: Option<String>, // only can be modified by itself and admin
    pub login: Option<String>, // TODO: add last login modify day varification to prevent user from changing login too often
    pub email: Option<String>, // TODO: add email verification, only can modified by itself
    pub admin: Option<bool>,   // always need admin
}

#[put("/{login}")]
async fn update_user(
    req: HttpRequest,
    pool: web::Data<database::PostgresPool>,
    param: web::Path<UserLogin>,
    update: web::Json<UpdateBuilder>,
) -> Result<HttpResponse, actix_web::Error> {
    let (is_itself, is_admin) = {
        let ext = req.extensions();
        let request_user = ext
            .get::<jwt::Claims>()
            .ok_or(actix_web::error::ErrorUnauthorized("User not found"))?;

        (request_user.login == param.login, request_user.admin)
    };

    let param_login: &str = &param.login;

    let deny = update.username.is_some() && !is_itself && !is_admin
        || update.login.is_some() && !is_itself && !is_admin
        || update.email.is_some() && !is_itself && !is_admin
        || update.admin.is_some() && !is_admin;

    if deny {
        return Err(actix_web::error::ErrorForbidden("Operation not allowed"));
    }

    let mut conn = pool.get().await?;

    if let Some(username) = &update.username {
        diesel::update(
            models::user::dsl::user.filter(models::user::dsl::login.eq(param_login)),
        )
        .set(models::user::dsl::name.eq(username))
        .execute(&mut conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    }

    if let Some(login) = &update.login {
        let existing_user: Result<models::User, diesel::result::Error> = models::user::dsl::user
            .filter(models::user::dsl::login.eq(login))
            .first(&mut conn)
            .await;

        if existing_user.is_ok() {
            return Err(actix_web::error::ErrorConflict("Login already exists"));
        }

        diesel::update(
            models::user::dsl::user.filter(models::user::dsl::login.eq(param_login)),
        )
        .set(models::user::dsl::login.eq(&login))
        .execute(&mut conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    }

    if let Some(email) = &update.email {
        diesel::update(
            models::user::dsl::user.filter(models::user::dsl::login.eq(param_login)),
        )
        .set(models::user::dsl::email.eq(email))
        .execute(&mut conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    }

    if let Some(admin) = update.admin {
        diesel::update(
            models::user::dsl::user.filter(models::user::dsl::login.eq(param_login)),
        )
        .set(models::user::dsl::admin.eq(admin))
        .execute(&mut conn)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    }

    Ok(HttpResponse::Ok().finish())
}
