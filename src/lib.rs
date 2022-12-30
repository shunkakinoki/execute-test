mod config;
use anyhow::{Ok, Result};
pub use config::NodeConfig;
use foundry_evm::executor::{
    backend::DatabaseError, fork::CreateFork, opts::EvmOpts, Backend, ExecutorBuilder,
};
use primitive_types::{H160, U256};
use std::{str::FromStr, string::String};

pub async fn spawn(mut config: NodeConfig) -> Result<U256, DatabaseError> {
    let fork_url = String::from("https://mainnet.infura.io/v3/4c94c74f4dce4c43a8081cc3ebd6b3b9");
    let gas_limit: u64 = 18446744073709551615;

    let evm_opts = EvmOpts { fork_url: Some(fork_url.clone()), ..Default::default() };

    let env = evm_opts.evm_env_blocking().unwrap();
    let builder = ExecutorBuilder::default().with_gas_limit(gas_limit.into()).set_tracing(true);

    let fork_opts = Some(CreateFork { url: fork_url, enable_caching: true, env, evm_opts });
    let db = Backend::spawn(fork_opts);
    let executor = builder.build(db);

    let account_bal =
        executor.get_balance(H160::from_str("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed").unwrap());

    return account_bal;
}
