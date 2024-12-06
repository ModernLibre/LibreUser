use actix_web::web;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};

lazy_static! {
    pub static ref LIBRE_JWT_ALGORITHM: jsonwebtoken::Algorithm = env_jwt_algorithm();
}

pub(super) struct JwtUtil {
    public_key: DecodingKey,
    private_key: EncodingKey,
    algorithm: Algorithm,
}

// const DEFAULT_PUBLIC_KEY_PATH: &str = "./key/publickey.pem";
// const DEFAULT_PRIVATE_KEY_PATH: &str = "./key/privatekey.pem";

pub fn config_jwt_middleware(cfg: &mut web::ServiceConfig) {
    let jwt_util = env_jwt_util();
    cfg.app_data(web::Data::new(jwt_util)); // todo...
}

fn env_jwt_util() -> JwtUtil {
    let algorithm = env_jwt_algorithm();
    let pubkey_path =
        std::env::var("JWT_PUBLIC_KEY_PATH").expect("JWT_PUBLIC_KEY_PATH must be set");
    let privkey_path =
        std::env::var("JWT_PRIVATE_KEY_PATH").expect("JWT_PRIVATE_KEY_PATH must be set");

    let pub_file = std::fs::read(pubkey_path).expect("Failed to read public key file");
    let priv_file = std::fs::read(privkey_path).expect("Failed to read private key file");

    let public_key = 
        DecodingKey::from_rsa_pem(&pub_file).expect("Failed to parse public key file");
    let private_key =
        EncodingKey::from_rsa_pem(&priv_file).expect("Failed to parse private key file");

    return JwtUtil {
        public_key,
        private_key,
        algorithm,
    };
}

fn env_jwt_algorithm() -> jsonwebtoken::Algorithm {
    let alg = std::env::var("LIBRE_JWT_ALGORITHM")
        .unwrap_or_else(|_| "RS256".to_string())
        .parse()
        .expect("Failed to parse LIBRE_JWT_ALGORITHM");

    match alg {
        jsonwebtoken::Algorithm::RS256
        | jsonwebtoken::Algorithm::RS384
        | jsonwebtoken::Algorithm::RS512 => alg,
        _ => panic!("LIBRE_JWT_ALGORITHM must be an RSA algorithm (RS256, RS384, RS512)"),
    }
}
