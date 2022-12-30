#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Chain ID of the EVM chain
    pub chain_id: Option<u64>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self { chain_id: None }
    }
}
