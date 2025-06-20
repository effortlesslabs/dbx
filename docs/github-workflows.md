# GitHub Workflows for DBX

This document describes the GitHub Actions workflows set up for comprehensive testing of the DBX project.

## Overview

The DBX project now includes four main GitHub workflows that provide comprehensive testing coverage:

1. **Main CI Pipeline** (`ci.yml`) - Primary workflow for pull requests and pushes
2. **Rust Tests** (`rust-tests.yml`) - Detailed Rust-specific testing
3. **TypeScript Tests** (`typescript-tests.yml`) - TypeScript SDK testing
4. **Scheduled Tests** (`scheduled-tests.yml`) - Nightly and weekly comprehensive tests

## Workflows Description

### 1. Main CI Pipeline (`ci.yml`)

**Purpose**: Primary workflow that orchestrates all testing activities with intelligent path-based filtering.

**Triggers**:
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches

**Features**:
- **Change Detection**: Only runs relevant tests based on changed files
- **Smart Dependencies**: Tests run in optimal order with proper dependencies
- **Quick Checks**: Fast formatting and linting checks before expensive tests
- **End-to-End Testing**: Complete integration testing with Docker
- **Security Auditing**: Automated security checks on main branch

**Jobs**:
- `changes` - Detects which parts of the codebase changed
- `rust-quick-check` - Fast Rust formatting and compilation checks
- `typescript-quick-check` - Fast TypeScript type checking and linting
- `unit-tests` - Rust unit tests with Redis
- `integration-tests` - Rust integration tests
- `docker-tests` - Full Docker-based testing
- `typescript-tests` - TypeScript SDK tests against live API
- `e2e-tests` - End-to-end testing of the entire system
- `security-audit` - Security vulnerability scanning
- `build-status` - Final status summary

### 2. Rust Tests (`rust-tests.yml`)

**Purpose**: Comprehensive Rust testing across multiple environments and configurations.

**Triggers**:
- Push/PR to main branches
- Changes to Rust code

**Features**:
- **Multi-Version Testing**: Tests against stable, beta, and nightly Rust
- **Docker Integration**: Tests in both native and Docker environments
- **Security Auditing**: cargo-audit for dependency vulnerabilities
- **Code Coverage**: LLVM-based coverage reporting with Codecov integration
- **Performance Benchmarks**: Automated benchmark running and tracking

**Jobs**:
- `test` - Main test suite across Rust versions
- `docker-tests` - Docker-based integration testing
- `security-audit` - Dependency vulnerability scanning
- `coverage` - Code coverage generation and reporting
- `benchmark` - Performance benchmark execution

### 3. TypeScript Tests (`typescript-tests.yml`)

**Purpose**: Comprehensive testing of the TypeScript SDK across multiple Node.js versions and platforms.

**Triggers**:
- Changes to `ts/` directory
- Changes to workflow files

**Features**:
- **Multi-Node Testing**: Tests across Node.js 18, 20, and 22
- **Cross-Platform**: Tests on Ubuntu, Windows, and macOS
- **Live API Testing**: Tests against running DBX API instance
- **Security Auditing**: npm/pnpm audit for vulnerabilities
- **Coverage Reporting**: Jest-based coverage with Codecov integration

**Jobs**:
- `typescript-tests` - Core TypeScript testing with live API
- `typescript-build-matrix` - Cross-platform build verification
- `typescript-security` - Security vulnerability scanning
- `typescript-coverage` - Test coverage reporting

### 4. Scheduled Tests (`scheduled-tests.yml`)

**Purpose**: Automated nightly and weekly comprehensive testing for continuous quality assurance.

**Triggers**:
- **Nightly**: Every day at 2 AM UTC
- **Weekly**: Every Sunday at 4 AM UTC
- **Manual**: Can be triggered manually with different test types

**Features**:
- **Compatibility Testing**: Tests against multiple Redis versions
- **Stress Testing**: Load testing and performance validation
- **Memory Leak Detection**: MIRI and Valgrind-based memory safety checks
- **Cross-Platform Testing**: Comprehensive testing across OS and versions
- **Performance Tracking**: Benchmark results archival and monitoring

