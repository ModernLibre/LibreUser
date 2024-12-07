
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;



use crate::jwt;

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
