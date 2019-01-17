# stripe-rust

[![stripe-rust on Travis CI](https://travis-ci.org/wyyerd/stripe-rs.svg?branch=master)](https://travis-ci.org/wyyerd/stripe-rs)
[![stripe-rust on crates.io](https://img.shields.io/crates/v/stripe-rust.svg)](https://crates.io/crates/stripe-rust)
[![stripe-rust on docs.rs](https://docs.rs/stripe-rust/badge.svg)](https://docs.rs/stripe-rust)

Rust API bindings for the Stripe v1 HTTP API.

This is compatible with all currently supported versions of Stripe's client-side
libraries including https://js.stripe.com/v2/ and https://js.stripe.com/v3/.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
stripe-rust = "0.9.0"
```

And this in your crate root:

```rust
extern crate stripe;
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
  let mut params = stripe::ChargeParams::default();
  // NOTE: Stripe represents currency in the lowest denominations (e.g. cents)
  params.amount = Some(1095); // e.g. $10.95
  params.source = Some(stripe::PaymentSourceParams::Token(token));

  // Example: Override currency to be in Canadian Dollars
  params.currency = Some(stripe::Currency::CAD);
  let charge = stripe::Charge::create(&client, params).unwrap();
  println!("{:?}", charge); // =>  Charge { id: "ch_12345", amount: 1095, .. }
```

```rust
  /* Listing Stripe Charges */

  let params = stripe::ChargeListParams::default();
  let charges = stripe::Charge::list(&client, params).unwrap();
  println!("{:?}", charges); // =>  List { data: [Charge { id: "ch_12345", .. }] }
```

### Using Custom Connect accounts

This crate supports impersonating a custom connect account.

To impersonate the account get a new Client and pass in the account id.

```rust
  let client = client.with_headers(stripe::Headers {
    stripe_account: Some("acct_ABC".to_string()),
    client_id: Some("ca_XYZ".to_string())
  });

  // Then, all requests can be made normally
  let params = stripe::CustomerListParams::default();
  let customers = stripe::Customer::list(&client, params).unwrap();
  println!("{:?}", customers); // =>  List { data: [Customer { .. }] }
```