**Jobs**:
- `nightly-tests` - Daily testing across Rust/Redis version matrix
- `comprehensive-tests` - Weekly cross-platform testing
- `stress-tests` - Load testing and stress validation
- `compatibility-tests` - Redis version compatibility checks
- `memory-leak-tests` - Memory safety and leak detection
- `performance-benchmarks` - Performance measurement and tracking
- `notify-results` - Failure notifications (can be extended with webhooks)

## Configuration Requirements

### Repository Secrets

For full functionality, configure these GitHub repository secrets:

```bash
CODECOV_TOKEN          # For code coverage reporting
SLACK_WEBHOOK_URL      # For failure notifications (optional)
```

### Branch Protection

Recommended branch protection rules for `main` and `develop`:

- Require status checks to pass before merging
- Require the following status checks:
  - `Build Status`
  - `Rust Quick Check`
  - `TypeScript Quick Check`
- Require up-to-date branches before merging
- Require linear history

## Usage Guidelines

### For Developers

1. **Pull Requests**: All workflows will run automatically on PRs
2. **Quick Feedback**: Fast checks (formatting, linting) run first for quick feedback
3. **Change-Based Testing**: Only relevant tests run based on changed files
4. **Local Testing**: Use the same commands locally:
   ```bash
   # Rust tests
   cargo test --all
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   
   # TypeScript tests
   cd ts && pnpm test
   cd ts && pnpm run lint
   cd ts && pnpm run format
   
   # Docker tests
   docker-compose up -d
   docker-compose exec dbx-api cargo test
   ```

### For Maintainers

1. **Manual Workflow Dispatch**: Trigger specific test types manually
2. **Monitoring**: Check nightly and weekly test results
3. **Performance Tracking**: Monitor benchmark results over time
4. **Security**: Review security audit results regularly

## Monitoring and Debugging

### Workflow Status

Check workflow status at:
- `https://github.com/effortlesslabs/dbx/actions`

### Common Issues and Solutions

1. **Redis Connection Failures**:
   - Check Redis service health in workflow logs
   - Verify `REDIS_URL` environment variable

2. **Docker Build Failures**:
   - Check Docker service availability
   - Review `docker-compose.yml` configuration

3. **TypeScript Test Failures**:
   - Ensure DBX API is running and accessible
   - Check Node.js version compatibility

4. **Flaky Tests**:
   - Use the scheduled tests to identify patterns
   - Check for race conditions in integration tests

### Performance Monitoring

- **Benchmark Results**: Stored as artifacts in scheduled workflow runs
- **Coverage Reports**: Available in Codecov dashboard
- **Build Times**: Monitor workflow execution times for performance regressions

## Extending the Workflows

### Adding New Test Types

1. Add new job to appropriate workflow file
2. Use existing patterns for services and caching
3. Update documentation

### Adding New Platforms

1. Extend matrix configurations in workflow files
2. Test platform-specific requirements
3. Update compatibility documentation

### Adding Notifications

Extend the `notify-results` job in scheduled tests:

```yaml
- name: Notify Slack on failure
  if: failure()
  run: |
    curl -X POST -H 'Content-type: application/json' \
      --data '{"text":"DBX tests failed: ${{ github.run_id }}"}' \
      ${{ secrets.SLACK_WEBHOOK_URL }}
```

## Best Practices

1. **Test Locally First**: Run tests locally before pushing
2. **Small Commits**: Keep commits focused for better CI feedback
3. **Monitor Performance**: Watch for test execution time increases
4. **Update Dependencies**: Keep workflow actions and dependencies updated
5. **Document Changes**: Update this document when modifying workflows

## Troubleshooting

### Common Workflow Failures

| Error | Cause | Solution |
|-------|--------|----------|
| `cargo test` fails | Rust compilation errors | Fix compilation issues locally |
| Redis connection timeout | Service startup delay | Increase health check timeout |
| Docker build timeout | Resource constraints | Optimize Docker build steps |
| TypeScript type errors | Type incompatibilities | Update TypeScript definitions |
| Coverage upload fails | Missing `CODECOV_TOKEN` | Add secret to repository |

### Getting Help

1. Check workflow logs for detailed error messages
2. Review this documentation for common solutions
3. Test the same commands locally to reproduce issues
4. Check GitHub Actions status page for service issues