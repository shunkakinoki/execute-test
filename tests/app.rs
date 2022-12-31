use foundry_simulator::NodeArgs;

#[tokio::test(flavor = "multi_thread")]
async fn test_default_config() {
    let _: App = App::parse_from(["fosim", "--help"]);
}
