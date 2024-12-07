use std::borrow::Cow;

use actix_web::{
    dev::ServiceRequest, get, middleware, post, put, web, HttpMessage, HttpRequest, HttpResponse,
    Responder, ResponseError,
};
use actix_web_httpauth::middleware::HttpAuthentication;

use diesel::prelude::*;
use diesel::{query_source, ExpressionMethods};

use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{database, models, jwt};

mod signin;
mod users;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let middleware = HttpAuthentication::bearer(jwt::validator);
    cfg.service(
        web::scope("/users")
            .wrap(middleware)
            .service(users::get_users)
            .service(users::get_user_with_login)
            .service(users::update_user),
    );
}
