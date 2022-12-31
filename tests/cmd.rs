use foundry_simulator::{simulate, spawn, NodeConfig};

#[tokio::test(flavor = "multi_thread")]
async fn test_default_config() {
    let config = NodeConfig::test();
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(res, "".to_string())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_base_transfer() {
    let config = NodeConfig::test().with_value(Some("300"));
    println!("Res: {:#?}", config);
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(
        res,
        "Transfering 0.0000000000000003 ETH from 0x0000…0000 to 0x0000…0000".to_string()
    )
}
