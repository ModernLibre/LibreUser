use actix_web::web;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use redis::aio::MultiplexedConnection;

pub mod error;

pub struct PostgresPool(pub Pool<AsyncPgConnection>);
pub struct RedisMultiplexClient(pub redis::Client);

impl PostgresPool {
    pub async fn get(&self) -> Result<PooledConnection<'_, AsyncPgConnection>, actix_web::Error> {
        self.0
            .get()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))
    }
}

impl RedisMultiplexClient {
    pub async fn get(&self) -> Result<MultiplexedConnection, actix_web::Error> {
        self.0
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    let pg_pool = actix_web::rt::System::new().block_on(init_postgres_pool());
    let redis_pool = init_redis_pool();
    cfg.app_data(pg_pool).app_data(redis_pool);
}

pub async fn init_postgres_pool() -> PostgresPool {
    // create a new connection pool with the default config
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set"),
    );
    let pool = actix_web::rt::System::new()
        .block_on(Pool::builder().build(config))
        .expect("Failed to create pool"); // Enhancement: IO error handling
    PostgresPool(pool)
}

pub fn init_redis_pool() -> RedisMultiplexClient {
    let client = redis::Client::open(std::env::var("REDIS_URL").expect("REDIS_URL must be set"))
        .expect("Failed to create redis client"); // Enhancement: IO error handling
    RedisMultiplexClient(client)
}
