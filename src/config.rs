use ethers::types::Address;
use std::str::FromStr;

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
    /// value
    pub value: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            url: "https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9".to_string(),
            from: Address::zero(),
            to: Address::zero(),
            calldata: "".to_string(),
            value: "0".to_string(),
        }
    }
}

impl NodeConfig {
    /// Code from: https://github.com/foundry-rs/foundry/blob/master/anvil/src/config.rs#L320
    #[doc(hidden)]
    pub fn test() -> Self {
        Self { ..Default::default() }
    }
}

impl NodeConfig {
    pub fn with_from(mut self, from: String) -> Self {
        self.from = Address::from_str(&from).unwrap();
        self
    }
    pub fn with_to(mut self, to: String) -> Self {
        self.to = Address::from_str(&to).unwrap();
        self
    }
    pub fn with_value(mut self, value: String) -> Self {
        self.value = value;
        self
    }
    pub fn with_calldata(mut self, calldata: String) -> Self {
        self.calldata = calldata;
        self
    }
}
