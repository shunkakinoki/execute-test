use anyhow::{Ok, Result};
use bytes::Bytes;
use ethers::{
    abi::Detokenize,
    types::{Log, H160, H256, U256},
};
use foundry_evm::executor::{fork::CreateFork, opts::EvmOpts, Backend, Executor, ExecutorBuilder};
use futures::future::join_all;
use std::{
    str::{from_utf8, FromStr},
    string::String,
};

#[tokio::main]
async fn main() -> Result<()> {
    let fork_url = String::from("https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9");
    let gas_limit: u64 = 18446744073709551615;

    let evm_opts = EvmOpts { fork_url: Some(fork_url.clone()), ..Default::default() };

    let env = evm_opts.evm_env_blocking().unwrap();

    let fork_opts = Some(CreateFork { url: fork_url, enable_caching: true, env, evm_opts });

    let db = Backend::spawn(fork_opts);

    let builder = ExecutorBuilder::default().with_gas_limit(gas_limit.into()).set_tracing(true);

    let mut executor = builder.build(db);

    let account_bal =
        executor.get_balance(H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap());

    println!("Balance before: {:#?}", account_bal.unwrap());

    let res = executor
        .call_raw_committing(
            H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
            H160::from_str("0x225E9B54F41F44F42150b6aAA730Da5f2d23FAf2").unwrap(),
            Bytes::from(""),
            U256::from(300_000_000),
        )
        .unwrap();

    let account_bal =
        executor.get_balance(H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap());

    println!("Balance after: {:#?}", account_bal.unwrap());
    println!("Gas used: {:#?}", res.gas_used);
    // println!("State change: {:#?}", res);

    let token_balance_res = executor
        .call_raw_committing(
            H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
            H160::from_str("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
            hex::decode("70a082310000000000000000000000004fd9d0ee6d6564e80a9ee00c0163fc952d0a45ed")
                .expect("valid")
                .into(),
            U256::zero(),
        )
        .unwrap();

    println!(
        "Token balance before: {:#?}",
        U256::from_big_endian(token_balance_res.result.as_ref())
    );

    let token_res = executor
        .call_raw_committing(
            H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
            H160::from_str("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
            hex::decode("a9059cbb000000000000000000000000225e9b54f41f44f42150b6aaa730da5f2d23faf2000000000000000000000000000000000000000000000000000000003b9aca00").expect("valid").into(),
            U256::zero(),
        )
        .unwrap();

    let token_balance_res = executor
        .call_raw_committing(
            H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
            H160::from_str("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
            hex::decode("70a082310000000000000000000000004fd9d0ee6d6564e80a9ee00c0163fc952d0a45ed")
                .expect("valid")
                .into(),
            U256::zero(),
        )
        .unwrap();

    let token_name_res = executor
        .call_raw(
            H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
            H160::from_str("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
            hex::decode("95d89b41").expect("valid").into(),
            U256::zero(),
        )
        .unwrap();

    println!(
        "Token balance after: {:#?}",
        U256::from_big_endian(token_balance_res.result.as_ref())
    );
    println!(
        "Token symbol: {:#?}",
        from_utf8(token_name_res.result.as_ref()).unwrap().replace(" ", "").trim_matches('\0')
    );
    println!("Gas used: {:#?}", token_res.gas_used);
    // println!("State change: {:#?}", token_res);

    if token_res.logs.iter().any(|log: &Log| {
        return log.topics.contains(
            &H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
                .unwrap(),
        );
    }) {
        println!("transfer log found!")
    }

    async fn resolve_call_args<D: Detokenize>(args: &[String], executor: Executor) -> Vec<D> {
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

    let c = [String::from("name()(string)"), String::from("symbol()(string)")];

    let results = resolve_call_args::<String>(&c, executor.clone()).await;

    println!("Token name: {:#?}", results.first().unwrap());
    println!("Token symbol: {:#?}", results.get(results.len().wrapping_sub(1)).unwrap());

    let c = [String::from("totalSupply()(uint256)")];

    let results = resolve_call_args::<U256>(&c, executor.clone()).await;

    println!("Total supply: {:#?}", results.first().unwrap());

    println!("Token name: {:#?}", results.last().unwrap());

    // let token_name_res: CallResult<String> = executor
    //     .call(
    //         H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").
    // unwrap(),         H160::from_str("
    // 0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
    //         "symbol()(string)",
    //         (),
    //         0.into(),
    //         None,
    //     )
    //     .unwrap();

    // println!("Token symbol: {:#?}", token_name_res.result);

    // let error_res = executor
    //     .call_raw_committing(
    //         H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").
    // unwrap(),         H160::from_str("
    // 0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F").unwrap(),
    //         hex::decode("9cbb000000000000000000000000225e9b54f41f44f42150b6aaa730da5f2d23faf2000000000000000000000000000000000000000000000000000000003b9aca00").expect("valid").into(),
    //         U256::zero(),
    //     )
    //     .unwrap();
    // println!("Tx error: {:#?}", error_res.exit_reason);

    Ok(())
}
