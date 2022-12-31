#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Fork of EVM Chain
    pub fork_url: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        let fork_url =
            String::from("https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9");
        Self { fork_url }
    }
}
