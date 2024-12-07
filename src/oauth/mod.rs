use actix_web::web;
use oauth2::{
    AuthorizationCode, CsrfToken,
};
use serde::Deserialize;

mod error;
mod github;
pub use error::Error;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/oauth")
        .configure(github::github_config)
    );
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

#[derive(Deserialize, Debug)]
pub struct BaseOauthUser {
    pub id: String,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
}

impl From<github::GitHubUser> for BaseOauthUser {
    fn from(user: github::GitHubUser) -> Self {
        Self {
            id: user.id.to_string(),
            login: user.login,
            name: user.name,
            email: user.email,
            avatar_url: user.avatar_url.to_string(),
        }
    }
}
