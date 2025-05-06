use serde::Deserialize;

/// Configuration for the HTTP server.
///
/// Contains settings related to the API server's network configuration and behavior.
#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    /// Hostname or IP address the server will bind to.
    pub host: String,

    /// TCP port the server will listen on.
    pub port: u16,

    /// Interval in seconds between token supply updates.
    ///
    /// Controls how frequently the background worker will fetch new supply
    /// data from the blockchain.
    pub update_interval: u64,
}
