#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("error loading config")]
    Config(#[from] confy::ConfyError),

    #[error("database migration error")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("password hashing error: {0}")]
    Hashing(String), // argonautica::Error does not implement std::error::Error

    #[error("error parsing IP address")]
    AddrParse(#[from] std::net::AddrParseError),

    #[error("I/O error")]
    IO(#[from] std::io::Error),
}

impl From<argonautica::Error> for Error {
    fn from(value: argonautica::Error) -> Self {
        Self::Hashing(value.to_string())
    }
}
