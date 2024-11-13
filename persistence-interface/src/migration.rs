use std::fmt::{Display, Formatter};
use std::future::Future;
use thiserror::Error;

pub trait Migration {
    fn migrate(&self, path: &str) -> impl Future<Output = Result<u64, MigrationError>>;
}

#[derive(Debug, Error)]
pub enum MigrationError {
    MigrationFileReadError(std::io::Error),
    MigrationExecutionError(sqlx::Error),
}

impl Display for MigrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            MigrationError::MigrationFileReadError(e) => format!("MigrationFileReadError({})", e),
            MigrationError::MigrationExecutionError(e) => format!("MigrationExecutionError({})", e),
        };
        write!(f, "{}", s)
    }
}

impl<T> From<MigrationError> for Result<T, MigrationError> {
    fn from(value: MigrationError) -> Self {
        Err(value)
    }
}
