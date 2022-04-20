# async-stripe

![CI](https://github.com/arlyon/async-stripe/workflows/CI/badge.svg)
[![async-stripe on crates.io](https://img.shields.io/crates/v/async-stripe.svg)](https://crates.io/crates/async-stripe)
[![async-stripe  on docs.rs](https://docs.rs/async-stripe/badge.svg)](https://docs.rs/async-stripe)

Convenient rust bindings and types for the Stripe HTTP API aiming to support
the entire API surface. Not the case? Please open an issue.

## Documentation

See the [rust api docs](https://docs.rs/async-stripe), the [examples](/examples), or [payments.rs](https://payments.rs).

## Installation

`async-stripe` is compatible with the [`async-std`](https://github.com/async-rs/async-std) and [`tokio`](https://github.com/tokio-rs/tokio) runtimes and the `native-tls` and `rustls` backends. When adding the dependency, you much select a runtime feature.

```toml
[dependencies]
async-stripe = { version = "0.14", features = ["runtime-tokio-hyper"] }
```

## Feature Flags

### Runtimes

- `runtime-tokio-hyper`
- `runtime-tokio-hyper-rustls`
- `runtime-blocking`
- `runtime-blocking-rustls`
- `runtime-async-std-surf`

### API Features

Additionally, since this is a large library, it is possible to conditionally
enable features as required to reduce compile times and final binary size.
Refer to the [Stripe API docs](https://stripe.com/docs/api) to determine
which APIs are included as part of each feature flag.

```toml
# Example: Core-only (enough to create a `Charge` or `Card` or `Customer`)
async-stripe = { version = "*", default-features = false, features = ["runtime-async-std-surf"] }

# Example: Support for "Subscriptions" and "Invoices"
async-stripe = { version = "*", default-features = false, features = ["runtime-async-std-surf", "billing"] }
```

## API Versions

The latest supported version of the Stripe API is `2020-08-27`.
Set the corresponding crate version depending on which version of the Stripe API you are pinned to.
If you don't see the specific version you are on, prefer the next available version.

- `0.14` - stripe version `2020-08-27`
- `0.12` - stripe version `2019-09-09`

## MSRV

We currently have `1.54.0` pinned in CI, so any version of rustc newer than that should work.
If this is not the case, please open an issue. As a policy, we permit MSRV increases in non-breaking releases.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for information on contributing to async-stripe.

## License

This project started as a fork of [stripe-rs](https://github.com/wyyerd/stripe-rs).
We would not be here without them! :)

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
