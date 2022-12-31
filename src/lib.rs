mod config;
use anyhow::{Ok, Result};
use bytes::Bytes;
pub use config::NodeConfig;
use ethers::types::{H160, U256};
use foundry_evm::executor::{fork::CreateFork, opts::EvmOpts, Backend, Executor, ExecutorBuilder};
use std::str::FromStr;

pub fn spawn(config: NodeConfig) -> Executor {
    let evm_opts = EvmOpts { fork_url: Some(config.fork_url.clone()), ..Default::default() };

    let env = evm_opts.evm_env_blocking().unwrap();
    let builder = ExecutorBuilder::default().set_tracing(true);

    let fork_opts =
        Some(CreateFork { url: config.fork_url.clone(), enable_caching: true, env, evm_opts });
    let db = Backend::spawn(fork_opts);
    let executor = builder.build(db);

    return executor;
}

pub async fn simulate(mut executor: Executor) -> Result<()> {
    Ok(())
}
