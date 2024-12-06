use actix_web::{get, post, put, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;

#[post("/users")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/users/{login}")]
async fn get_user_with_login() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[put("/users/{login}")]
async fn update_user() -> impl Responder {
    HttpResponse::Ok().finish()
}

