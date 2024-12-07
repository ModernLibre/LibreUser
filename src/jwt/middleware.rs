use actix_web::{dev::ServiceRequest, HttpMessage as _};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::jwt::JwtUtil;

// 认证中间件
pub(crate) async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::error::Error> {
    let jwt = req
        .app_data::<JwtUtil>()
        .expect("JwtUtil is not configured");
    match jwt.validate_jwt(&credentials.token()) {
        Ok(user) => {
            req.extensions_mut().insert(user.claims);
            Ok(req)
        }
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e.to_string())),
    }
}
