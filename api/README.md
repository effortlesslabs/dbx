# DBX API Server

A REST API and WebSocket server for Redis operations built with Rust and Axum.

## Features

- **HTTP REST API**: Simple, one-off REST requests for database operations
- **WebSocket API**: Real-time, low-latency interface for batch commands and streaming
- **String Operations**: GET, SET, DELETE, INCR, INCRBY, EXISTS, TTL
- **Batch Operations**: Set/Get/Delete multiple keys at once
- **Advanced Operations**: SETNX, Compare-and-Set, Rate Limiting
- **Lua Scripts**: Pre-built scripts for complex operations
- **Health Checks**: Server and Redis connection monitoring
- **CORS Support**: Cross-origin resource sharing enabled
- **Comprehensive Error Handling**: Proper HTTP status codes and error messages
- **Multi-Database Support**: Redis, PostgreSQL (planned)
- **Dual Protocol**: HTTP REST API and WebSocket for real-time operations
- **Modular Architecture**: Trait-based adapter layer for unified database operations
- **High Performance**: Built with Rust for optimal performance
- **Docker Support**: Complete containerization for development and production

## Quick Start

### Using Docker (Recommended)

1. **Development Environment:**

   ```bash
   # Setup development environment with all tools
   ./scripts/docker-setup.sh
   ```

2. **Production Environment:**

   ```bash
   # Setup production environment
   ./scripts/docker-prod.sh
   ```

3. **Manual Docker Setup:**

   ```bash
   # Development
   docker-compose --profile dev up -d

   # Production
   docker-compose -f docker-compose.prod.yml up -d
   ```

### Local Development

1. **Prerequisites:**

   - Rust 1.75+
   - Redis server
   - Cargo

2. **Setup:**

   ```bash
   # Clone the repository
   git clone <repository-url>
   cd dbx/api

   # Copy environment file
   cp env.example .env

   # Install dependencies
   cargo build

   # Run the server
   cargo run
   ```

## API Documentation

### HTTP API

#### Base URL

- Development: `http://localhost:3000`
- Production: `https://your-domain.com`

#### Endpoints

##### Health Check

```http
GET /health
```

**Response:**

```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

##### Redis Operations

**Set Key-Value:**

```http
POST /api/redis/set
Content-Type: application/json

{
  "key": "user:123",
  "value": "John Doe",
  "expiry": 3600
}
```

**Get Value:**

```http
GET /api/redis/get/user:123
```

**Delete Key:**

```http
DELETE /api/redis/delete/user:123
```

**List Keys:**

```http
GET /api/redis/keys?pattern=user:*
```

### WebSocket API

#### Connection

```javascript
const ws = new WebSocket("ws://localhost:3000/ws");
```

#### Message Format

```json
{
  "id": "unique-request-id",
  "command": "SET",
  "args": {
    "key": "user:123",
    "value": "John Doe",
    "expiry": 3600
  }
}
```

#### Supported Commands

- `SET` - Set key-value pair
- `GET` - Get value by key
- `DELETE` - Delete key
- `KEYS` - List keys by pattern
- `EXISTS` - Check if key exists
- `TTL` - Get time to live
- `EXPIRE` - Set expiration time

#### Response Format

```json
{
  "id": "unique-request-id",
  "success": true,
  "data": "value",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Configuration

### Environment Variables

| Variable        | Default                  | Description                    |
| --------------- | ------------------------ | ------------------------------ |
| `REDIS_URL`     | `redis://localhost:6379` | Redis connection URL           |
| `DATABASE_TYPE` | `redis`                  | Database type (redis/postgres) |
| `HOST`          | `127.0.0.1`              | Server host                    |
| `PORT`          | `3000`                   | Server port                    |
| `POOL_SIZE`     | `10`                     | Connection pool size           |
| `LOG_LEVEL`     | `INFO`                   | Logging level                  |

### Docker Environment

The Docker setup automatically configures environment variables:

**Development:**

```bash
REDIS_URL=redis://redis:6379
DATABASE_TYPE=redis
HOST=0.0.0.0
PORT=3000
POOL_SIZE=5
LOG_LEVEL=DEBUG
```

**Production:**

```bash
REDIS_URL=redis://redis:6379
DATABASE_TYPE=redis
HOST=0.0.0.0
PORT=3000
POOL_SIZE=20
LOG_LEVEL=WARN
```

## Docker Services

### Core Services

- **DBX API**: Main API server (port 3000)
- **Redis**: Database server (port 6379)

### Development Tools

- **Redis Commander**: Web UI for Redis (port 8081)
- **Redis Insight**: Advanced Redis management (port 8001)
- **Jaeger**: Distributed tracing (port 16686)
- **Prometheus**: Metrics collection (port 9090)
- **Grafana**: Metrics visualization (port 3001)

### Production Services

- **Nginx**: Reverse proxy with SSL (ports 80, 443)
- **SSL**: HTTPS termination and security headers

## Examples

### JavaScript Client

```javascript
// HTTP API
const response = await fetch("http://localhost:3000/api/redis/set", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({
    key: "user:123",
    value: "John Doe",
    expiry: 3600,
  }),
});

// WebSocket API
const ws = new WebSocket("ws://localhost:3000/ws");
ws.onmessage = (event) => {
  const response = JSON.parse(event.data);
  console.log("Response:", response);
};

ws.send(
  JSON.stringify({
    id: "1",
    command: "SET",
    args: { key: "user:123", value: "John Doe" },
  })
);
```

### Rust Client

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // HTTP API
    let response = client
        .post("http://localhost:3000/api/redis/set")
        .json(&serde_json::json!({
            "key": "user:123",
            "value": "John Doe",
            "expiry": 3600
        }))
        .send()
        .await?;

    println!("Response: {:?}", response.text().await?);
    Ok(())
}
```

## Development

### Project Structure

```
api/
├── src/
│   ├── main.rs              # Application entry point
│   ├── server.rs            # Server configuration
│   ├── config.rs            # Configuration management
│   ├── constants/           # Centralized constants
│   ├── handlers/            # Request handlers
│   ├── middleware.rs        # Middleware components
│   ├── models.rs            # Data models
│   └── routes/              # API routes
├── crates/                  # Shared crates
├── tests/                   # Test files
├── examples/                # Usage examples
├── scripts/                 # Setup scripts
├── docs/                    # Documentation
└── docker-compose*.yml      # Docker configurations
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test basic_tests

