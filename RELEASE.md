# Release Process

## Overview

This project uses GitHub Actions to automatically create releases when version tags are pushed.

## How to Create a Release

### 1. Update Version Number

Update the version in `Cargo.toml`:

```toml
[package]
name = "qrgen"
version = "0.2.0"  # Update this
```

### 2. Commit Version Change

```bash
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git push origin main
```

### 3. Create and Push Tag

```bash
# Create a tag with version number
git tag v0.2.0

# Push the tag to GitHub
git push origin v0.2.0
```

**Important:** The tag must start with `v` followed by semantic versioning (e.g., `v1.0.0`, `v0.2.1`)

### 4. Wait for CI/CD

The release workflow will automatically:
1. ✅ Create a GitHub Release
2. ✅ Build binaries for all 5 platforms
3. ✅ Create `.tar.gz` archives (Linux/macOS) and `.zip` (Windows)
4. ✅ Generate SHA256 checksums for all binaries
5. ✅ Upload all artifacts to the release

### 5. Verify Release

Visit: https://github.com/DougieWougie/qrgen-rust/releases

You should see:
- Release notes
- Download links for all platforms
- Checksum files

## Release Artifacts

Each release includes:

### Linux x86_64
- `qrgen-X.Y.Z-linux-x86_64.tar.gz`
- `qrgen-X.Y.Z-linux-x86_64.tar.gz.sha256`

### Linux ARM64
- `qrgen-X.Y.Z-linux-aarch64.tar.gz`
- `qrgen-X.Y.Z-linux-aarch64.tar.gz.sha256`

### macOS x86_64 (Intel)
- `qrgen-X.Y.Z-macos-x86_64.tar.gz`
- `qrgen-X.Y.Z-macos-x86_64.tar.gz.sha256`

### macOS ARM64 (Apple Silicon)
- `qrgen-X.Y.Z-macos-aarch64.tar.gz`
- `qrgen-X.Y.Z-macos-aarch64.tar.gz.sha256`

### Windows x86_64
- `qrgen-X.Y.Z-windows-x86_64.zip`
- `qrgen-X.Y.Z-windows-x86_64.zip.sha256`

## Semantic Versioning

Follow semantic versioning (semver):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

Examples:
- `v1.0.0` - First stable release
- `v1.1.0` - Added new feature
- `v1.1.1` - Fixed a bug
- `v2.0.0` - Breaking changes

## Quick Release Commands

```bash
# Example: Release version 0.2.0

# 1. Update Cargo.toml version to 0.2.0
# 2. Then run:

git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git push origin main
git tag v0.2.0
git push origin v0.2.0

# 3. Watch the release build at:
# https://github.com/DougieWougie/qrgen-rust/actions
```

## Delete a Tag (if needed)

If you made a mistake:

```bash
# Delete local tag
git tag -d v0.2.0

# Delete remote tag
git push origin :refs/tags/v0.2.0

# Delete the release on GitHub manually if it was created
```

## Updating Release Notes

After the release is created, you can edit it on GitHub:

1. Go to https://github.com/DougieWougie/qrgen-rust/releases
2. Find your release
3. Click "Edit release"
4. Update description, add changelog, etc.
5. Save changes

## Changelog Template

When editing release notes, consider adding:

```markdown
## Changes

### Added
- New feature X
- Support for Y

### Fixed
- Bug in Z functionality
- Issue with A

### Changed
- Improved performance of B
- Updated dependencies

### Removed
- Deprecated feature C
```

## Pre-releases

For beta/alpha releases, use tags like:
- `v1.0.0-beta.1`
- `v1.0.0-alpha.2`
- `v1.0.0-rc.1`

Mark as pre-release on GitHub after creation.

## Troubleshooting

### Release workflow didn't trigger
- Ensure tag starts with `v`
- Check tag was pushed: `git push origin v0.2.0`
- Verify workflow file exists: `.github/workflows/release.yml`

### Build failed for specific platform
- Check Actions logs for errors
- ARM64 might need linker fixes
- Windows might need different archive tool

### Release already exists
- Delete the release on GitHub first
- Delete the tag: `git push origin :refs/tags/v0.2.0`
- Create new tag and push again

## Best Practices

1. ✅ Always test locally before creating a release
2. ✅ Run `cargo test` to ensure all tests pass
3. ✅ Run `cargo build --release` to verify it builds
4. ✅ Update CHANGELOG.md before releasing
5. ✅ Use semantic versioning consistently
6. ✅ Never force-push tags
7. ✅ Keep release notes informative and clear

## Example First Release

```bash
# For version 0.1.0 (initial release)

# 1. Ensure Cargo.toml has version = "0.1.0"

# 2. Create and push tag
git tag v0.1.0
git push origin v0.1.0

# 3. Wait for CI to build (5-10 minutes)

# 4. Verify at:
# https://github.com/DougieWougie/qrgen-rust/releases/tag/v0.1.0
```

## CI/CD Details

The release workflow (`.github/workflows/release.yml`):
- Triggers on tags matching `v*.*.*`
- Creates GitHub release automatically
- Builds for 5 platforms in parallel
- Generates checksums for verification
- Uploads all artifacts to release

Total build time: ~10-15 minutes
