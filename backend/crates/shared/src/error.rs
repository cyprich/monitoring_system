use std::fmt::Display;

use crate::UNKNOWN;

#[derive(Debug)]
pub enum Error {
    Env(dotenvy::Error),
    IoGeneral(std::io::Error),
    IoNotFound(std::io::Error),
    Reqwest(reqwest::Error),
    ReqwestFromString(String),
    HTTPResponse(u16),
    ParseInt(std::num::ParseIntError),
    DbGeneral(sqlx::Error),
    DbForeignKey(sqlx::Error),
    DbPool(sqlx::Error),
    DbMigration(Box<sqlx::migrate::MigrateError>),
    DbConfig(String),
    DbNothingChanged,
    TomlSer(toml::ser::Error),
    TomlDe(toml::de::Error),
    UnsupportedSystem,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Env(error) => {
                let msg = "Environment variable error:";
                match error.not_found() {
                    true => write!(f, "{msg} variable {error} not found",),
                    false => write!(f, "{msg} {error}"),
                }
            }
            Error::IoGeneral(error) => {
                write!(f, "General IO Error: {} - {}", error.kind(), error)
            }
            Error::IoNotFound(error) => {
                write!(f, "IO Error: Not found - {}", error)
            }
            Error::Reqwest(error) => {
                write!(
                    f,
                    "Reqwest error, URL: {}, reponse code: {}, error: {}",
                    error.url().map(|u| u.as_str()).unwrap_or(UNKNOWN),
                    error
                        .status()
                        .map(|s| s.as_u16().to_string())
                        .unwrap_or(UNKNOWN.to_string()),
                    error
                )
            }
            Error::ReqwestFromString(error) => {
                write!(f, "Reqwest error: {}", error)
            }
            Error::HTTPResponse(error) => write!(f, "Error - HTTP Response code: {}", error),
            Error::ParseInt(error) => write!(f, "Error parsing integer: {}", error),
            Error::DbGeneral(error) => write!(f, "Unexpected Database error: {}", error),
            Error::DbForeignKey(error) => {
                write!(f, "Database error - foreign key missing: {}", error)
            }
            Error::DbPool(error) => write!(f, "Database error - pool error: {}", error),
            Error::DbMigration(error) => write!(f, "Database error - migrations: {}", error),
            Error::DbConfig(error) => write!(f, "Database error - config: {}", error),
            Error::DbNothingChanged => write!(f, "Database error - nothing changed"),
            Error::TomlSer(error) => write!(f, "Error serializing TOML: {}", error),
            Error::TomlDe(error) => write!(f, "Error deserializing TOML: {}", error),
            Error::UnsupportedSystem => write!(f, "System not supported"),
        }
    }
}

impl From<dotenvy::Error> for Error {
    fn from(value: dotenvy::Error) -> Self {
        Error::Env(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => Error::IoNotFound(value),
            _ => Error::IoGeneral(value),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

// HTTPResponse - not necessary, it's just `u16`

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(ref val) => {
                if let Some(val) = val.code() {
                    return match val.as_ref() {
                        "23503" => Error::DbForeignKey(value),
                        _ => Error::DbGeneral(value),
                    };
                }
                Error::DbGeneral(value)
            }
            sqlx::Error::Configuration(val) => Error::DbConfig(val.to_string()),
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => Error::DbPool(value),
            sqlx::Error::Migrate(val) => Error::DbMigration(val),
            _ => Error::DbGeneral(value),
        }
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Error::TomlSer(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::TomlDe(value)
    }
}
