use serde::Deserialize;

/// Configuration for blockchain connection.
///
/// Contains settings required to connect to a blockchain node and interact with
/// the blockchain network.
#[derive(Deserialize, Debug)]
pub struct BlockchainConfig {
    /// URL endpoint for the blockchain's RPC service.
    pub rpc_url: String,

    /// Identifier for the blockchain network.
    pub chain_id: u64,
}
