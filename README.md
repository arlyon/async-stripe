# async-stripe

![CI](https://github.com/arlyon/async-stripe/workflows/CI/badge.svg)
[![async-stripe on crates.io](https://img.shields.io/crates/v/async-stripe.svg)](https://crates.io/crates/async-stripe)
[![async-stripe  on docs.rs](https://docs.rs/async-stripe/badge.svg)](https://docs.rs/async-stripe)

Convenient rust bindings and types for the Stripe HTTP API aiming to support
the entire API surface. Not the case? Please open an issue. We update our
definitions [every week](https://github.com/arlyon/async-stripe/actions/workflows/openapi.yml)
to ensure that we are up to date.
Want to see a changelog of the Stripe
API? [Look no further](https://stripe.com/docs/changelog).

## Documentation

See the [Rust API docs](https://docs.rs/async-stripe), the [examples](/examples),
or [payments.rs](https://payments.rs).

## Example

This asynchronous example uses `Tokio` to create
a [Stripe Customer](https://stripe.com/docs/api/customers/object). Your `Cargo.toml` could
look like this:

```toml
tokio = { version = "1", features = ["full"] }
async-stripe = { version = "0.28", features = ["runtime-tokio-hyper"] }
stripe_core = { version = "0.28", features = ["customer"] }
```

And then the code:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_key = "very-secret-key-for-testing";
    let client = stripe::Client::new(secret_key);
    let customer = stripe_core::customer::CreateCustomer {
        name: Some("Test User"),
        email: Some("test@async-stripe.com"),
        description: Some(
            "A fake customer that is used to illustrate the examples in async-stripe.",
        ),
        ..Default::default()
    }
        .send(&client)
        .await?;

    println!("created a customer at https://dashboard.stripe.com/test/customers/{}", customer.id);
    Ok(())
}
```

A full list of examples can be found in the [examples](/examples).

## Relevant Crates

### Stripe Client

The main entry point is the `async-stripe` crate which provides a client for making Stripe
API requests.
`async-stripe` is compatible with the [`async-std`](https://github.com/async-rs/async-std)
and [`tokio`](https://github.com/tokio-rs/tokio) runtimes and the `native-tls`
and `rustls` backends. When adding the dependency, you must select a runtime feature.

#### Installation

```toml
[dependencies]
async-stripe = { version = "0.31", features = ["runtime-tokio-hyper"] }
```

#### Feature Flags

`async-stripe` supports the following features combining runtime and TLS choices:

- `runtime-tokio-hyper`
- `runtime-tokio-hyper-rustls`
- `runtime-tokio-hyper-rustls-webpki`
- `runtime-blocking`
- `runtime-blocking-rustls`
- `runtime-blocking-rustls-webpki`
- `runtime-async-std-surf`

### Stripe Request Crates

To actually make requests with the client, you will have to use crates like `stripe-*`,
which define requests for a subset of the `Stripe` API. For example,
`stripe_connect` includes all requests under the `Connect` part of the sidebar in
the [Stripe API docs](https://stripe.com/docs/api)

The organization of the Stripe API chunks into crates is currently:

- `stripe_billing`: `Billing`
- `stripe_checkout`: `Checkout`
- `stripe_connect`: `Connect`
- `stripe_core`: `Core Resources`
- `stripe_fraud`: `Fraud`
- `stripe_issuing`: `Issuing`
- `stripe_payment`: `Payment Methods` and `Payment Links`
- `stripe_product`: `Products`
- `stripe_terminal`: `Terminal`
- `stripe_treasury`: `Treasury`
- `stripe_misc`: `Tax`, `Identity`, `Reporting`, `Sigma`, `Financial Connections`
  and `Webhooks`

To help minimize compile times, within each of these crates, API requests are gated by
features. For example,
making requests related to [Accounts](https://stripe.com/docs/api/accounts) requires
enabling the `account`
feature flag in `stripe_connect`.

For a full, up-to-date list of where each Stripe API request lives, please
see [this table](crate_info.md)

### Stripe Webhooks

The `stripe_webhook` crate includes functionality for receiving and
validating [Stripe Webhook Events](https://stripe.com/docs/webhooks).
The [examples](/examples) directory includes examples for listening to webhooks
with `axum`, `actix-web`, and `rocket`.

## API Versions

This library always tracks the latest version of the stripe API.

https://github.com/arlyon/async-stripe/blob/f0fd7115aa3b7500134da10f848c8e93ba8eca2e/src/resources/generated/version.rs#L1-L3

If you want to find a version
that matches the API you are on, you can easily navigate back through the git blame in
that file.
Set the corresponding crate version depending on which version of the Stripe API you are
pinned to.
If you don't see the specific version you are on, prefer the next available version.

## MSRV

We currently have `1.86.0` pinned in CI, so any version of rustc newer than that should
work.
If this is not the case, please open an issue. As a policy, we permit MSRV increases in
non-breaking releases.
If you have a compelling usecase for bumping it, we are usually open to do so, as long as
the rust version is not too new (generally 3 releases).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for information on contributing to async-stripe.

## License

This project started as a fork of [stripe-rs](https://github.com/wyyerd/stripe-rs).
We would not be here without them! :)

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE)
  or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
