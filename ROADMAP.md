# DBX Roadmap

This document outlines the development roadmap for the DBX project, detailing both current progress and future plans.

## Current Status

As of now, DBX has implemented:

- **Redis Adapter**: Complete Redis client functionality with connection management
  - [x] String primitives with comprehensive operations
  - [x] **Hash primitives with full CRUD operations, batch operations, pipeline support, transactions, and Lua scripts**
  - [x] Set primitives with comprehensive operations
  - [x] Pipeline support for batched commands
  - [x] Transaction support for atomic operations
  - [x] Lua scripting capabilities
  - [x] Predefined utility scripts for common patterns
- **REST API Layer (Redis)**: Complete REST API server for Redis with modular architecture
  - [x] String operations API endpoints
  - [x] **Hash operations API endpoints (GET, SET, DELETE, batch operations)**
  - [x] Set operations API endpoints
  - [x] Key management endpoints
  - [x] Script execution endpoints
  - [x] Proper route nesting under `/api/v1/redis`
  - [x] State management and server compilation fixes
- **WebSocket API Layer (Redis)**: Real-time WebSocket interface for low-latency operations
  - [x] JSON-encoded command protocol
  - [x] Support for all Redis string operations
  - [x] Support for Redis hash operations
  - [x] Batch operations for efficient multi-key operations
  - [x] Connection management and error handling
  - [x] Client examples in Rust and JavaScript
  - [ ] PubSub/streaming support (future enhancement)
- **Multi-database CLI**: Unified CLI to select database type and connection URL at runtime
- **Extensible Architecture**: Easy to add new databases (Postgres, MongoDB, etc.) by adding handler/route modules
- **TypeScript SDK**: Complete TypeScript SDK with full type definitions
- **Server Infrastructure**:
  - [x] Proper state management and router configuration
  - [x] CORS support
  - [x] Error handling and fallback routes
  - [x] Health check endpoints
  - [x] Redis connection validation

## Short-term Goals (0-3 months)

### High Priority - Production Readiness

- [ ] **Docker Support**
  - [ ] Add `Dockerfile` for the API server
  - [ ] Create `docker-compose.yml` with Redis included
  - [ ] Add health checks and proper container configuration
  - [ ] Environment variable configuration for database URLs
- [ ] **Environment Configuration**
  - [ ] Support for `.env` files
  - [ ] Environment variable overrides for all config options
  - [ ] Default configurations for common deployment scenarios
- [ ] **Authentication & Security**
  - [ ] Add authentication and authorization mechanisms
  - [ ] API key management
  - [ ] Rate limiting
  - [ ] Input validation and sanitization
- [ ] **Deployment Documentation**
  - [ ] Quick start guide with Docker
  - [ ] Deployment guides for common platforms (Heroku, Railway, DigitalOcean)
  - [ ] Production configuration examples

### Core Features - Redis Completion

- [ ] **Redis Adapter Enhancements**
  - [ ] Add support for remaining Redis data types (Lists, Sorted Sets)
  - [ ] Implement PubSub functionality
  - [ ] Add cluster support
  - [ ] Implement connection pooling improvements
  - [ ] Add Redis Streams support
- [ ] **API Enhancements**
  - [ ] Add List operations API endpoints
  - [ ] Add Sorted Set operations API endpoints
  - [ ] Add Stream operations API endpoints
  - [ ] Add PubSub endpoints
  - [ ] Add cluster management endpoints
- [ ] **Performance & Monitoring**
  - [ ] Graceful shutdown handling
  - [ ] Signal handling (SIGTERM, SIGINT)
  - [ ] Enhanced health check endpoint with detailed metrics
  - [ ] Basic metrics endpoint
  - [ ] Logging configuration for production
  - [ ] Performance benchmarking suite

### Medium Priority

- [ ] **New Database Adapters**
  - [ ] SQLite adapter with REST API
  - [ ] Basic PostgreSQL adapter with REST API
  - [ ] Add modular routes/handlers for new databases
