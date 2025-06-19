# DBX Roadmap

This document outlines the development roadmap for the DBX project, detailing both current progress and future plans.

## Current Status

As of now, DBX has implemented:

- **Redis Adapter**: Basic Redis client functionality with connection management
  - String primitives with comprehensive operations
  - Pipeline support for batched commands
  - Transaction support for atomic operations
  - Lua scripting capabilities
  - Predefined utility scripts for common patterns
- **REST API Layer (Redis)**: Modular REST API server for Redis, with per-database routes/handlers
- **Multi-database CLI**: Unified CLI to select database type and connection URL at runtime
- **Extensible Architecture**: Easy to add new databases (Postgres, MongoDB, etc.) by adding handler/route modules and updating the enum/CLI

## Short-term Goals (0-3 months)

- [x] **Core Interfaces**
  - [x] Define common traits across all database types
  - [x] Implement error handling framework
  - [x] Add comprehensive logging
- [x] **REST API Layer (Redis)**

  - [x] Design REST API endpoints for Redis operations
  - [x] Implement REST API server for Redis
  - [x] Modularize routes/handlers per database
  - [ ] Add authentication and authorization mechanisms

- [x] **CLI System**

  - [x] Develop CLI tool to run REST API server
  - [x] Support configuration via database type and URL
  - [x] Provide usage commands and help documentation

- [ ] **Redis Adapter Enhancements**

  - [ ] Add support for all Redis data types (Lists, Sets, Hashes, Sorted Sets)
  - [ ] Implement PubSub functionality
  - [ ] Add cluster support
  - [ ] Implement connection pooling improvements

- [ ] **New Database Adapters**

  - [ ] SQLite adapter
  - [ ] Basic PostgreSQL adapter (with REST API)
  - [ ] Add modular routes/handlers for new databases

- [ ] **Documentation**
  - [x] CLI and API usage examples
  - [x] Modular architecture and extension guide
  - [ ] Comprehensive API documentation
  - [ ] Usage examples for all implemented features
  - [ ] Integration guides

## Mid-term Goals (3-6 months)

- [ ] **Advanced Database Features**
  - [ ] Query builder interface
  - [ ] Migration support
  - [ ] Schema validation
- [ ] **Additional Database Adapters**
  - [ ] MongoDB adapter (with REST API)
  - [ ] MySQL adapter (with REST API)
  - [ ] DynamoDB adapter
- [ ] **Runtime Compatibility**

  - [ ] WASM compatibility
  - [ ] Embedded systems support
  - [ ] Worker runtime support

- [ ] **Performance Optimizations**
  - [ ] Benchmarking suite
  - [ ] Performance tuning
  - [ ] Connection pooling improvements

## Long-term Goals (6+ months)

- [ ] **Language Bindings**
  - [ ] TypeScript/JavaScript bindings
  - [ ] Python bindings
  - [ ] Ruby bindings
  - [ ] C# bindings
  - [ ] Java bindings
- [ ] **Enterprise Features**
  - [ ] Advanced security features
  - [ ] Monitoring and observability
  - [ ] Distributed tracing integration
- [ ] **Advanced Use Cases**
  - [ ] Caching layer
  - [ ] Rate limiting
  - [ ] Distributed locks
  - [ ] Job queues

## Community Goals

- [ ] **Community Building**

  - [ ] Contributor guidelines
  - [ ] Code of conduct
  - [ ] Regular release schedule
  - [ ] Community meetings

- [ ] **Quality Assurance**
  - [ ] Comprehensive test suite
  - [ ] CI/CD pipeline
  - [ ] Code coverage reporting
  - [ ] Security scanning

## How to Add a New Database

1. Add a new variant to the `DatabaseType` enum in `api/src/config.rs`
2. Create new handler and route modules in `api/src/handlers/` and `api/src/routes/`
3. Update the CLI/server logic to support the new type
4. Add API endpoint documentation and usage examples

## How to Contribute

We welcome contributions to help us achieve these roadmap items! If you're interested in working on a specific feature or enhancement:

1. Check the [issues](https://github.com/effortlesslabs/dbx/issues) to see if there's already work being done
2. If not, create a new issue describing what you'd like to work on
3. Fork the repository and submit a pull request with your changes

For major features, please discuss them first in the issues to ensure they align with the project's direction.
