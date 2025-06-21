use alloy::primitives::Address;
use serde::Deserialize;
use tracing::info;

use crate::domain::model::Source;

use super::{blockchain::BlockchainConfig, environment::Environment, server::ServerConfig};

/// Root configuration for the application.
///
/// Combines all sub-configurations into a single structure for centralized
/// configuration management.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// The name of the application. Will appear in telemetry.
    pub application_name: String,

    /// The token for which supply information is retrieved.
    pub token: Address,

    /// Server-related configuration settings.
    pub server: ServerConfig,

    /// Blockchain connection configuration.
    pub blockchain: BlockchainConfig,

    /// The sources to exclude from circulating supply.
    pub excluded_sources: Vec<Source>,
}

impl Config {
    /// Loads application configuration from files and environment variables.
    ///
    /// The configuration is loaded in the following order, with later sources
    /// overriding earlier ones:
    ///
    /// 1. File (`configuration/[environment].yaml`)
    /// 2. Environment variables with prefix `APP_`
    ///
    /// # Returns
    ///
    /// The loaded configuration or an error.
    pub fn load() -> Result<Config, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let config_dir = base_path.join("configuration");

        // Determine environment from APP_ENVIRONMENT or default to "local".
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        info!("Building configuration for {}", environment.as_str());

        let config_filename = format!("{}.yaml", environment.as_str());

        // Build configuration from multiple sources.
        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join(config_filename)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        settings.try_deserialize::<Config>()
    }
}
