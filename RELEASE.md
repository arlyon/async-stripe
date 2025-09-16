# Release Process

This document explains the automated release process for async-stripe using release-plz.

## Overview

The project uses [release-plz](https://release-plz.ieme.me/) to automate the entire release workflow, including independent versioning, changelog generation, and publishing to crates.io. Release-plz provides Rust-native workspace support with intelligent dependency management.

## Release Workflow

### 1. Development & PRs
- Create PRs for features, fixes, or code generation updates
- Use conventional commit messages for proper version detection
- All PRs follow standard merge practices

### 2. Automated Release PR Creation
- After merge to master, release-plz automatically:
  - Analyzes commit history since last release
  - Determines version bumps for affected crates using conventional commits + API analysis
  - Creates/updates a "Release PR" with:
    - Version bumps in affected Cargo.toml files
    - Updated workspace-level CHANGELOG.md
    - Dependency version updates

### 3. Release Review & Approval
- Team reviews the Release PR to verify:
  - Version bump appropriateness
  - Changelog accuracy
  - Dependency update correctness
- No code changes - just version and changelog updates

### 4. Release Execution
- Merging the Release PR triggers automatic publishing:
  - Creates git tags for each released crate (`<package>-v<version>`)
  - Publishes crates to crates.io in dependency order
  - Creates GitHub releases with changelog notes
  - Skips crates with no changes

## Prerequisites

### GitHub Actions Permissions
To enable release-plz to work properly, you need to configure GitHub Actions permissions:

1. **Go to Repository Settings** → **Actions** → **General**
2. **Set Workflow permissions** to "Read and write permissions"
3. **Check "Allow GitHub Actions to create and approve pull requests"**

This allows the workflow to:
- Create and update Release PRs
- Commit version changes
- Create Git tags and GitHub releases

### Required Secrets
The workflow requires these repository secrets:

- `REPO_SCOPED_TOKEN`: Personal Access Token with repo permissions (for triggering CI on Release PRs)
- `CARGO_REGISTRY_TOKEN`: Token from crates.io for publishing packages

## Configuration (release-plz.toml)

The release process is configured in `release-plz.toml`:

### Workspace Settings
```toml
[workspace]
semver_check = true          # Enable cargo-semver-checks for API analysis
changelog_update = true      # Single workspace-level changelog
dependencies_update = true   # Auto-update dependency versions
```

### Package-Specific Settings
```toml
[[package]]
name = "async-stripe"
publish_features = ["runtime-tokio-hyper"]  # Main library features
```

### Key Features

1. **Independent Versioning**
   - Each crate versions independently based on actual changes
   - Only crates with modifications get version bumps
   - Dependent crates auto-update when dependencies change

2. **Intelligent Semantic Versioning**
   - Uses conventional commits for initial version determination
   - Enhanced with `cargo-semver-checks` for actual API breaking change detection
   - Can promote minor bumps to major if breaking changes detected

3. **Workspace-Aware Publishing**
   - Publishes crates in correct dependency order
   - Automatically updates internal dependency version constraints
   - Skips crates already published at current version

4. **Single Changelog**
   - Maintains one `CHANGELOG.md` at workspace root
   - Groups changes by crate and version
   - Generated using git-cliff

## Commit Message Format

Use conventional commit format for proper version detection:

- `feat:` - new feature (minor version bump)
- `fix:` - bug fix (patch version bump)  
- `feat!:` or `BREAKING CHANGE:` - breaking change (major version bump)
- `chore:`, `docs:`, `test:` - no version bump

Examples:
```
feat(types): add cryptocurrency payment methods
fix(webhook): resolve signature validation timing
feat(client)!: change async API to use new error types
```

## Manual Intervention

The process is mostly automated with a review gate. Manual intervention needed for:
- Reviewing and approving Release PRs
- Fixing failed releases
- Emergency hotfixes outside the normal flow
- Adjusting release configuration

## Crate Publishing Order

Release-plz automatically determines publishing order based on dependencies:

1. `async-stripe-types` (foundation)
2. `async-stripe-shared` (depends on types)
3. Generated domain crates (billing, checkout, etc.)
4. `async-stripe-client-core`
5. `async-stripe-webhook` 
6. `async-stripe` (main library)

Each crate only publishes if it has changes or updated dependencies.