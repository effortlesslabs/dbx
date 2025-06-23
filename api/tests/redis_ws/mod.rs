pub mod string;

use crate::common::TestContext;
use crate::get_test_ws_base_url;

pub async fn create_test_context() -> TestContext {
    let base_url = get_test_ws_base_url().await;
    TestContext::new(base_url)
}
