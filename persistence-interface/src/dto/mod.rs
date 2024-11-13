use crate::dto::migration::Migration;
use crate::dto::server_configuration::ServerConfigurationRepository;

pub mod migration;
pub mod server_configuration;

pub trait DatabaseOperations: ServerConfigurationRepository + Migration {}
