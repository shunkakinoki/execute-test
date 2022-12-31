use clap::Parser;
use ethers::{providers::Middleware, types::NameOrAddress};
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
        help_heading = "From address"
    )]
    pub from: NameOrAddress,
}

impl NodeArgs {
    pub async fn into_node_config(self) -> NodeConfig {
        let provider = try_get_http_provider(self.url).unwrap();

        let address = match self.from {
            NameOrAddress::Name(name) => provider.resolve_name(&name).await.unwrap(),
            NameOrAddress::Address(address) => address,
        };

        NodeConfig::default().with_from(Some(address))
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
