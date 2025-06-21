# Changelog

All notable changes to the DBX Crates package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Comprehensive Redis adapter with string, hash, and set operations
- Connection pooling support with async features
- Pipeline and transaction support for atomic operations
- Lua script integration for complex business logic
- Batch operations for performance optimization
- Comprehensive error handling with custom error types
- Type-safe operations for all Redis data types
- Connection health checks and automatic reconnection
- Rate limiting with Redis scripts
- TTL and expiry management
- Set operations (intersections, unions, differences)
- Hash operations with field-value pairs
- String operations with counters and basic operations

### Changed

- Initial release of DBX Crates package

### Deprecated

- None

### Removed

- None

### Fixed

- None

### Security

- None

## [0.1.0] - 2024-01-XX

### Added

- Initial release of DBX Crates
- Redis adapter with basic functionality
- String operations (GET, SET, INCR, DECR, etc.)
- Hash operations (HSET, HGET, HGETALL, etc.)
- Set operations (SADD, SREM, SMEMBERS, etc.)
- Pipeline support for batch operations
- Transaction support for atomic operations
- Lua script integration
- Connection pooling (with async feature)
- Comprehensive test suite
- Documentation and examples

### Features

- `async`: Enable async operations with Tokio
- `connection-pool`: Enable connection pooling
- `default`: Basic functionality only

### Dependencies

- redis = "0.23"
- thiserror = "1.0"
- serde = "1.0"
- serde_json = "1.0"
- tokio = "1.0" (optional)
- async-trait = "0.1" (optional)
- mockall = "0.11" (dev dependency)

---

## Version History

### Version 0.1.0

- **Release Date**: 2024-01-XX
- **Status**: Initial Release
- **Features**: Core Redis adapter functionality
- **Breaking Changes**: None (initial release)

---

## Contributing

To add entries to this changelog:

1. Add your changes under the `[Unreleased]` section
2. Use the appropriate category:

   - `Added` for new features
   - `Changed` for changes in existing functionality
   - `Deprecated` for soon-to-be removed features
   - `Removed` for now removed features
   - `Fixed` for any bug fixes
   - `Security` for security-related changes

3. When releasing a new version:
   - Move `[Unreleased]` content to a new version section
   - Update the version number and date
   - Create a new `[Unreleased]` section

## Links

- [GitHub Repository](https://github.com/your-org/dbx)
- [Documentation](https://docs.rs/dbx-crates)
- [Issues](https://github.com/your-org/dbx/issues)
- [Releases](https://github.com/your-org/dbx/releases)
