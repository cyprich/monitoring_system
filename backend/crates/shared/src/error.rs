use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    // TODO maybe something more specific (not found, bad request, ...)
    #[error("identifying")]
    Identify(),
    #[error("communication with api")]
    Api(),

    #[error("serializing collector config")]
    ConfigSerialize(#[from] toml::ser::Error),
    #[error("deserializing collector config")]
    ConfigDeserialize(#[from] toml::de::Error),
    #[error("loading collector config")]
    ConfigLoad,
    #[error("saving collector config")]
    ConfigSave(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database error")]
    Database(#[from] sqlx::Error),
    #[error("foreign key not found")]
    ForeignKey,
}
