use clap::Parser;
use foundry_simulator::{simulate, spawn, NodeConfig};

#[derive(Clone, Debug, Parser)]
pub struct NodeArgs {
    #[clap(long, short, visible_alias = "url", value_name = "URL", help_heading = "Fork config")]
    pub fork_url: Option<String>,
}

impl NodeArgs {
    pub fn into_node_config(self) -> NodeConfig {
        NodeConfig::default()
    }
}

impl NodeArgs {
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let s = spawn(self.into_node_config());
        simulate(s).await?;
        Ok(())
    }
}
