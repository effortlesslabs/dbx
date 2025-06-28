pub mod string;
pub mod admin;
pub mod hash;
pub mod set;

use crate::common::TestContext;
use crate::get_test_base_url;

pub async fn create_test_context() -> TestContext {
    let base_url = get_test_base_url().await;
    TestContext::new(base_url)
}
