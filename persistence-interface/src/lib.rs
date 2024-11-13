use migration::Migration;
use crate::repository::server_configuration_repository::ServerConfigurationRepository;

pub mod dto;
pub mod repository;
pub mod migration;

pub trait DatabaseOperations: ServerConfigurationRepository + Migration + Clone {}
