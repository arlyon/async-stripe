# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [1.0.0-alpha.11](https://github.com/arlyon/async-stripe/compare/async-stripe-client-core-v1.0.0-alpha.10...async-stripe-client-core-v1.0.0-alpha.11) - 2025-12-15

### Other

- update Cargo.toml dependencies

## [1.0.0-alpha.9](https://github.com/arlyon/async-stripe/compare/async-stripe-client-core-v1.0.0-alpha.8...async-stripe-client-core-v1.0.0-alpha.9) - 2025-12-04

### Fixed

- fix tests

### Other

- Merge pull request #804 from arlyon/claude/issue-722-20251126-2114
- Fix retry mechanism to follow Stripe documentation
- update readme
- make site public and add docs

## [1.0.0-alpha.8](https://github.com/arlyon/async-stripe/compare/async-stripe-client-core-v1.0.0-alpha.7...async-stripe-client-core-v1.0.0-alpha.8) - 2025-11-26

### Fixed

- allow restricted key (rk_) prefix in secret key validation
- fix clippy lints in tests

## [1.0.0-alpha.4](https://github.com/arlyon/async-stripe/compare/async-stripe-client-core-v1.0.0-alpha.3...async-stripe-client-core-v1.0.0-alpha.4) - 2025-10-02

### Other

- update everything else to edition 2024
- add migration guide

## [1.0.0-alpha.3](https://github.com/arlyon/async-stripe/compare/async-stripe-client-core-v1.0.0-alpha.2...async-stripe-client-core-v1.0.0-alpha.3) - 2025-09-16

### Added

- add From<Uuid> implementation for IdempotencyKey
- *(async-stripe-core)* Create newtype for idempotency key

### Fixed

- *(client-core)* add tracing dependency
- add some sanity checks to secret keys
- bump MSRV (closes #532)

### Other

- rework readme
- Remove unused total count field from list ([#730](https://github.com/arlyon/async-stripe/pull/730))
- bump to 1.0.0-alpha.2
- bump msrv to match master
- Fix formatting
- Bump to hyper1 and associated deps
- Client split ([#525](https://github.com/arlyon/async-stripe/pull/525))
- try bumping msrv
- Squashed commit of the following:
- Merge branch 'master' into readme-ref-next
- update MSRV
- bump msrv since openssl fails to compile
- mention next branch in readme
- Update README.md
- bump msrv
- bump msrv
- bump msrv
- Fix small typos
- bump MSRV
- much -> must
- Also update readme
- Update README.md
- Miscellaneous changes
- Bump MSRV to 1.54 to support modern examples
- Rename some links to the old repo
- Prepare to release 0.14
- Release 0.13.0
- Pin test toolchain to 1.49.0 (MSRV)
- Fix typos
- Bump version to rc3
- Change project name
- Overhaul contributing and readme
- Remove "blocking" as a default feature
- Update cargo example to include "default-tls" flag
- Update README.md
- Update openapi spec
- Bump version to 0.12.0
- Set default stripe api version
- Fix headers in documentation
- Bump crate version
- Bump version
- Update README w/ codegen instructions
- more updates
- 0.10.4 - add event id field
- bump version to 0.10.3 and add checkout.session test
- Bump version to 0.10.2
- Bump version to fix docs.rs build
- Bump version to 0.10.0
- Fix tests
- Update README.md
- Bump version number, update changelog
- Bump version to 0.9.1
- Bump version
- Fix examples for headers changes
- Bump version to 0.8.0
- Bump version to 0.7.2
- Fix handling of customer sources list
- Bump version number
- Bump version to 0.6.0
- Fix using client with another stripe account
- Bump version to 0.5.3
- Bump version to v0.5.2
- Bump version to v0.5.1
- Bump to v0.5.0 and change source repository
- Ensure payment source serializes correctly
- Update Source Enum and structs, added source functions
- Bump version to 0.4.7
- Bump version to 0.4.6
- Bump version to 0.4.5
- Bump to v0.4.4
- Fix documentation tests
- Update version to 0.4.2
- Add README section for using custom connect accounts
- Add a Getting Started section to the README
- Update docs to mention stripe-js client-side libraries
- Update version to 0.4.1
- Bump release to 0.4.0
- Merge branch '0.3.x'
- Bump release to 0.3.3
- Use Openssl by default, w/ plans to support rustls
- Bump version to 0.3.2 and update CHANGELOG
- Fix HTTPS Options in README
- Release v0.3.1
- Add features to allow either native-tls or openssl
- Remove version from README title
- Update released version to 0.3.0
- Add status badges to README
- Update readme for v0.2.0
- Revert README update; v0.1.1 is latest release
- Bump version to 0.2.0
- Add license/links/tags for Cargo
- Update Cargo.toml w/ publishable name
- Initial commit
