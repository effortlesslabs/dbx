#[cfg(test)]
mod tests {
    use crate::routes::redis_ws::sorted_set::*;
    use actix_web::{test, web, App};
    use actix_web_actors::ws;
    use serde_json::json;
    use std::time::Duration;

    #[actix_web::test]
    async fn test_zadd_ws() {
        let app = test::init_service(
            App::new().route("/ws/zadd", web::get().to(zadd_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zadd")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrange_ws() {
        let app = test::init_service(
            App::new().route("/ws/zrange", web::get().to(zrange_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zrange")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrem_ws() {
        let app = test::init_service(
            App::new().route("/ws/zrem", web::get().to(zrem_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zrem")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zscore_ws() {
        let app = test::init_service(
            App::new().route("/ws/zscore", web::get().to(zscore_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zscore")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrank_ws() {
        let app = test::init_service(
            App::new().route("/ws/zrank", web::get().to(zrank_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zrank")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zcard_ws() {
        let app = test::init_service(
            App::new().route("/ws/zcard", web::get().to(zcard_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zcard")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zcount_ws() {
        let app = test::init_service(
            App::new().route("/ws/zcount", web::get().to(zcount_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zcount")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zincrby_ws() {
        let app = test::init_service(
            App::new().route("/ws/zincrby", web::get().to(zincrby_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zincrby")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrangebyscore_ws() {
        let app = test::init_service(
            App::new().route("/ws/zrangebyscore", web::get().to(zrangebyscore_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zrangebyscore")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zrevrange_ws() {
        let app = test::init_service(
            App::new().route("/ws/zrevrange", web::get().to(zrevrange_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zrevrange")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zremrangebyrank_ws() {
        let app = test::init_service(
            App::new().route("/ws/zremrangebyrank", web::get().to(zremrangebyrank_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zremrangebyrank")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zremrangebyscore_ws() {
        let app = test::init_service(
            App::new().route("/ws/zremrangebyscore", web::get().to(zremrangebyscore_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zremrangebyscore")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zinterstore_ws() {
        let app = test::init_service(
            App::new().route("/ws/zinterstore", web::get().to(zinterstore_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zinterstore")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_zunionstore_ws() {
        let app = test::init_service(
            App::new().route("/ws/zunionstore", web::get().to(zunionstore_ws))
        ).await;

        let req = test::TestRequest::get()
            .uri("/ws/zunionstore")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}