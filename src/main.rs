use anyhow::{Ok, Result};
use bytes::Bytes;
use foundry_evm::executor::{fork::CreateFork, Executor};
use foundry_evm::executor::{opts::EvmOpts, Backend, ExecutorBuilder};
use primitive_types::{H160, U256};
use std::str::FromStr;

fn main() -> Result<()> {
    println!("Hello, world!");
    let fork_url = String::from("https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9");
    let gas_limit: u64 = 18446744073709551615;

    let evm_opts = EvmOpts {
        fork_url: Some(fork_url.clone()),
        ..Default::default()
    };

    let env = evm_opts.evm_env_blocking().unwrap();

    let fork_opts = Some(CreateFork {
        url: fork_url,
        enable_caching: true,
        env,
        evm_opts,
    });

    let db = Backend::spawn(fork_opts);

    let builder = ExecutorBuilder::default()
        .with_gas_limit(gas_limit.into())
        .set_tracing(true);

    let mut executor = builder.build(db);

    let account_bal =
        executor.get_balance(H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap());

    println!("Balance before: {:#?}", account_bal.unwrap());

    let res = executor.call_raw_committing(
        H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap(),
        H160::from_str("0x225E9B54F41F44F42150b6aAA730Da5f2d23FAf2").unwrap(),
        Bytes::from(""),
        U256::from(300_000_000),
    );

    if res.is_err() {
        println!("Error: {:#?}", res.unwrap_err());
    }

    let account_bal =
        executor.get_balance(H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap());

    println!("Balance after: {:#?}", account_bal.unwrap());

    Ok(())
}
