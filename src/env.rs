/// 根据本地/集群环境加载不同的环境变量
pub fn load_env() {
    // 检查环境变量中的 KUBERNETES_SERVICE 标志位
    let is_kubernetes =
        std::env::var("KUBERNETES_SERVICE").unwrap_or_else(|_| "false".to_string()) == "true";

    // 如果不在 Kubernetes 集群中，则加载 .env 文件，否则默认使用 ConfigMap 和 Secret 注入的环境变量
    if !is_kubernetes {
        if dotenv::dotenv().is_err() {
            println!("Failed to read .env file");
        } else {
            println!(".env file loaded successfully");
        }
    }

    // 设置日志级别
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
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

lazy_static! {
    pub static ref LIBRE_JWT_ALGORITHM: jsonwebtoken::Algorithm = env_jwt_algorithm();
}
