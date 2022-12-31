use anyhow::{Ok, Result};
use bytes::Bytes;
use clap::Parser;
use ethers::types::{H160, U256};
use foundry_evm::executor::{fork::CreateFork, opts::EvmOpts, Backend, ExecutorBuilder};
use foundry_simulator::{simulate, spawn, NodeConfig};
use std::{str::FromStr, string::String};

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
    pub async fn run(self) -> Result<()> {
        let fork_url =
            String::from("https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9");
        let gas_limit: u64 = 18446744073709551615;

        let evm_opts = EvmOpts { fork_url: Some(fork_url.clone()), ..Default::default() };

        let env = evm_opts.evm_env_blocking().unwrap();

        let fork_opts = Some(CreateFork { url: fork_url, enable_caching: true, env, evm_opts });

        let db = Backend::spawn(fork_opts);

        let builder = ExecutorBuilder::default().with_gas_limit(gas_limit.into()).set_tracing(true);

        let mut executor = builder.build(db);

        let account_bal = executor
            .get_balance(H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap());

        println!("Balance before: {:#?}", account_bal.unwrap());

        let res = executor
            .call_raw_committing(
                H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
                H160::from_str("0x225E9B54F41F44F42150b6aAA730Da5f2d23FAf2").unwrap(),
                Bytes::from(""),
                U256::from(300_000_000),
            )
            .unwrap();

        let account_bal = executor
            .get_balance(H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap());

        println!("Balance after: {:#?}", account_bal.unwrap());
        println!("Gas used: {:#?}", res.gas_used);

        Ok(())
    }
}