- [ ] **Documentation**
  - [x] CLI and API usage examples
  - [x] WebSocket API documentation and examples
  - [x] Modular architecture and extension guide
  - [ ] Comprehensive API documentation with OpenAPI/Swagger
  - [ ] Usage examples for all implemented features
  - [ ] Integration guides
  - [ ] Performance tuning guide

## Mid-term Goals (3-6 months)

- [ ] **Advanced Database Features**
  - [ ] Query builder interface
  - [ ] Migration support
  - [ ] Schema validation
  - [ ] Backup and restore functionality
- [ ] **Additional Database Adapters**
  - [ ] MongoDB adapter (with REST API and WebSocket)
  - [ ] MySQL adapter (with REST API and WebSocket)
  - [ ] DynamoDB adapter
  - [ ] Cassandra adapter
- [ ] **Runtime Compatibility**
  - [ ] WASM compatibility
  - [ ] Embedded systems support
  - [ ] Worker runtime support
- [ ] **Advanced Use Cases**
  - [ ] Caching layer with TTL management
  - [ ] Distributed locks implementation
  - [ ] Job queues with Redis
  - [ ] Session management
  - [ ] Real-time analytics

## Long-term Goals (6+ months)

- [ ] **Language Bindings**
  - [x] TypeScript/JavaScript bindings
  - [ ] Python bindings
  - [ ] Ruby bindings
  - [ ] C# bindings
  - [ ] Java bindings
  - [ ] Go bindings
- [ ] **Enterprise Features**
  - [ ] Advanced security features (encryption, audit logs)
  - [ ] Monitoring and observability (Prometheus, Grafana)
  - [ ] Distributed tracing integration (Jaeger, Zipkin)
  - [ ] Multi-tenancy support
  - [ ] Data replication and failover
- [ ] **Cloud Integration**
  - [ ] AWS ElastiCache integration
  - [ ] Azure Cache for Redis integration
  - [ ] Google Cloud Memorystore integration
  - [ ] Kubernetes operator

## Community Goals

- [ ] **Community Building**
  - [ ] Contributor guidelines
  - [ ] Code of conduct
  - [ ] Regular release schedule
  - [ ] Community meetings
  - [ ] Discord/Slack community
- [ ] **Quality Assurance**
  - [ ] Comprehensive test suite (unit, integration, e2e)
  - [ ] CI/CD pipeline with GitHub Actions
  - [ ] Code coverage reporting
  - [ ] Security scanning
  - [ ] Performance regression testing

## Recent Achievements

- ✅ **Redis Hash API**: Complete implementation with full CRUD operations
- ✅ **Server Compilation**: Fixed type mismatches and state management issues
- ✅ **Route Registration**: Proper nesting of Redis routes under `/api/v1/redis`
- ✅ **Connection Management**: Robust Redis connection handling with validation
- ✅ **Error Handling**: Comprehensive error responses and fallback routes

## How to Add a New Database

1. Add a new variant to the `DatabaseType` enum in `api/src/config.rs`
2. Create new handler and route modules in `api/src/handlers/` and `api/src/routes/`
3. Update the CLI/server logic to support the new type
4. Add API endpoint documentation and usage examples

## How to Add WebSocket Support for New Databases

1. Create a new WebSocket handler in `api/src/handlers/websocket.rs` or create a separate module
2. Implement the `process_command` method for the new database type
3. Add WebSocket routes in `api/src/routes/websocket.rs`
4. Update the server to include the new WebSocket handler
5. Add WebSocket command documentation and examples

## How to Contribute

We welcome contributions to help us achieve these roadmap items! If you're interested in working on a specific feature or enhancement:

1. Check the [issues](https://github.com/effortlesslabs/dbx/issues) to see if there's already work being done
2. If not, create a new issue describing what you'd like to work on
3. Fork the repository and submit a pull request with your changes

For major features, please discuss them first in the issues to ensure they align with the project's direction.

## Next Immediate Steps

1. **Docker Support**: Containerize the application for easy deployment
2. **Authentication**: Add basic API key authentication
3. **Redis Lists**: Implement List data type support
4. **Documentation**: Create comprehensive API documentation
5. **Testing**: Add integration tests for all endpoints
