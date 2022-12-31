mod config;
use anyhow::{Ok, Result};
pub use config::NodeConfig;
use ethers::types::{H160, U256};
use foundry_evm::executor::{fork::CreateFork, opts::EvmOpts, Backend, Executor, ExecutorBuilder};
use std::str::FromStr;

pub async fn spawn(config: NodeConfig) -> Executor {
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
    let token_res = executor
        .call_raw_committing(
            H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
            H160::from_str("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
            hex::decode("a9059cbb000000000000000000000000225e9b54f41f44f42150b6aaa730da5f2d23faf2000000000000000000000000000000000000000000000000000000003b9aca00").expect("valid").into(),
            U256::zero(),
        )
        .unwrap();
    println!("{:?}", token_res);

    Ok(())
}
