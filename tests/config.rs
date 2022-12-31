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
        .with_block(Some(16306300))
        .with_from("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed".to_string())
        .with_to("0x04F2694C8fcee23e8Fd0dfEA1d4f5Bb8c352111F".to_string())
        .with_calldata("a9059cbb000000000000000000000000225e9b54f41f44f42150b6aaa730da5f2d23faf2000000000000000000000000000000000000000000000000000000003b9aca00".to_string());
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(res, "Transfering 0.000000001 sOHM from 0x4fd9…45ed to 0x225e…faf2".to_string())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_tx_revert() {
    let config = NodeConfig::test()
        .with_block(Some(16001411))
        .with_from("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed".to_string())
        .with_to("0x65c25fadd9b88df5c8c101a3b99a5d614b708596".to_string())
        .with_calldata("asdfasdfadfasdf".to_string());
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(res, "Reverted".to_string())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nft_mint() {
    let config = NodeConfig::test()
        .with_block(Some(13834180))
        .with_from("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed".to_string())
        .with_to("0x6144d927ee371de7e7f8221b596f3432e7a8e6d9".to_string())
        .with_calldata("1249c58b".to_string());
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(res, "Minting Wagumi Cats#11 to 0x4fd9…45ed".to_string())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_nft_transfer() {
    let config = NodeConfig::test()
        .with_block(Some(16306973))
        .with_from("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed".to_string())
        .with_to("0xd945f759d422ae30a6166838317b937de08380e3".to_string())
        .with_calldata("42842e0e0000000000000000000000004fd9d0ee6d6564e80a9ee00c0163fc952d0a45ed0000000000000000000000002af8ddab77a7c90a38cf26f29763365d0028cfef000000000000000000000000000000000000000000000000000000000001c4ed360c6ebe".to_string());
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(
        res,
        "Transfering Zora API Genesis Hackathon#115949 from 0x4fd9…45ed to 0x2af8…cfef".to_string()
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sudo_swap() {
    let config = NodeConfig::test()
        .with_block(Some(15281016))
        .with_from("0x4fd9D0eE6D6564E80A9Ee00c0163fC952d0A45Ed".to_string())
        .with_to("0x2af8ddab77a7c90a38cf26f29763365d0028cfef".to_string())
        .with_calldata("42842e0e0000000000000000000000004fd9d0ee6d6564e80a9ee00c0163fc952d0a45ed0000000000000000000000002af8ddab77a7c90a38cf26f29763365d0028cfef000000000000000000000000000000000000000000000000000000000001c4ed360c6ebe".to_string());
    let s = spawn(&config);
    let res = simulate(s, &config).await.unwrap();
    assert_eq!(res, "".to_string())
}
