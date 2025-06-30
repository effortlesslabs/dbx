use crate::common::TestContext;
use crate::get_test_base_url;
use serde_json::Value;

#[tokio::test]
async fn test_admin_ping() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .get(format!("{}/redis/admin/ping", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: String = res.json().await.unwrap();
    assert_eq!(body, "PONG");
}

#[tokio::test]
async fn test_admin_info() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .get(format!("{}/redis/admin/info", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: String = res.json().await.unwrap();
    assert!(body.contains("redis_version"));
}

#[tokio::test]
async fn test_admin_dbsize() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .get(format!("{}/redis/admin/dbsize", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: i64 = res.json().await.unwrap();
    assert!(body >= 0);
}

#[tokio::test]
async fn test_admin_health() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .get(format!("{}/redis/admin/health", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: Value = res.json().await.unwrap();
    assert!(body["is_healthy"].as_bool().unwrap_or(false));
    assert!(body["ping_response"].as_str().unwrap_or("") == "PONG");
}

#[tokio::test]
async fn test_admin_status() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .get(format!("{}/redis/admin/status", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: Value = res.json().await.unwrap();
    assert!(body["uptime_seconds"].as_i64().unwrap_or(0) >= 0);
    assert!(body["version"].as_str().is_some());
}

#[tokio::test]
async fn test_admin_memory_stats() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .get(format!("{}/redis/admin/stats/memory", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: Value = res.json().await.unwrap();
    assert!(body.get("used_memory").is_some());
}

#[tokio::test]
async fn test_admin_config_all() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .get(format!("{}/redis/admin/config/all", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: Value = res.json().await.unwrap();
    assert!(body.get("maxmemory").is_some() || body.get("timeout").is_some());
}

#[tokio::test]
async fn test_admin_flushdb() {
    let ctx = TestContext::new(get_test_base_url().await);
    let res = ctx
        .client
        .delete(format!("{}/redis/admin/flushdb", ctx.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 200);
}
