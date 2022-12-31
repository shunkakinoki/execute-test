mod config;
use anyhow::{Ok, Result};
use bytes::Bytes;
pub use config::NodeConfig;
use ethers::{
    abi::{Detokenize, Token},
    types::{Address, H256, U256},
};
use foundry_evm::executor::CallResult;
use foundry_evm::executor::EvmError;
use foundry_evm::executor::{fork::CreateFork, opts::EvmOpts, Backend, Executor, ExecutorBuilder};
use futures::future::join_all;
use std::str::FromStr;
pub fn spawn(config: &NodeConfig) -> Executor {
    let gas_limit: u64 = 18446744073709551615;

    let evm_opts = EvmOpts {
        fork_url: Some(config.url.clone()),
        fork_block_number: config.block,
        ..Default::default()
    };

    let env = evm_opts.evm_env_blocking().unwrap();
    let builder = ExecutorBuilder::default().with_gas_limit(gas_limit.into()).set_tracing(true);

    let fork_opts =
        Some(CreateFork { url: config.url.clone(), enable_caching: true, env, evm_opts });
    let db = Backend::spawn(fork_opts);
    let executor = builder.build(db);

    return executor;
}

pub async fn resolve_call_args<D: Detokenize>(
    args: &[String],
    executor: &Executor,
    config: &NodeConfig,
) -> Vec<D> {
    join_all(args.iter().map(|arg| async {
        executor.call(config.from, config.to, arg.clone(), (), 0.into(), None).unwrap().result
    }))
    .await
}

pub async fn resolve_interface<D: Detokenize>(
    arg: &String,
    interface: &String,
    executor: &Executor,
    config: &NodeConfig,
) -> Result<CallResult<D>, EvmError> {
    executor.call(
        config.from,
        config.to,
        arg.clone(),
        vec![hex::decode(interface).unwrap()],
        0.into(),
        None,
    )
}

pub async fn simulate(mut executor: Executor, config: &NodeConfig) -> Result<String> {
    if config.calldata == "" && config.value != "0" {
        let r = format!(
            "Transfering {} ETH from {} to {}",
            (&config.value).parse::<f64>().unwrap() / 1e18,
            config.from,
            config.to
        );
        return Ok(r);
    }

    let res = executor
        .call_raw_committing(
            config.from,
            config.to,
            hex::decode(&config.calldata).unwrap_or(Bytes::from("").to_vec()).into(),
            U256::from_str(&config.value).unwrap_or(U256::zero()).into(),
        )
        .unwrap();

    if res.reverted {
        println!("Logs: {:#?}", &res);
        return Ok("Reverted".to_string());
    }

    // let c = [String::from("supportsInterface()(bool)")];
    // let results = resolve_call_args::<Token>(&c, &executor, &config).await;
    // println!("Logs: {:#?}", &results);

    // let a = resolve_interface::<Token>(
    //     &String::from("supportsInterface()(string)"),
    //     &String::from("0x06fdde03"),
    //     &executor,
    //     &config,
    // )
    // .await;
    // println!("Supports Interface: {:#?}", &a);

    for log in &res.logs {
        if log.topics.contains(
            &H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
                .unwrap(),
        ) && log.topics.len() == 3
        {
            let c = [
                String::from("name()(string)"),
                String::from("symbol()(string)"),
                String::from("decimals()(uint8)"),
            ];

            let results = resolve_call_args::<Token>(&c, &executor, &config).await;

            let r = format!(
                "Transfering {} {} from {} to {}",
                U256::from_big_endian(&log.data[0..32]).as_u128() as f64 / 1e18,
                results.get(results.len().wrapping_sub(2)).unwrap(),
                Address::from_slice(&log.topics[1][12..32]),
                Address::from_slice(&log.topics[2][12..32]),
            );

            return Ok(r);
        }
        if log.topics.contains(
            &H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
                .unwrap(),
        ) && log.topics.len() == 4
        {
            let c = [String::from("name()(string)")];

            let results = resolve_call_args::<Token>(&c, &executor, &config).await;

            if Address::from_slice(&log.topics[1][12..32]) == Address::zero() {
                let r = format!(
                    "Minting {}#{} to {}",
                    results.first().unwrap(),
                    U256::from_big_endian(&log.topics[3][0..32]).as_u128() as f64,
                    Address::from_slice(&log.topics[2][12..32]),
                );
                return Ok(r);
            }

            let r = format!(
                "Transfering {}#{} from {} to {}",
                results.first().unwrap(),
                U256::from_big_endian(&log.topics[3][0..32]).as_u128() as f64,
                Address::from_slice(&log.topics[1][12..32]),
                Address::from_slice(&log.topics[2][12..32]),
            );

            return Ok(r);
        }
    }

    println!("Full Log: {:#?}", &res);

    Ok("".to_string())
}
