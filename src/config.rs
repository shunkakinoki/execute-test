use ethers::types::Address;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Fork of EVM Chain
    pub fork_url: String,
    /// from address
    pub from: Address,
    /// to address
    pub to: Address,
}

impl Default for NodeConfig {
    fn default() -> Self {
        let fork_url =
            String::from("https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9");
        let kaki = Address::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap();
        Self { fork_url, from: kaki, to: kaki }
    }
}

impl NodeConfig {
    /// Sets the from address
    #[must_use]
    pub fn with_from<U: Into<Address>>(mut self, from: Option<U>) -> Self {
        if let Some(from) = from {
            self.from = from.into();
        }
        self
    }

    /// Sets the to address
    #[must_use]
    pub fn with_to<U: Into<Address>>(mut self, to: Option<U>) -> Self {
        if let Some(to) = to {
            self.to = to.into();
        }
        self
    }
}
