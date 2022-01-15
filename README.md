# async-stripe

![CI](https://github.com/arlyon/stripe-rs/workflows/CI/badge.svg)
[![async-stripe on crates.io](https://img.shields.io/crates/v/async-stripe.svg)](https://crates.io/crates/async-stripe)
[![async-stripe  on docs.rs](https://docs.rs/async-stripe/badge.svg)](https://docs.rs/async-stripe)

Rust API bindings for the Stripe v1 HTTP API.

This is compatible with all currently supported versions of Stripe's client-side
libraries including https://js.stripe.com/v2/ and https://js.stripe.com/v3/.

## API Version

The latest supported version of the Stripe API is `2020-08-27`.
Set the corresponding crate version depending on which version of the Stripe API you are pinned to.
If you don't see the specific version you are on, prefer the next available version.

- `0.13` - stripe version `2020-08-27`
- `0.12` - stripe version `2019-09-09`

## Install

`async-stripe` is compatible with the [`async-std`](https://github.com/async-rs/async-std) and [`tokio`](https://github.com/tokio-rs/tokio) runtimes and the `native-tls` and `rustls` backends. When adding the dependency, you much select a runtime feature.

```toml
[dependencies]
async-stripe = { version = "0.13", features = ["runtime-async-std-surf"] }
```

### Feature Flags

#### Runtimes

- `runtime-tokio-hyper`
- `runtime-tokio-hyper-rustls`
- `runtime-blocking`
- `runtime-blocking-rustls`
- `runtime-async-std-surf`

#### API Features

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

## Getting Started

```rust
/* Create a Stripe Client */

let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");

/* Create a Stripe Charge */

let token = "TOKEN_FROM_CHECKOUT".parse().expect("token to be valid");
let mut params = stripe::CreateCharge::new();

// NOTE: Stripe represents currency in the lowest denominations (e.g. cents)
params.amount = Some(1095); // e.g. $10.95
params.source = Some(stripe::ChargeSourceParams::Token(token));

// Example: Override currency to be in Canadian Dollars
params.currency = Some(stripe::Currency::CAD);

let charge = stripe::Charge::create(&client, params).unwrap();
println!("{:?}", charge); // =>  Charge { id: "ch_12345", amount: 1095, .. }


/* List Stripe Charges */

let params = stripe::ListCharges::new();
let charges = stripe::Charge::list(&client, params).unwrap();
println!("{:?}", charges); // =>  List { data: [Charge { id: "ch_12345", .. }] }
```

Most requests for creating or updating a Stripe object use the same Rust struct,
so you may frequently need to refer to the [official API docs](https://stripe.com/docs/api)
to determine which fields are required for either request.

### Using Custom Connect accounts

This crate supports impersonating a custom connect account. To impersonate the account get a new Client and pass in the account id.

```rust
let mut headers = stripe::Headers::default();
headers.stripe_account = Some("acct_ABC".to_string());
headers.client_id = Some("ca_XYZ".to_string());
let client = client.with_headers(headers);

// Then, all requests can be made normally
let params = stripe::CustomerListParams::default();
let customers = stripe::Customer::list(&client, params).unwrap();
println!("{:?}", customers); // =>  List { data: [Customer { .. }] }
```

## MSRV

We currently have `1.49.0` pinned in CI, so any version of rustc newer than that should work. If this is not the case, please open an issue.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for information on contributing to async-stripe.

## License

This project started as a fork of [stripe-rs](https://github.com/wyyerd/stripe-rs).
We would not be here without them! :)

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
