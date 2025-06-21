# Contributing to DBX Crates

Thank you for your interest in contributing to DBX Crates! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Feature Requests](#feature-requests)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- Redis server (for testing)
- Docker (optional, for containerized testing)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/dbx.git
   cd dbx
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/original-org/dbx.git
   ```

## Development Setup

### 1. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install additional tools
rustup component add rustfmt clippy
cargo install cargo-audit cargo-tarpaulin
```

### 2. Build the Project

```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p dbx-crates

# Build with specific features
cargo build -p dbx-crates --features "async,connection-pool"
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run crates tests only
cargo test -p dbx-crates

# Run with verbose output
cargo test -p dbx-crates -- --nocapture

# Run specific test
cargo test -p dbx-crates test_name

# Run doc tests
cargo test -p dbx-crates --doc
```

### 4. Code Quality Checks

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -p dbx-crates

# Run clippy with all warnings
cargo clippy -p dbx-crates -- -D warnings

# Security audit
cargo audit

# Code coverage (requires tarpaulin)
cargo tarpaulin -p dbx-crates
```

## Code Style

### Rust Conventions

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common issues
- Prefer `rustc` warnings over `clippy` warnings when they conflict

### Naming Conventions

- **Functions**: `snake_case`
- **Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Types**: `PascalCase`
- **Modules**: `snake_case`
- **Files**: `snake_case.rs`

### Documentation

- Document all public APIs with doc comments
- Use `///` for public items, `//!` for module-level docs
- Include examples in documentation
- Follow the [Rust Documentation Guidelines](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)

Example:

````rust
/// Creates a new Redis connection with the specified URL.
///
/// # Arguments
///
/// * `url` - The Redis connection URL (e.g., "redis://127.0.0.1:6379")
///
/// # Returns
///
/// Returns a `RedisResult<Redis>` containing the Redis instance or an error.
///
/// # Examples
///
/// ```rust
/// use dbx_crates::adapter::redis::Redis;
///
/// let redis = Redis::from_url("redis://127.0.0.1:6379")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn from_url(url: &str) -> RedisResult<Self> {
    // Implementation...
}
````

### Error Handling

- Use `thiserror` for custom error types
- Provide meaningful error messages
- Include context in error variants
- Use `?` operator for error propagation

Example:

```rust
#[derive(Debug, thiserror::Error)]
pub enum RedisError {
    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("Invalid command: {command}")]
    InvalidCommand { command: String },

    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

### Testing

- Write unit tests for all public functions
- Include integration tests for complex workflows
- Use meaningful test names
- Test both success and error cases
- Mock external dependencies when appropriate

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_connection_creation() {
        // Test implementation
    }

    #[test]
    fn test_redis_connection_with_invalid_url() {
        // Test error case
    }
}
```

## Testing

### Test Structure

```
crates/
├── src/
│   └── adapter/
│       └── redis/
│           ├── mod.rs
│           ├── client.rs
│           └── primitives/
│               ├── mod.rs
│               ├── string.rs
│               ├── hash.rs
│               └── set.rs
└── tests/
    ├── integration_tests.rs
    └── common/
        └── mod.rs
```

### Running Tests

```bash
# Unit tests
cargo test -p dbx-crates

# Integration tests
cargo test --test integration_tests

# Doc tests
cargo test -p dbx-crates --doc

# Test with specific features
cargo test -p dbx-crates --features "async,connection-pool"
```

### Test Guidelines

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test complete workflows
3. **Doc Tests**: Ensure documentation examples work
4. **Property Tests**: Use `proptest` for complex data structures
5. **Benchmark Tests**: Use `criterion` for performance testing

### Mocking

- Use `mockall` for mocking external dependencies
- Create mock implementations for testing without Redis
- Use feature flags to conditionally include mocks

Example:

```rust
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait RedisConnection {
    fn ping(&self) -> Result<bool, RedisError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_redis_connection::MockRedisConnection;

    #[test]
    fn test_ping() {
        let mut mock = MockRedisConnection::new();
        mock.expect_ping().returning(|| Ok(true));

        assert!(mock.ping().unwrap());
    }
}
```

## Pull Request Process

### Before Submitting

1. **Create a feature branch**:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**:

   - Write code following the style guidelines
   - Add tests for new functionality
   - Update documentation
   - Update CHANGELOG.md if needed

