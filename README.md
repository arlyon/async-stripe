# stripe-rust

[![stripe-rust on Travis CI](https://travis-ci.org/wyyerd/stripe-rs.svg?branch=master)](https://travis-ci.org/wyyerd/stripe-rs)
[![stripe-rust on crates.io](https://img.shields.io/crates/v/stripe-rust.svg)](https://crates.io/crates/stripe-rust)
[![stripe-rust on docs.rs](https://docs.rs/stripe-rust/badge.svg)](https://docs.rs/stripe-rust)

Rust API bindings for the Stripe v1 HTTP API.

This is compatible with all currently supported versions of Stripe's client-side
libraries including https://js.stripe.com/v2/ and https://js.stripe.com/v3/.

## API Version
The latest supported version of the Stripe API is w`2020-03-02`.
Set the corresponding crate version depending on which version of the Stripe API you are pinned to.
If you don't see the specific version you are on, prefer the next available version.

 - `0.13.*` - stripe version `2020-03-02` (not yet published)
 - `0.12.*` - stripe version `2019-09-09`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
stripe-rust = "0.13.*"
```

To see how the library is used, look through the [examples](examples) folder.

## Getting Started

To get started, we need to create a client:

```rust
  let client = stripe::Client::new("sk_test_YOUR_STRIPE_SECRET");
```

Then we can begin making requests as we'd like. Most Stripe requests accept
many optional parameters, so we usually get the `::default()` params and then
set the ones we want from there.

Most requests for creating or updating a Stripe object use the same Rust struct,
so you may frequently need to refer to the [official API docs](https://stripe.com/docs/api)
to determine which fields are required for either request.

```rust
  /* Creating a Stripe Charge */

  let token = "TOKEN_FROM_CHECKOUT".parse().expect("token to be valid");
  let mut params = stripe::CreateCharge::new();

  // NOTE: Stripe represents currency in the lowest denominations (e.g. cents)
  params.amount = Some(1095); // e.g. $10.95
  params.source = Some(stripe::ChargeSourceParams::Token(token));

  // Example: Override currency to be in Canadian Dollars
  params.currency = Some(stripe::Currency::CAD);

  let charge = stripe::Charge::create(&client, params).unwrap();
  println!("{:?}", charge); // =>  Charge { id: "ch_12345", amount: 1095, .. }
```

```rust
  /* Listing Stripe Charges */

  let params = stripe::ListCharges::new();
  let charges = stripe::Charge::list(&client, params).unwrap();
  println!("{:?}", charges); // =>  List { data: [Charge { id: "ch_12345", .. }] }
```

### Using Custom Connect accounts

This crate supports impersonating a custom connect account.

To impersonate the account get a new Client and pass in the account id.

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

### Feature Flags
By default the `full` stripe api is enabled.

To reduce code size, disable default features and enable just the APIs you use:

```toml
# Example: Core-only (enough to create a `Charge` or `Card` or `Customer`)
stripe-rust = { version = "*", default-features = false, features = ["default-tls"] }

# Example: Support for "Subscriptions" and "Invoices"
stripe-rust = { version = "*", default-features = false, features = ["default-tls", "billing"] }
```

Refer to the [Stripe API docs](https://stripe.com/docs/api) to determine
which APIs are included as part of each feature flag.

## Contributing

### Code Generation
This library is (mostly) authored via code generation by parsing
the OpenAPI specification for Stripe.

To update the generated code, use the included scripts:

```sh
# Generate files into the ./openapi/out directort (working directory must be project root)
> ./openapi/build

# Copy reviewed files to ./src/resources
> ./openapi/commit
```
