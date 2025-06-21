# Contributing to DBX Crates

Thank you for contributing to DBX Crates! This guide covers the essentials.

## Quick Start

### Prerequisites

- Rust 1.70+
- Git
- Redis server (for testing)

### Setup

```bash
# Fork and clone
git clone https://github.com/your-username/dbx.git
cd dbx

# Install tools
rustup component add rustfmt clippy

# Build and test
cargo build -p dbx-crates
cargo test -p dbx-crates
```

## Development Workflow

### 1. Create Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Follow Rust conventions
- Add tests for new functionality
- Update documentation

### 3. Quality Checks

```bash
cargo fmt
cargo clippy -p dbx-crates -- -D warnings
cargo test -p dbx-crates
cargo test -p dbx-crates --doc
```

### 4. Commit & Push

```bash
git add .
git commit -m "feat: add new Redis operations"
git push origin feature/your-feature-name
```

## Code Style

### Rust Conventions

- Use `snake_case` for functions and variables
- Use `PascalCase` for types
- Use `SCREAMING_SNAKE_CASE` for constants
- Document public APIs with `///` comments

### Example Documentation

````rust
/// Creates a new Redis connection.
///
/// # Examples
///
/// ```rust
/// use dbx_crates::adapter::redis::Redis;
/// let redis = Redis::from_url("redis://127.0.0.1:6379")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn from_url(url: &str) -> RedisResult<Self> {
    // Implementation...
}
````

## Testing

### Test Commands

```bash
# Unit tests
cargo test -p dbx-crates

# With specific features
cargo test -p dbx-crates --features "async,connection-pool"

# Doc tests
cargo test -p dbx-crates --doc
```

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

## Pull Request Process

### Before Submitting

1. ✅ Code follows style guidelines
2. ✅ Tests pass for all feature combinations
3. ✅ Documentation updated
4. ✅ Self-review completed

### Commit Message Format

```
<type>: <description>

feat: add new Redis list operations
fix: handle connection timeout properly
docs: update API documentation
test: add integration tests
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### PR Checklist

- [ ] Clear title and description
- [ ] All tests pass
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Documentation updated

## Issue Reporting

### Bug Report Template

```markdown
## Description

Brief description of the bug

## Steps to Reproduce

1. Step 1
2. Step 2

## Expected vs Actual Behavior

What you expected vs what happened

## Environment

- Rust version: `rustc --version`
- OS:
- DBX Crates version:
```

## Getting Help

- **Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Documentation**: Check README and API docs

Thank you for contributing! 🚀
