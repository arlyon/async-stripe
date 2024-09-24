# async-stripe

![CI](https://github.com/arlyon/async-stripe/workflows/CI/badge.svg)
[![async-stripe on crates.io](https://img.shields.io/crates/v/async-stripe.svg)](https://crates.io/crates/async-stripe)
[![async-stripe  on docs.rs](https://docs.rs/async-stripe/badge.svg)](https://docs.rs/async-stripe)

Convenient rust bindings and types for the Stripe HTTP API aiming to support
the entire API surface. Not the case? Please open an issue. We update our
definitions [every week](https://github.com/arlyon/async-stripe/actions/workflows/openapi.yml) to ensure that we are up to date.
Want to see a changelog of the Stripe API? [Look no further](https://stripe.com/docs/changelog).

> **Note**
>
> We are currently working on a major rewrite of the library in the `next` branch. This rewrite aims to make the library more efficient and easier to use. Some lovely numbers from the rewrite:
>
> - A clean release build of `examples/endpoints` goes from ~4m to 50s with `min-ser` enabled.
> - The actual time to build just the binary goes from 75s to 7s, making incremental builds for code depending on async-stripe much faster.
> - Stripped binary size of the `examples/endpoints` binary went from ~70MB to ~20MB, with further reduction to ~13MB using a fat LTO build.
>
> We are actively seeking testers to help us ensure the new version is stable and performant. If you are interested in trying out the new version, you can add the following to your `Cargo.toml`:
>
> ```toml
> [dependencies]
> async-stripe = { git = "https://github.com/arlyon/async-stripe", branch = "next" }
> ```
>
> Your feedback is invaluable to us, so please report any issues or suggestions you may have. We are still expecting a few breaking changes before RC.

## Documentation

See the [Rust API docs](https://docs.rs/async-stripe), the [examples](/examples), or [payments.rs](https://payments.rs).

## Installation

`async-stripe` is compatible with the [`async-std`](https://github.com/async-rs/async-std) and [`tokio`](https://github.com/tokio-rs/tokio) runtimes and the `native-tls` and `rustls` backends. When adding the dependency, you must select a runtime feature.

```toml
[dependencies]
async-stripe = { version = "0.31", features = ["runtime-tokio-hyper"] }
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

This library always tracks the latest version of the stripe API.

https://github.com/arlyon/async-stripe/blob/f0fd7115aa3b7500134da10f848c8e93ba8eca2e/src/resources/generated/version.rs#L1-L3

If you want to find a version
that matches the API you are on, you can easily navigate back through the git blame in that file.
Set the corresponding crate version depending on which version of the Stripe API you are pinned to.
If you don't see the specific version you are on, prefer the next available version.


## MSRV

We currently have `1.78.0` pinned in CI, so any version of rustc newer than that should work.
If this is not the case, please open an issue. As a policy, we permit MSRV increases in non-breaking releases.
If you have a compelling usecase for bumping it, we are usually open to do so, as long as
the rust version is not too new (generally 3 releases).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for information on contributing to async-stripe.

## License

This project started as a fork of [stripe-rs](https://github.com/wyyerd/stripe-rs).
We would not be here without them! :)

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
