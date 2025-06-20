# GitHub Workflows for DBX

This directory contains GitHub Actions workflows for the DBX project. These workflows provide comprehensive CI/CD, testing, security, and performance monitoring for both the API and crates components.

## 🔄 Workflows Overview

### 1. **test.yml** - Main Test Suite
**Triggers:** Push/PR to main/develop branches  
**Purpose:** Core testing workflow that runs on multiple Rust versions

**Jobs:**
- **test**: Runs on Rust stable/beta/nightly with Redis service
  - Code formatting checks (`cargo fmt`)
  - Clippy linting (`cargo clippy`)
  - Unit tests for crates and API
  - Integration tests with Redis
  - Example compilation checks

- **security-audit**: Security vulnerability scanning
- **coverage**: Code coverage analysis with Codecov integration
- **benchmark**: Performance benchmarks (if available)

**Redis Integration:** ✅ Redis 7 service with health checks

### 2. **api-tests.yml** - Dedicated API Testing
**Triggers:** Push/PR affecting API or crates  
**Purpose:** Comprehensive API testing including HTTP endpoints

**Jobs:**
- **api-unit-tests**: API-specific unit tests
- **api-integration-tests**: Integration tests with Redis
- **api-http-tests**: Live HTTP endpoint testing
  - Health endpoint validation
  - String/Hash/List operation testing
  - WebSocket connection testing
- **load-testing**: Performance testing with `wrk`
- **api-documentation**: Documentation validation

**Features:**
- Live API server testing
- HTTP endpoint validation
- Load testing with realistic scenarios
- WebSocket testing support

### 3. **security.yml** - Security & Dependencies
**Triggers:** Weekly schedule + push/PR to main  
**Purpose:** Security scanning and dependency management

**Jobs:**
- **security-audit**: `cargo audit` for vulnerabilities
- **dependency-check**: License compliance checking
- **update-dependencies**: Automated dependency updates (weekly)
- **codeql**: GitHub CodeQL security analysis
- **docker-security**: Trivy container vulnerability scanning
- **secret-scan**: TruffleHog secret detection

**Automated Features:**
- Weekly dependency update PRs
- Security vulnerability reports
- License compliance monitoring

### 4. **release.yml** - Release Automation
**Triggers:** Version tags (v*.*.*) + manual dispatch  
**Purpose:** Automated release process

**Jobs:**
- **test**: Pre-release testing
- **build**: Multi-platform binary builds
  - Linux (GNU/musl)
  - macOS (Intel/ARM)
- **docker**: Docker image building and publishing
- **publish-crates**: Publish to crates.io
- **create-release**: GitHub release creation

**Platforms:** Linux x64, Linux musl, macOS x64, macOS ARM64

### 5. **benchmark.yml** - Performance Monitoring
**Triggers:** Push to main, PRs, weekly schedule  
**Purpose:** Performance testing and regression detection

**Jobs:**
- **benchmark**: Rust benchmark execution
- **api-benchmark**: API performance testing
- **memory-profiling**: Memory usage analysis with Valgrind
- **performance-regression**: PR performance comparison

**Tools Used:**
- Criterion for Rust benchmarks
- wrk/hey for HTTP load testing
- Valgrind for memory profiling

## 🔧 Setup Requirements

### GitHub Secrets
Add these secrets to your repository:

```bash
# For Docker publishing
DOCKER_USERNAME=your_docker_username
DOCKER_PASSWORD=your_docker_password

# For crates.io publishing
CARGO_REGISTRY_TOKEN=your_crates_io_token
```

### Repository Settings
1. Enable GitHub Actions in repository settings
2. Configure branch protection rules for main/develop
3. Enable security alerts and dependency scanning

## 📊 Workflow Features

### Redis Integration
All workflows include Redis 7 service with:
- Health checks every 10 seconds
- 5 retry attempts with 5-second timeout
- Exposed on port 6379

### Caching Strategy
- Cargo registry and git cache
- Build artifact caching
- Separate cache keys per job type
- Restore fallbacks for cache misses

### Multi-Platform Support
- Ubuntu Latest (primary)
- macOS for release builds
- Rust stable/beta/nightly versions

### Error Handling
- Graceful failure handling
- Artifact collection on failures
- Detailed logging with RUST_BACKTRACE

## 🚀 Running Tests Locally

### Prerequisites
```bash
# Install Redis
docker run -d -p 6379:6379 redis:7-alpine

# Install system dependencies (Ubuntu/Debian)
sudo apt-get install libssl-dev pkg-config

# Install Rust toolchain
rustup install stable beta nightly
```

### Run Tests
```bash
# Unit tests (no Redis required)
cargo test --workspace --lib

# Integration tests (requires Redis)
REDIS_URL=redis://localhost:6379 cargo test --workspace --lib -- --ignored

# API tests
cd api
REDIS_URL=redis://localhost:6379 cargo test --verbose
```

### Run Benchmarks
```bash
# Install criterion
cargo install cargo-criterion

# Run benchmarks
REDIS_URL=redis://localhost:6379 cargo bench --workspace
```

## 📈 Monitoring & Reports

### Artifacts Generated
- **Test Results**: Test output and coverage reports
- **Benchmark Results**: Performance metrics and comparison
- **Security Reports**: Vulnerability and license scans
- **Release Binaries**: Multi-platform executables

### Integration Points
- **Codecov**: Code coverage reporting
- **GitHub Security**: Vulnerability alerts
- **Docker Hub**: Container image registry
- **crates.io**: Rust package registry

## 🔍 Troubleshooting

### Common Issues

**Redis Connection Failures:**
```bash
# Check Redis health in workflow logs
redis-cli -h localhost -p 6379 ping
```

**Build Cache Issues:**
- Clear cache by updating Cargo.lock
- Cache keys are based on Cargo.lock hash

**Test Timeouts:**
- Increase timeout values in workflow files
- Check Redis service startup logs

**Memory Issues:**
- Monitor resource usage in workflow logs
- Consider using larger GitHub Action runners

### Debugging Workflows
1. Check workflow run logs in GitHub Actions tab
2. Look for Redis service health check failures
3. Verify environment variables are set correctly
4. Check artifact uploads for detailed reports

## 📝 Customization

### Adding New Tests
1. Add test files to appropriate directories
2. Update workflow triggers if needed
3. Add new jobs to relevant workflow files

### Modifying Redis Configuration
Update the service configuration in workflow files:
```yaml
services:
  redis:
    image: redis:7-alpine
    # Add custom configuration here
```

### Performance Tuning
- Adjust cache strategies
- Modify parallel job limits
- Optimize build dependencies

---

For more information, see the individual workflow files and the main project README.