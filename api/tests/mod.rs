pub mod common;
pub mod redis;
pub mod redis_ws;

use std::sync::Arc;
use dbx_api::{ config::{ Config, DatabaseType }, server::Server, constants::defaults::Defaults };
use std::net::SocketAddr;

pub struct TestServer {
    pub server: Server,
    pub addr: SocketAddr,
}

impl TestServer {
    pub async fn new() -> anyhow::Result<Self> {
        let config = Config {
            database_type: DatabaseType::Redis,
            database_url: std::env
                ::var("DATABASE_URL")
                .unwrap_or_else(|_| Defaults::TEST_DATABASE_URL.to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| Defaults::TEST_HOST.to_string()),
            port: 0, // Use port 0 for random available port
            pool_size: std::env
                ::var("POOL_SIZE")
                .unwrap_or_else(|_| Defaults::TEST_POOL_SIZE.to_string())
                .parse()
                .unwrap_or(Defaults::TEST_POOL_SIZE),
        };

        let server = Server::new(config).await?;
        // Bind to port 0 to get a random available port
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let addr = listener.local_addr()?;
        drop(listener); // Release the port so axum can bind to it
        Ok(Self { server, addr })
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let app = self.server.create_router();
        let listener = tokio::net::TcpListener::bind(self.addr).await?;
        tokio::spawn(async move {
            if
                let Err(e) = axum::serve(
                    listener,
                    app.into_make_service_with_connect_info::<std::net::SocketAddr>()
                ).await
            {
                eprintln!("Server error: {}", e);
            }
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        Ok(())
    }
}

pub async fn get_test_server() -> Arc<TestServer> {
    let test_server = TestServer::new().await.expect("Failed to create test server");
    test_server.start().await.expect("Failed to start test server");
    Arc::new(test_server)
}

pub async fn get_test_base_url() -> String {
    let server = get_test_server().await;
    format!("http://{}", server.addr)
}

pub async fn get_test_ws_base_url() -> String {
    let server = get_test_server().await;
    format!("http://{}", server.addr)
}
