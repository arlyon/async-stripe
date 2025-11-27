# async-stripe

![CI](https://github.com/arlyon/async-stripe/workflows/CI/badge.svg)
[![async-stripe on crates.io](https://img.shields.io/crates/v/async-stripe.svg)](https://crates.io/crates/async-stripe)
[![async-stripe  on docs.rs](https://docs.rs/async-stripe/badge.svg)](https://docs.rs/async-stripe)

A convenient, performant, and strongly-typed Rust library for the Stripe
HTTP API.

This library provides asynchronous bindings for the entire Stripe API
surface. It's generated directly from Stripe's official OpenAPI
specification and is **updated weekly via CI** to ensure it's always
up-to-date with the latest API changes.

## ✨ Key Features

- **Complete & Current API Coverage**: Generated from the official
  Stripe OpenAPI spec for maximum coverage and correctness. Our CI runs
  weekly to pull in the latest API definitions.

- **Asynchronous & Performant**: Built with async Rust for high
  performance. Uses `miniserde` for blazing-fast deserialization,
  keeping your binary sizes small and compile times fast.

- **Modular by Design**: The API is split into multiple crates
  (`stripe-core`, `stripe-billing`, etc.), so you only compile what you
  need, further reducing bloat.

- **Ergonomic API**: A fluent, builder-style API for constructing
  requests that is easy to use and discover.

- **Flexible HTTP Clients**: Supports `tokio` and `async-std` runtimes
  with `native-tls` and `rustls` backends.

> ⏰ **1.0 Alpha Notice**: We are still expecting a few breaking changes before RC. We recommend using the [Migration Guide](MIGRATION.md) when upgrading from 0.x versions.

## 🚀 Quick Start

Here's a quick example of how to create a new Stripe Customer.

**1. Add dependencies to your `Cargo.toml`:**

You'll need the main `async-stripe` crate for the client and a resource
crate for the APIs you want to use (e.g., `stripe-core` for customers).

    [dependencies]
    async-stripe = "=1.0.0-alpha.8"
    stripe-core = { version = "=1.0.0-alpha.8", features = ["customer"] }
    tokio = { version = "1", features = ["full"] }

**2. Create a Customer:**

The new API uses a builder pattern that flows naturally from request
creation to sending.

    use stripe::Client;
    use stripe_core::customer::CreateCustomer;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // 1. Initialize the client
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
        let client = Client::new(secret_key);

        // 2. Create a customer using the builder pattern
        let customer = CreateCustomer::new()
            .name("Alexander Lyon")
            .email("test@async-stripe.com")
            .description("A fake customer that is used to illustrate the examples in async-stripe.")
            .metadata([(String::from("async-stripe"), String::from("true"))])
            .send(&client)
            .await?;

        println!("Successfully created customer: {} ({})", customer.name.unwrap_or_default(), customer.id);

        Ok(())
    }

## How It Works

### Code Generation

`async-stripe` achieves its vast API coverage and stays current by
generating Rust code directly from the [official Stripe OpenAPI
specification](https://github.com/stripe/openapi "null"). A CI job runs
weekly to fetch the latest spec, regenerate the code, and open a pull
request. This ensures that new Stripe features and updates are available
in the library almost immediately.

### Performance: `serde` and `miniserde`

The Stripe API has a massive surface area, and the code generation
process creates a very large number of types. Using `serde` for both
serialization and deserialization in previous versions resulted in
excessive codegen size and long compile times. To solve this,
`async-stripe` now uses a hybrid approach:

- **`serde`** is used for **serializing** request parameters you send to
  Stripe. Its flexibility and rich feature set make it ideal for
  building complex requests.

- **`miniserde`** is used for **deserializing** API responses from
  Stripe. It's a minimal, high-performance JSON library that
  significantly reduces compile times and final binary size compared to
  a full `serde` dependency.

If you need to use `serde::Deserialize` on the Stripe response types
within your own application, you can enable the `deserialize` feature on
any of the `stripe-*` crates.

### Modular Crate Structure

The Stripe API is extensive. To avoid long compile times and large
dependencies, `async-stripe` is split into several crates. You only need
to include the ones corresponding to the API sections you use.

| API Area | Crate Name | Description |
|----|----|----|
| **Client** | `async-stripe` | The core client for making requests. **Always required.** |
| **Core Resources** | `stripe-core` | Customers, Charges, Payment Intents, Refunds, etc. |
| **Payments** | `stripe-payment` | Payment Methods, Payment Links, Sources. |
| **Billing & Subscriptions** | `stripe-billing` | Invoices, Subscriptions, Plans, Quotes. |
| **Stripe Connect** | `stripe-connect` | Accounts, Account Links, Transfers. |
| **Fraud Detection** | `stripe-fraud` | Radar, Reviews. |
| **Checkout** | `stripe-checkout` | Checkout Sessions. |
| **Webhooks** | `stripe-webhook` | Securely receive and deserialize webhook events. |
| **Products & Pricing** | `stripe-product` | Products, Prices, Coupons, Tax Rates. |
| **Issuing** | `stripe-issuing` | Card creation and management for Stripe Issuing. |
| **Terminal** | `stripe-terminal` | In-person payments with Stripe Terminal. |
| **Treasury** | `stripe-treasury` | Financial accounts and money movement. |
| **Miscellaneous** | `stripe-misc` | Tax, Identity, Reporting, Sigma, etc. |

> For a complete, up-to-date mapping of every API object to its crate
> and feature flag, see the
> [`crate_info.md`](https://www.google.com/search?q=crate_info.md "null")
> file in the repository. We are actively working on improving API
> surface exploration. For now, you can explore the `generated/`
> directory in the repository to see all available API crates and their
> respective modules.

## Usage Guide

### 1. Choose Your HTTP Client

The core `async-stripe` crate provides the client. You must enable a
feature flag to select your desired async runtime and TLS backend.

| Feature Flag | Async Runtime | HTTP Client | TLS Backend |
|----|----|----|----|
| `default-tls` (Default) | `tokio` | `hyper` | `native-tls` |
| `rustls-tls-webpki-roots` | `tokio` | `hyper` | `rustls` (from `webpki-roots`) |
| `rustls-tls-native` | `tokio` | `hyper` | `rustls` (from native cert store) |
| `async-std-surf` | `async-std` | `surf` | `native-tls` |
| `blocking` | Sync (uses `tokio` internally) | `hyper` | Depends on active TLS feature |

### 2. Add Resource Crates and Features

For each Stripe resource you want to interact with, add its crate and
enable the corresponding feature flag. For example, to use `Customer`
and `Charge` objects:

    # Cargo.toml
    [dependencies]
    # ...
    stripe-core = { version = "...", features = ["customer", "charge"] }

### 3. Pagination

Listing resources is now more ergonomic. You can call `.paginate()`
directly on a `List*` request struct to get a stream of results.

    use stripe_core::customer::ListCustomer;
    use futures_util::TryStreamExt;

    # async fn run(client: &stripe::Client) -> Result<(), stripe::StripeError> {
    let mut stream = ListCustomer::new().paginate().stream(client);
    while let Some(customer) = stream.try_next().await? {
        println!("Got customer: {}", customer.id);
    }
    # Ok(())
    # }

### 4. Handling Webhooks

Webhook handling is now isolated in the `stripe-webhook` crate and the
API is much simpler. You can now match directly on the event object
type.

    use stripe_webhook::{Event, EventObject, Webhook};

    fn handle_webhook(payload: &str, sig: &str, secret: &str) {
        let event = Webhook::construct_event(payload, sig, secret).unwrap();
        match event.data.object {
            EventObject::CheckoutSessionCompleted(session) => {
                println!("Received checkout session completed webhook with id: {:?}", session.id);
            }
            EventObject::AccountUpdated(account) => {
                println!("Received account updated webhook for account: {:?}", account.id);
            }
            _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
        }
    }

## Project Status

This library is actively maintained. The code is regenerated weekly to
ensure it never falls behind the official Stripe API.

We are working on setting up fully automated releases to `crates.io` to
coincide with the weekly updates. This will ensure you can depend on the
latest Stripe features as soon as they are available.

## Contributing

Contributions are welcome! Please see
[CONTRIBUTING.md](https://www.google.com/search?q=CONTRIBUTING.md "null")
for details on how to get started, run tests, and contribute to the code
generation process.

## License

This project is licensed under either of:

- Apache License, Version 2.0
  ([LICENSE-APACHE](https://www.google.com/search?q=LICENSE-APACHE "null")
  or http://www.apache.org/licenses/LICENSE-2.0)

- MIT license
  ([LICENSE-MIT](https://www.google.com/search?q=LICENSE-MIT "null") or
  http://opensource.org/licenses/MIT)

This project began as a fork of
[stripe-rs](https://github.com/wyyerd/stripe-rs "null"), and we are
incredibly grateful for their foundational work.
