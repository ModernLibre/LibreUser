use anyhow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use url::Url;

mod github;
mod error;
pub use error::Error;

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