# Run with Docker
docker-compose exec dbx-api cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security vulnerabilities
cargo audit
```

## Monitoring and Observability

### Health Checks

- **API Health**: `GET /health`
- **Docker Health**: Built-in health checks for all services
- **Redis Health**: Automatic ping checks

### Metrics

- **Prometheus**: Metrics collection at `/metrics`
- **Grafana**: Pre-configured dashboards
- **Custom Metrics**: Request rates, response times, error rates

### Logging

- **Structured Logging**: JSON format in production
- **Log Levels**: Configurable per environment
- **Log Aggregation**: Ready for ELK stack

## Security

### Production Security Features

- **SSL/TLS**: HTTPS with modern cipher suites
- **Security Headers**: HSTS, XSS protection, content type options
- **Rate Limiting**: API rate limiting via Nginx
- **Non-root Containers**: All services run as non-root users
- **Network Isolation**: Services communicate via internal network
- **Resource Limits**: Memory and CPU limits per container

### Redis Security

- **Password Protection**: Configure `requirepass` in `redis.conf`
- **Network Binding**: Bind to internal network only
- **Memory Limits**: Prevent memory exhaustion attacks

## Deployment

### Docker Deployment

```bash
# Development
./scripts/docker-setup.sh

# Production
./scripts/docker-prod.sh
```

### Manual Deployment

```bash
# Build release
cargo build --release

# Run with environment variables
REDIS_URL=redis://your-redis:6379 cargo run --release
```

### Scaling

```bash
# Scale API instances
docker-compose -f docker-compose.prod.yml up -d --scale dbx-api=3

# Scale with load balancer
docker-compose -f docker-compose.prod.yml up -d --scale dbx-api=5
```

## Troubleshooting

### Common Issues

1. **Port Conflicts**

   ```bash
   # Check port usage
   netstat -tulpn | grep :3000

   # Change ports in docker-compose.yml
   ports:
     - "3001:3000"
   ```

2. **Redis Connection Issues**

   ```bash
   # Test Redis connection
   redis-cli ping

   # Check Redis logs
   docker-compose logs redis
   ```

3. **SSL Certificate Issues**
   ```bash
   # Generate new self-signed certificate
   openssl req -x509 -newkey rsa:4096 -keyout ssl/key.pem -out ssl/cert.pem -days 365 -nodes
   ```

### Debugging

```bash
# View logs
docker-compose logs -f

# Access container shell
docker exec -it dbx-api bash

# Check health status
curl http://localhost:3000/health
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [x] HTTP API Layer (Redis)
- [x] WebSocket API Layer (Redis)
- [x] Docker Configuration
- [x] Monitoring and Observability
- [ ] PostgreSQL Support
- [ ] Authentication and Authorization
- [ ] PubSub/Streaming Support
- [ ] GraphQL API
- [ ] Kubernetes Deployment
- [ ] Performance Benchmarks
