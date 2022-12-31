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
    /// Returns a new config intended to be used in tests, which does not print
    /// and binds to a random, free port by setting it to `0`
    #[doc(hidden)]
    pub fn test() -> Self {
        Self { ..Default::default() }
    }
}

impl NodeConfig {
    #[must_use]
    pub fn with_value<U: Into<String>>(mut self, value: Option<U>) -> Self {
        if let Some(value) = value {
            self.value = value.into();
        }
        self
    }
}
