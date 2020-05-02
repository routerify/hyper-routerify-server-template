extern crate hyper_routerify_server_template;

#[tokio::test]
async fn test() {
    hyper_routerify_server_template::setup_test_environment();

    assert_eq!(1, 1);
}
