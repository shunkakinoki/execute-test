mod config;
use anyhow::{Ok, Result};
pub use config::NodeConfig;
use ethers::{
    abi::Detokenize,
    types::{Log, H160, H256, U256},
};
use foundry_evm::executor::{fork::CreateFork, opts::EvmOpts, Backend, Executor, ExecutorBuilder};
use futures::future::join_all;
use std::str::FromStr;

pub fn spawn(config: NodeConfig) -> Executor {
    let gas_limit: u64 = 18446744073709551615;

    let evm_opts = EvmOpts { fork_url: Some(config.fork_url.clone()), ..Default::default() };

    let env = evm_opts.evm_env_blocking().unwrap();
    let builder = ExecutorBuilder::default().with_gas_limit(gas_limit.into()).set_tracing(true);

    let fork_opts =
        Some(CreateFork { url: config.fork_url.clone(), enable_caching: true, env, evm_opts });
    let db = Backend::spawn(fork_opts);
    let executor = builder.build(db);

    return executor;
}

pub async fn resolve_call_args<D: Detokenize>(args: &[String], executor: Executor) -> Vec<D> {
    join_all(args.iter().map(|arg| async {
        executor
            .call(
                H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
                H160::from_str("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
                arg.clone(),
                (),
                0.into(),
                None,
            )
            .unwrap()
            .result
    }))
    .await
}

pub async fn simulate(mut executor: Executor) -> Result<()> {
    let res = executor
        .call_raw_committing(
            H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
            H160::from_str("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
            hex::decode("a9059cbb000000000000000000000000225e9b54f41f44f42150b6aaa730da5f2d23faf2000000000000000000000000000000000000000000000000000000003b9aca00").expect("valid").into(),
            U256::zero(),
        )
        .unwrap();

    if res.logs.iter().any(|log: &Log| {
        return log.topics.contains(
            &H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
                .unwrap(),
        );
    }) {
        let c = [String::from("name()(string)"), String::from("symbol()(string)")];

        let results = resolve_call_args::<String>(&c, executor.clone()).await;

        println!("Token name: {:#?}", results.first().unwrap());
        println!("Token symbol: {:#?}", results.get(results.len().wrapping_sub(1)).unwrap());
    }

    Ok(())
}
