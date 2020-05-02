use std::sync::Once;

#[allow(dead_code)]
static TEST_ENV_INIT: Once = Once::new();

#[allow(dead_code)]
pub fn setup_test_environment() {
    TEST_ENV_INIT.call_once(|| {
        dotenv::dotenv().ok();
    });
}
