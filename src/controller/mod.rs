use std::borrow::Cow;

use actix_web::{dev::ServiceRequest, get, middleware, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web_httpauth::middleware::HttpAuthentication;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{middleware::validator, models::User, database};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let middleware = HttpAuthentication::bearer(validator);
    cfg.service(
        web::scope("/users")
        .wrap(middleware)
        .service(get_users)
        .service(get_user_with_login)
        // .service(create_user)
        .service(update_user)
    );
}

async fn create_user() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("")]
async fn get_users(req: HttpRequest, pool: web::Data<database::PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let request_user = ext.get::<User>()
        .ok_or(actix_web::error::ErrorUnauthorized("User not authenticated"))?;
    if (!request_user.admin) {
        return Err(actix_web::error::ErrorUnauthorized("User is not an admin"));
    }
    let conn = pool.get()
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;


    Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Deserialize, PartialEq)]
struct UserLogin {
    login: Cow<'static, str>,
}

#[get("/{login}")]
async fn get_user_with_login(req: HttpRequest, pool: web::Data<database::PgPool>, param: web::Path<UserLogin>) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let _request_user = ext.get::<User>()
    .ok_or(actix_web::error::ErrorUnauthorized("User not found"))?;

    // if request_user.login != login.login && !request_user.admin {
    //     return Err(actix_web::error::ErrorForbidden("User is not an admin"));
    // }

    let mut conn = pool.get()
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let query_result = web::block(move || {
        use crate::database::user::dsl;
        use diesel::QueryDsl;
        dsl::user.filter(dsl::login.eq(param.login.to_owned()))
        .get_result::<User>(&mut conn)
    }).await?;

    let user = query_result
    .map_err(|err| match err {
        diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("User not found"),
        _ => actix_web::error::ErrorInternalServerError(err)
    })?;

    return Ok(HttpResponse::Ok().json(user));
}

#[derive(serde::Deserialize)]
struct UpdateBuilder {
    pub username: Option<String>,   // only can be modified by itself and admin
    pub login: Option<String>,      // TODO: add last login modify day varification to prevent user from changing login too often
    pub email: Option<String>,      // TODO: add email verification, only can modified by itself
    pub admin: Option<bool>,        // always need admin
}

#[put("/{login}")]
async fn update_user(
    req: HttpRequest, 
    pool: web::Data<database::PgPool>, 
    param: web::Path<UserLogin>,
    update: web::Json<UpdateBuilder>
) -> Result<HttpResponse, actix_web::Error> {
    let ext = req.extensions();
    let request_user = ext.get::<User>()
    .ok_or(actix_web::error::ErrorUnauthorized("User not found"))?;

    let is_itself = request_user.login == param.login;
    let is_admin = request_user.admin;

    // if request_user.login != param.login && !request_user.admin {
    //     return Err(actix_web::error::ErrorForbidden("User is not an admin"));
    // }

    let deny = 
        update.username.is_some() && !is_itself && !is_admin ||
        update.login.is_some() && !is_itself && !is_admin ||
        update.email.is_some() && !is_itself && !is_admin ||
        update.admin.is_some() && !is_admin;
    
    if deny {
        return Err(actix_web::error::ErrorForbidden("Operation not allowed"));
    }

    // update those...

    let pol1 = (**pool).clone();
    let mut conn = pool.get()
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // let query_result: Result<usize, diesel::result::Error> = web::block(move || {
    //     use crate::database::user::dsl;
    //     use diesel::QueryDsl;
    //     use diesel::RunQueryDsl;
    //     diesel::update(dsl::user.filter(dsl::login.eq(param.login.to_owned())))
    //         .set((
    //             dsl::username.eq(update.username.to_owned()),
    //             dsl::email.eq(update.email.to_owned()),
    //             dsl::admin.eq(update.admin),
    //         ))
    //         .execute(&mut conn)
    // }).await?;

    // let user = query_result
    // .map_err(|err| match err {
    //     diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("User not found"),
    //     _ => actix_web::error::ErrorInternalServerError(err)
    // })?;

    // return Ok(HttpResponse::Ok().json(user));
    todo!()
}

async fn update_username<'a>(
    login: &'a str,
    new_username: &'a str,
) {
    use crate::database::user::dsl;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    todo!()
}
