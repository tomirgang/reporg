use config::{Config, ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OidcClient {
    pub id: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OidcUrl {
    pub issuer: String,
    pub redirect: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Oidc {
    pub client: OidcClient,
    pub url: OidcUrl,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tester {
    pub key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FrontendLogin {
    pub success: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FrontendLogout {
    pub success: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Frontend {
    pub baseurl: String,
    pub login: FrontendLogin,
    pub logout: FrontendLogout,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cors {
    pub origin: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Members {
    pub admin: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: Database,
    pub oidc: Oidc,
    pub tester: Tester,
    pub log: Log,
    pub frontend: Frontend,
    pub cors: Cors,
    pub members: Members,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let s = Config::builder()
            .add_source(Environment::with_prefix("REPORG").separator("_").list_separator(" "))
            .build()?;

        s.try_deserialize()
    }
}