3. **Run quality checks**:

   ```bash
   cargo fmt
   cargo clippy -p dbx-crates -- -D warnings
   cargo test -p dbx-crates
   cargo test -p dbx-crates --doc
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add new Redis list operations"
   ```

### Commit Message Format

Use conventional commits format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:

```
feat(redis): add list operations support
fix(client): handle connection timeout properly
docs: update API documentation
test: add integration tests for batch operations
```

### Submitting the PR

1. **Push your branch**:

   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request** on GitHub with:

   - Clear title and description
   - Link to related issues
   - Summary of changes
   - Testing instructions
   - Screenshots (if UI changes)

3. **PR Template**:

   ```markdown
   ## Description

   Brief description of changes

   ## Type of Change

   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing

   - [ ] Unit tests pass
   - [ ] Integration tests pass
   - [ ] Documentation tests pass

   ## Checklist

   - [ ] Code follows style guidelines
   - [ ] Self-review completed
   - [ ] Documentation updated
   - [ ] CHANGELOG updated
   ```

### Review Process

1. **Automated Checks**: CI/CD pipeline runs tests and quality checks
2. **Code Review**: At least one maintainer must approve
3. **Address Feedback**: Respond to review comments
4. **Merge**: PR is merged after approval

## Issue Reporting

### Bug Reports

When reporting bugs, include:

1. **Environment**:

   - Rust version: `rustc --version`
   - OS and version
   - DBX Crates version

2. **Reproduction Steps**:

   - Clear, step-by-step instructions
   - Minimal code example
   - Expected vs actual behavior

3. **Additional Information**:
   - Error messages and stack traces
   - Logs (if applicable)
   - Screenshots (if UI-related)

### Issue Template

```markdown
## Bug Description

Brief description of the bug

## Steps to Reproduce

1. Step 1
2. Step 2
3. Step 3

## Expected Behavior

What you expected to happen

## Actual Behavior

What actually happened

## Environment

- Rust version:
- OS:
- DBX Crates version:

## Additional Information

Any other relevant information
```

## Feature Requests

### Before Requesting

1. **Check existing issues** for similar requests
2. **Search documentation** for existing solutions
3. **Consider alternatives** and their trade-offs

### Feature Request Template

```markdown
## Feature Description

Clear description of the requested feature

## Use Case

Why this feature is needed and how it would be used

## Proposed Solution

Optional: suggest an implementation approach

## Alternatives Considered

Other approaches that were considered

## Additional Context

Any other relevant information
```

## Documentation

### Documentation Standards

- **API Documentation**: Document all public APIs
- **Examples**: Include working code examples
- **Error Handling**: Document error conditions
- **Performance**: Note performance characteristics
- **Thread Safety**: Document thread safety guarantees

### Documentation Structure

```
docs/
├── README.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── examples/
│   ├── basic_usage.rs
│   ├── advanced_usage.rs
│   └── integration_examples.rs
└── api/
    ├── redis.md
    ├── client.md
    └── primitives.md
```

### Writing Documentation

1. **Clear and Concise**: Use simple, clear language
2. **Complete Examples**: Provide working code examples
3. **Error Cases**: Document common error scenarios
4. **Performance Notes**: Include performance considerations
5. **Thread Safety**: Document concurrency guarantees

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. **Update Version**:

   - Update `Cargo.toml` version
   - Update `CHANGELOG.md`
   - Update documentation

2. **Quality Assurance**:

   - Run full test suite
   - Run security audit
   - Check documentation
   - Verify examples work

3. **Release**:
   - Create release tag
   - Publish to crates.io
   - Update GitHub release notes

### Release Commands

```bash
# Update version
cargo set-version 1.2.3

# Create release commit
git commit -am "chore: release version 1.2.3"

# Create tag
git tag -a v1.2.3 -m "Release version 1.2.3"

# Push changes
git push origin main --tags

# Publish to crates.io
cargo publish -p dbx-crates
```

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Documentation**: Check the README and API docs
- **Examples**: Look at the examples directory

## Recognition

Contributors will be recognized in:

- GitHub contributors list
- CHANGELOG.md
- Release notes
- Project documentation

Thank you for contributing to DBX Crates! 🚀
