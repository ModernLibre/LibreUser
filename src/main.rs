use actix_web::{App, HttpServer};
use libre_user::{controller, database, jwt, oauth};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .configure(database::init)
        .configure(jwt::init)
        .configure(oauth::init)
        .configure(controller::init_routes)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
