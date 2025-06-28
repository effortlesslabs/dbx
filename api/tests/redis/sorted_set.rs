#[cfg(test)]
mod tests {
    use crate::routes::redis::sorted_set::*;
    use actix_web::{test, web, App};
    use serde_json::json;

    #[actix_web::test]
    async fn test_zadd() {
        let app = test::init_service(
            App::new().route("/zadd", web::post().to(zadd))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zadd")
            .set_json(&json!({
                "key": "test_zset",
                "items": [
                    {"score": 1.0, "member": "member1"},
                    {"score": 2.0, "member": "member2"}
                ]
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrange() {
        let app = test::init_service(
            App::new().route("/zrange", web::post().to(zrange))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zrange")
            .set_json(&json!({
                "key": "test_zset",
                "start": 0,
                "stop": -1
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrem() {
        let app = test::init_service(
            App::new().route("/zrem", web::post().to(zrem))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zrem")
            .set_json(&json!({
                "key": "test_zset",
                "members": ["member1", "member2"]
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zscore() {
        let app = test::init_service(
            App::new().route("/zscore", web::post().to(zscore))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zscore")
            .set_json(&json!({
                "key": "test_zset",
                "member": "member1"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrank() {
        let app = test::init_service(
            App::new().route("/zrank", web::post().to(zrank))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zrank")
            .set_json(&json!({
                "key": "test_zset",
                "member": "member1"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zcard() {
        let app = test::init_service(
            App::new().route("/zcard", web::post().to(zcard))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zcard")
            .set_json(&json!({
                "key": "test_zset"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zcount() {
        let app = test::init_service(
            App::new().route("/zcount", web::post().to(zcount))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zcount")
            .set_json(&json!({
                "key": "test_zset",
                "min": 0.0,
                "max": 10.0
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zincrby() {
        let app = test::init_service(
            App::new().route("/zincrby", web::post().to(zincrby))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zincrby")
            .set_json(&json!({
                "key": "test_zset",
                "increment": 1.5,
                "member": "member1"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrangebyscore() {
        let app = test::init_service(
            App::new().route("/zrangebyscore", web::post().to(zrangebyscore))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zrangebyscore")
            .set_json(&json!({
                "key": "test_zset",
                "min": 0.0,
                "max": 10.0,
                "withscores": false
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrevrange() {
        let app = test::init_service(
            App::new().route("/zrevrange", web::post().to(zrevrange))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zrevrange")
            .set_json(&json!({
                "key": "test_zset",
                "start": 0,
                "stop": -1,
                "withscores": false
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zremrangebyrank() {
        let app = test::init_service(
            App::new().route("/zremrangebyrank", web::post().to(zremrangebyrank))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zremrangebyrank")
            .set_json(&json!({
                "key": "test_zset",
                "start": 0,
                "stop": 1
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zremrangebyscore() {
        let app = test::init_service(
            App::new().route("/zremrangebyscore", web::post().to(zremrangebyscore))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zremrangebyscore")
            .set_json(&json!({
                "key": "test_zset",
                "min": 0.0,
                "max": 5.0
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zinterstore() {
        let app = test::init_service(
            App::new().route("/zinterstore", web::post().to(zinterstore))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zinterstore")
            .set_json(&json!({
                "destination": "dest_zset",
                "keys": ["zset1", "zset2"]
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zunionstore() {
        let app = test::init_service(
            App::new().route("/zunionstore", web::post().to(zunionstore))
        ).await;

        let req = test::TestRequest::post()
            .uri("/zunionstore")
            .set_json(&json!({
                "destination": "dest_zset",
                "keys": ["zset1", "zset2"]
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}