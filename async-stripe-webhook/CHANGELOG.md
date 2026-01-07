# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [1.0.0-rc.0](https://github.com/arlyon/async-stripe/compare/async-stripe-webhook-v1.0.0-alpha.8...async-stripe-webhook-v1.0.0-rc.0) - 2025-12-04

### Added

- add generate_test_header for webhook signature testing

### Other

- round trip webhook
- Merge pull request #815 from arlyon/claude/issue-808-20251127-2210
- add wasm webhook parser

## [1.0.0-alpha.8](https://github.com/arlyon/async-stripe/compare/async-stripe-webhook-v1.0.0-alpha.7...async-stripe-webhook-v1.0.0-alpha.8) - 2025-11-26

### Fixed

- trace a warning when using a different SDK version than what stripe reports

### Other

- stronger compiler hints to improve stack space re-use
- run api generate
- BREAKING CHANGE: box all webhook variants to reduce clone stack usage

## [1.0.0-alpha.7](https://github.com/arlyon/async-stripe/compare/async-stripe-webhook-v1.0.0-alpha.6...async-stripe-webhook-v1.0.0-alpha.7) - 2025-11-13

### Other

- Generate latest changes from OpenApi spec

## [1.0.0-alpha.6](https://github.com/arlyon/async-stripe/compare/async-stripe-webhook-v1.0.0-alpha.5...async-stripe-webhook-v1.0.0-alpha.6) - 2025-10-31

### Other

- Generate latest changes from OpenApi spec

## [1.0.0-alpha.5](https://github.com/arlyon/async-stripe/compare/async-stripe-webhook-v1.0.0-alpha.4...async-stripe-webhook-v1.0.0-alpha.5) - 2025-10-30

### Fixed

- large stack frames lint

### Other

- deny large stack frames

## [1.0.0-alpha.4](https://github.com/arlyon/async-stripe/compare/async-stripe-webhook-v1.0.0-alpha.3...async-stripe-webhook-v1.0.0-alpha.4) - 2025-10-02

### Other

- update everything else to edition 2024

## [1.0.0-alpha.3](https://github.com/arlyon/async-stripe/compare/async-stripe-webhook-v1.0.0-alpha.2...async-stripe-webhook-v1.0.0-alpha.3) - 2025-09-16

### Other

- *(deps)* bump the production-dependencies group with 3 updates
- make sure we only run webhook billing test when billing is enabled
- Update openapi spec for next branch ([#728](https://github.com/arlyon/async-stripe/pull/728))
- Fix currency by adding unknown variant
- Fix clippy
