use clap::Parser;
use foundry_simulator::{spawn, NodeConfig};

#[derive(Clone, Debug, Parser)]
pub struct NodeArgs {
    #[clap(
        long,
        short,
        help = "Port number to listen on.",
        default_value = "8000",
        value_name = "NUM"
    )]
    pub port: u16,
}

impl NodeArgs {
    pub fn into_node_config(self) -> NodeConfig {
        NodeConfig::default()
    }
}

impl NodeArgs {
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let s = spawn(self.into_node_config()).await;
        println!("{:?}", s.unwrap());
        Ok(())
    }
}
