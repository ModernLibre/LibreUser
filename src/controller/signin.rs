use std::borrow::Cow;

use actix_web::{
    dev::ServiceRequest, get, middleware, post, put, web, HttpMessage, HttpRequest, HttpResponse,
    Responder, ResponseError,
};
use actix_web_httpauth::middleware::HttpAuthentication;

use diesel::prelude::*;
use diesel::{query_source, ExpressionMethods};

use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{database, models};

pub async fn signin(
    req: HttpRequest,
    pool: web::Data<database::PostgresPool>,
    param: web::Json<models::User>,
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let request_user = ext
        .get::<models::User>()
        .ok_or(actix_web::error::ErrorUnauthorized(
            "User not authenticated",
        ))?;
    if !request_user.admin {
        return Err(actix_web::error::ErrorUnauthorized("User is not an admin"));
    }
    let mut conn = pool.get().await?;

    let query_result: Result<models::User, diesel::result::Error> = models::user::dsl::user
        .filter(models::user::dsl::login.eq(param.login.to_owned()))
        .first(&mut conn)
        .await;

    let user = query_result.map_err(|err| match err {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("User not found"),
        _ => actix_web::error::ErrorInternalServerError(err),
    })?;

    Ok(HttpResponse::Ok().json(user))
}
