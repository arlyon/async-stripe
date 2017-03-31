stripe-rust
===========

[![stripe-rust on Travis CI](https://travis-ci.org/rapiditynetworks/stripe-rust.svg?branch=master)](https://travis-ci.org/rapiditynetworks/stripe-rust)
[![stripe-rust on crates.io](https://img.shields.io/crates/v/stripe-rust.svg)](https://crates.io/crates/stripe-rust)
[![stripe-rust on docs.rs](https://docs.rs/stripe-rust/badge.svg)](https://docs.rs/stripe-rust)

Rust API bindings for the Stripe v1 HTTP API

## Usage
Put this in your `Cargo.toml`:

```toml
[dependencies]
stripe-rust = "0.3.1"
```

And this in your crate root:

```rust
extern crate stripe;
```

## HTTPS Options
This crate uses [native-tls](https://github.com/sfackler/rust-native-tls) and
[hyper-native-tls](https://github.com/sfackler/hyper-native-tls/blob/master/Cargo.toml)
by default to communicate with the Stripe API over HTTPS.

### Native TLS
Put this in your `Cargo.toml` to explicitly prefer `native-tls` if the default changes:

```toml
[dependencies.stripe-rust]
version = "0.3.1"
features = ["with-native-tls"]
default-features = false
```

### OpenSSL
Put this in your `Cargo.toml` to use [openssl](https://github.com/sfackler/hyper-openssl)
and [hyper-openssl](https://github.com/sfackler/hyper-openssl) instead of the default:

```toml
[dependencies.stripe-rust]
version = "0.3.1"
features = ["with-openssl"]
default-features = false
```
