use chrono::TimeDelta;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::models::User;

mod error;
mod init;
pub use error::Error;
pub use init::config_jwt_middleware as init;
mod middleware;
pub(crate) use middleware::validator;

pub struct JwtUtil {
    pub public_key: DecodingKey,
    pub private_key: EncodingKey,
    pub algorithm: Algorithm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Issuer
    pub iss: String,
    /// Expiration time
    pub exp: u64,
    /// Subject, also user ID
    pub sub: String,
    /// Audience
    pub aud: String,
    /// Issued at
    pub iat: u64,
    /// JWT ID
    pub jti: String,
    /// Login
    pub login: String,
    /// User name
    pub name: String,
    /// Permissions
    pub admin: bool,
    // ...other fields...
}

impl From<&User> for Claims {
    fn from(user: &User) -> Self {
        let iat = jsonwebtoken::get_current_timestamp();
        Self {
            iss: "libre".to_string(),
            exp: iat + 3600,
            sub: user.uid.to_string(),
            aud: "libre".to_string(),
            iat,
            jti: user.uid.to_string(),
            login: user.login.clone(),
            name: user.name.clone(),
            admin: user.admin,
        }
    }
}

impl Claims {
    pub fn expiration(mut self, duration: TimeDelta) -> Self {
        self.exp = self.iat + duration.num_seconds() as u64;
        self
    }

    pub fn user(mut self, user: &User) -> Self {
        self.sub = user.uid.to_string();
        self.jti = user.uid.to_string();
        self.name = user.name.clone();
        self.admin = user.admin;
        self
    }

    pub fn generate_jwt(&self, jwt_util: &JwtUtil) -> Result<String, jsonwebtoken::errors::Error> {
        jwt_util.generate_jwt(self)
    }
}

pub fn generate_key_pair(bits: usize) -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

impl JwtUtil {
    pub fn validate_jwt(
        &self,
        token: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &self.public_key, &Validation::new(self.algorithm))
    }

    pub fn generate_jwt(&self, claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::new(self.algorithm), claims, &self.private_key)
    }
}

#[cfg(test)]
mod tests;
