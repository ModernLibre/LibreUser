use crate::error::ServiceError;
use actix_web::web;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;

pub type PgPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

pub mod error;
mod schema;
pub use schema::user;

pub async fn get_conn(
    pool: web::Data<PgPool>,
) -> Result<PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>, ServiceError> {
    Ok(web::block(move || pool.get()).await??)
}
