use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("loading environment variables")]
    Env(#[from] EnvError),
    #[error("running api server")]
    Server(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("communication with api")]
    Api,

    #[error("serializing collector config")]
    ConfigSerialize(#[from] toml::ser::Error),
    #[error("deserializing collector config")]
    ConfigDeserialize(#[from] toml::de::Error),
    #[error("loading collector config")]
    ConfigLoad,
    #[error("saving collector config")]
    ConfigSave(#[from] std::io::Error),

    #[error("integer parsing")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("error with HTTP request")]
    ReqwestError(#[from] reqwest::Error),
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("general error")]
    General,

    #[error("system is not supported")]
    UnsupportedSystem,

    #[error("loading environment variables")]
    Env(#[from] EnvError),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("foreign key not found")]
    ForeignKey,
    #[error("loading environment variables")]
    Env(#[from] EnvError),
}

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("loading environment file")]
    Dotenv(#[from] dotenvy::Error),
}
