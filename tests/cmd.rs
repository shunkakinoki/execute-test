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
    let config = NodeConfig::test().with_value("300".to_string());
    println!("Res: {:#?}", config);
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(
        res,
        "Transfering 0.0000000000000003 ETH from 0x0000…0000 to 0x0000…0000".to_string()
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn test_token_transfer() {
    let config = NodeConfig::test()
        .with_from("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed".to_string())
        .with_to("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F".to_string())
        .with_calldata("a9059cbb000000000000000000000000225e9b54f41f44f42150b6aaa730da5f2d23faf2000000000000000000000000000000000000000000000000000000003b9aca00".to_string());
    println!("Res: {:#?}", config);
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(res, "Transfering 0 sOHM from 0x4fd9…45ed to 0x04f2…111f".to_string())
}
