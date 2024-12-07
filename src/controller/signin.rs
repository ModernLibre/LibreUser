
// use actix_web::{
//     web, HttpMessage, HttpRequest, HttpResponse,
// };

// use diesel::prelude::*;
// use diesel::ExpressionMethods;

// use diesel_async::RunQueryDsl;

// use crate::{database, models};

// pub async fn signin(
//     req: HttpRequest,
//     pool: web::Data<database::PostgresPool>,
//     param: web::Json<models::User>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let ext = req.extensions();
//     let request_user = ext
//         .get::<models::User>()
//         .ok_or(actix_web::error::ErrorUnauthorized(
//             "User not authenticated",
//         ))?;
//     if !request_user.admin {
//         return Err(actix_web::error::ErrorUnauthorized("User is not an admin"));
//     }
//     let mut conn = pool.get().await?;

//     let query_result: Result<models::User, diesel::result::Error> = models::user::dsl::user
//         .filter(models::user::dsl::login.eq(param.login.to_owned()))
//         .first(&mut conn)
//         .await;

//     let user = query_result.map_err(|err| match err {
//         diesel::result::Error::NotFound => actix_web::error::ErrorNotFound("User not found"),
//         _ => actix_web::error::ErrorInternalServerError(err),
//     })?;

//     Ok(HttpResponse::Ok().json(user))
// }
