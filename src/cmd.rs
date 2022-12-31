use clap::Parser;
use ethers::{
    providers::Middleware,
    types::{NameOrAddress, H160},
};
use foundry_cli::opts::cast::parse_name_or_address;
use foundry_common::try_get_http_provider;
use foundry_simulator::{simulate, spawn, NodeConfig};

#[derive(Clone, Debug, Parser)]
pub struct NodeArgs {
    #[clap(
        long,
        short,
        visible_alias = "url",
        default_value = "https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9",
        value_name = "URL",
        help_heading = "Fork config"
    )]
    pub url: String,
    #[clap(
        long,
        short,
        visible_alias = "from",
        value_parser = parse_name_or_address,
        value_name = "FROM",
        help_heading = "From address or name"
    )]
    pub from: NameOrAddress,
    #[clap(
        long,
        short,
        visible_alias = "to",
        value_parser = parse_name_or_address,
        value_name = "TO",
        help_heading = "To address or name"
    )]
    pub to: NameOrAddress,
    #[clap(
        long,
        short,
        default_value = "",
        visible_alias = "data",
        value_name = "CALLDATA",
        help_heading = "Calldata"
    )]
    pub calldata: String,
}

/// Code from: https://github.com/foundry-rs/foundry/blob/master/cli/src/cmd/cast/storage.rs#L77
pub async fn get_address(url: &String, addr: &NameOrAddress) -> H160 {
    let provider = try_get_http_provider(url).unwrap();
    let address = match addr {
        NameOrAddress::Name(name) => provider.resolve_name(&name).await.unwrap(),
        NameOrAddress::Address(address) => address.clone(),
    };
    return address;
}

impl NodeArgs {
    pub async fn into_node_config(self) -> NodeConfig {
        let from = get_address(&self.url, &self.from).await;
        let to = get_address(&self.url, &self.to).await;
        NodeConfig { url: self.url, calldata: self.calldata, from, to }
    }
}

impl NodeArgs {
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.into_node_config().await;
        let s = spawn(&config);
        simulate(s, &config).await?;

        Ok(())
    }
}
