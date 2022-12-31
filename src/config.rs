use ethers::types::Address;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Fork of EVM Chain
    pub url: String,
    /// from address
    pub from: Address,
    /// to address
    pub to: Address,
    /// calldata
    pub calldata: String,
}
