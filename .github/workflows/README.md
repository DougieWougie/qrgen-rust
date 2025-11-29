# GitHub Actions CI/CD

This directory contains GitHub Actions workflows for automated testing and building.

## Workflows

### CI Workflow (`ci.yml`)

Automatically runs on:
- Push to `main` or `master` branches
- Pull requests targeting `main` or `master` branches

#### Jobs

##### 1. Test Suite (`test`)
Runs comprehensive testing on Ubuntu latest:
- ✅ Code formatting check (`cargo fmt`)
- ✅ Linting with Clippy (`cargo clippy`)
- ✅ All unit tests (47 tests)
- ✅ All integration tests (21 tests)
- ✅ Generates test report (JSON format)

**Cache Strategy:**
- Cargo registry
- Cargo git index
- Build artifacts

##### 2. Build Release (`build`)
Only runs if tests pass. Builds release binaries for multiple platforms:

| Platform | Target | Artifact Name |
|----------|--------|---------------|
| Linux x86_64 | `x86_64-unknown-linux-gnu` | `qrgen-linux-x86_64` |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | `qrgen-linux-aarch64` |
| macOS x86_64 | `x86_64-apple-darwin` | `qrgen-macos-x86_64` |
| macOS ARM64 (M1/M2) | `aarch64-apple-darwin` | `qrgen-macos-aarch64` |
| Windows x86_64 | `x86_64-pc-windows-msvc` | `qrgen-windows-x86_64.exe` |

**Features:**
- Cross-compilation for multiple targets
- Binary stripping for smaller size (Linux/macOS)
- Artifact retention: 7 days
- Separate caching per platform

##### 3. Linting (`lint`)
Runs code quality checks:
- ✅ Format verification
- ✅ Clippy warnings as errors

##### 4. Security Audit (`security`)
Security vulnerability scanning:
- ✅ Runs `cargo audit` on dependencies
- ✅ Identifies known security issues

## Workflow Execution

### On Push to Main/Master

```
1. Test Suite runs
   ├─ Format check
   ├─ Clippy linting
   └─ Run all tests

2. If tests PASS:
   └─ Build Release runs
      ├─ Build for Linux x86_64
      ├─ Build for Linux ARM64
      ├─ Build for macOS x86_64
      ├─ Build for macOS ARM64
      └─ Build for Windows x86_64

3. Parallel jobs:
   ├─ Linting
   └─ Security Audit
```

### On Pull Request

Same workflow as push, but:
- Does not create releases
- Provides build verification
- Shows test results in PR

## Artifacts

After successful builds, download artifacts from:
- GitHub Actions run summary
- "Artifacts" section at bottom of workflow run

Artifacts include:
- Compiled binaries for each platform
- Optimized for release (smaller, faster)
- Stripped of debug symbols (except Windows)

## Cache Strategy

The workflow uses aggressive caching to speed up builds:

1. **Cargo Registry** - Downloaded crate metadata
2. **Cargo Index** - Git index of crates
3. **Build Target** - Compiled dependencies

Cache keys include:
- OS/platform
- Target architecture
- `Cargo.lock` hash (invalidates on dependency changes)

## Local Testing

Test the workflow locally before pushing:

```bash
# Run the same checks as CI
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --release --verbose

# Build for your platform
cargo build --release
```

## Performance

Typical execution times:

| Job | First Run | Cached Run |
|-----|-----------|------------|
| Test Suite | ~2-3 min | ~30-45 sec |
| Build (per platform) | ~3-5 min | ~1-2 min |
| Linting | ~1-2 min | ~20-30 sec |
| Security Audit | ~30 sec | ~20 sec |

**Total workflow time:** ~5-10 minutes (with caching)

## Status Badge

Add this to your README.md to show build status:

```markdown
![CI](https://github.com/YOUR_USERNAME/qrgen-rust/workflows/CI/badge.svg)
```

## Troubleshooting

### Tests Failing in CI but Pass Locally

1. Check Rust version: CI uses stable, you might use nightly
2. Check OS differences: CI runs on Ubuntu
3. Review test logs in GitHub Actions

### Build Failing for Specific Platform

1. Check cross-compilation tools (ARM64 Linux requires gcc-aarch64-linux-gnu)
2. Review platform-specific dependencies
3. Check for platform-specific code issues

### Cache Issues

If builds are slower than expected:
1. Check cache hit rate in workflow logs
2. Clear cache by changing cache key (update workflow file)
3. Dependencies changed? Cache auto-invalidates

### Clippy Warnings

If Clippy fails:
1. Run locally: `cargo clippy --all-targets --all-features -- -D warnings`
2. Fix warnings or use `#[allow(clippy::warning_name)]` if justified
3. Commit fixes and push

## Customization

### Add More Platforms

Add to the build matrix in `ci.yml`:

```yaml
- os: ubuntu-latest
  target: x86_64-unknown-linux-musl  # Static linking
  artifact_name: qrgen
  asset_name: qrgen-linux-musl
```

### Change Test Command

Modify the test step:

```yaml
- name: Run tests
  run: cargo test --release --verbose --all-features
```

### Add Code Coverage

Add after tests:

```yaml
- name: Generate code coverage
  uses: actions-rs/tarpaulin@v0.1
  with:
    args: '--out Lcov'
```

## Security

The workflow:
- ✅ Uses official GitHub Actions
- ✅ Pins action versions (v4, not @latest)
- ✅ Runs `cargo audit` for dependencies
- ✅ Only triggers on specific branches
- ✅ No secrets required

## Future Enhancements

- [ ] Automated releases on version tags
- [ ] Code coverage reporting
- [ ] Benchmark tracking
- [ ] Deploy to package registries (crates.io)
- [ ] Nightly builds
- [ ] Performance regression testing
