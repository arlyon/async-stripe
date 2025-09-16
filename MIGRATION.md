# Migration Guide: Upgrading to 1.0 Alpha

> **Note**: We are still expecting a few breaking changes before RC. This guide will be updated as changes are made.

This release overhauls the code generation process to achieve near-complete coverage of the Stripe API 
(https://github.com/arlyon/async-stripe/issues/32). To avoid further bloating compile times, it also includes
crates splitting. Consequently, __there are many breaking changes in this release__. If you have any trouble updating to this release or want help with it, please file an issue! There
will likely be multiple release candidates, so suggestions are welcome for ways to improve ergonomics along the release candidate trail. This release
should not regress compile times, but will not improve them. However, it paves the way for upcoming improvements!

## Request Migration
The main breaking changes center around crate splitting and naming. Please see the [README](README.md#modular-crate-structure) for a
more detailed overview of the new crate structure. In short, instead of one `async-stripe` crate, there is now:
- `async-stripe`: A client for making Stripe requests
- `stripe-*`: A collection of crates implementing requests for different subsets of the Stripe API

So while client initialization is unchanged, making a request requires the following general migration:

### Before
`Cargo.toml`
```toml
async-stripe = { version = "0.28", features = ["runtime-tokio-hyper", "core"] }
```

```rust
use stripe::{Client, CreateCustomer, Customer};

async fn create_customer(client: &Client) -> Result<(), stripe::Error> {
  let customer = Customer::create(
    &client,
    CreateCustomer {
      email: Some("test@async-stripe.com"),
      ..Default::default()
    },
  ).await?;
  Ok(())
}
```

### After
`Cargo.toml`
```toml
async-stripe = { version = "TBD", features = ["runtime-tokio-hyper"] }
# Note the addition of the `customer` feature as well - each object in the Stripe API
# now has its related requests feature-gated.
stripe-core = { version = "TBD", features = ["runtime-tokio-hyper", "customer"] }
```

```rust
use stripe::Client;
use stripe_core::customer::CreateCustomer;

async fn create_customer(client: &Client) -> Result<(), stripe::Error> {
  let customer = CreateCustomer {
    email: Some("test@async-stripe.com"),
    ..Default::default()
  }
          .send(client)
          .await?;
  Ok(())
}
```

The locations where such a migration is necessary are most easily found due to compiler errors on upgrading. Information
on determining the crate a request lives in can be found in the [README](README.md#stripe-request-crates). The 
general steps will be:
1. Find the required crate and feature for the request by [searching here](crate_info.md)
2. Using `Account` and `create` as an example, convert the general structure:
   - `Account::create(&client, CreateAccount::new()).await` to `CreateAccount::new().send(&client).await`
3. Resolve any parameter name changes (with help from autocompletion or the docs). Naming should hopefully be more
consistent and there will no longer be cases of duplicate names (https://github.com/arlyon/async-stripe/issues/154)


## Webhook Improvements
The generated webhook `EventObject` now takes advantage of explicit webhook object types
exposed in the OpenAPI spec. This allows keeping generated definitions up to date with the latest
events more easily. Destructuring `EventObject` now directly gives the inner object type, rather
than requiring matching on both `EventType` and `EventObject`. See the migration example below for how
usage changes.

### Before
```rust
use stripe::{EventType, EventObject};

fn handle_webhook_event(event: stripe::Event) {
  match event.type_ {
    EventType::CheckoutSessionCompleted => {
      if let EventObject::CheckoutSession(session) = event.data.object {
        println!("Checkout session completed! {session:?}");
      } else {
        // How should we handle an unexpected object for this event type?
        println!("Unexpected object for checkout event");
      }
    }
    _ => {},
  } 
}
```

### After
```rust
use stripe_webhook::{Event, EventObject};

fn handle_webhook_event(event: stripe::Event) {
  match event.data.object {
    EventObject::CheckoutSessionCompleted(session) => { 
      println!("Checkout session completed! {session:?}");
    }
    _ => {},
  }
}

```
The same examples for use with `axum`, `actix-web`, and `rocket` can be found in the [examples](/examples) folder.
Instead of importing from `stripe` and enabling the `webhook` feature, you should include the dependency 
`stripe_webhook` and import from that crate instead.

## Generated Type Definitions
- `Deleted<>` objects (such `Deleted<Account>`) no longer expose a boolean `deleted`. This was always `true` and only
used internally as a discriminant when deserializing. The generic type `Deleted` has been removed and replaced by 
generated types such as `DeletedAccount`, which sometimes contain additional fields.
- Optional `List<T>` no longer deserialized with `serde(default)`, the type has been changed `List<T>` -> `Option<List<T>>`. The default
implementation of `List<T>` produced data not upholding the `List<T>` invariant of having a meaningful `url`.
- Types used only for requests now use borrowed types more consistently (and more often). For example, previously the top-level
`CreateCustomer.description` expected `Option<&str>`, but `CreateCustomerShipping.phone` expected `Option<String>`. Now
both expect `Option<&str>`. In general, the following changes are made to request parameters that required owned data:
  - `String` -> `&str`
  - `Metadata` (alias for `HashMap<String, String>`) -> `&HashMap<String, String>`
  - `Vec<>` -> `&[<>]`

## Pagination
The required migration is similar to the [migration](#request-migration) for other requests. The main changes
are around parameter handling to avoid lifetime issues (https://github.com/arlyon/async-stripe/issues/246).

Additionally, obtaining a paginable stream no longer requires a separate request for the first page. `paginate` can
be called directly on `List*` parameters.

### Before
```
let params = ListCustomers::new();
let mut stream = Customer::list(&client, &params).await?.paginate(params);
// ...Any desired stream operations
```

### After
```
let mut stream = ListAccount::new().paginate().stream(&client)
// ...Any desired stream operations
```

## Other Breaking Changes
- Enums no longer implement `Default` (the OpenAPI spec does not specify that a specific variant should be preferred.)
- Some more complex definitions of `*Id` have been relaxed. This should not affect general usage treating
such types as opaque identifiers, but types such as `PaymentSourceId` now just are a newtype wrapper around a `String`. This
change was made to both simplify code generation, and avoid deserialization errors due to missing 
or changed prefix specifications (which was common since the Stripe API does not consider this a [breaking change](https://stripe.com/docs/upgrades#what-changes-does-stripe-consider-to-be-backwards-compatible).
- To minimize compile time, deserialization implementations are limited to where they are necessary. Previously,
some nested parameter types (such as `CreateAccountDocuments`, but not `CreateAccount`) would provide deserialization
implementations. Please let us know if such implementations are useful, and they could be added under a feature flag.
- Types related to the errors Stripe returns now use generated types, which should ensure
they stay up to date, preventing errors error deserialization errors like (https://github.com/arlyon/async-stripe/issues/381) and (https://github.com/arlyon/async-stripe/issues/384)
  - The main user-facing change will be `StripeError::Stripe(RequestError)` -> `StripeError::Stripe(ApiErrors, StatusCode)` since the autogenerated `ApiErrors` 
  does not include the status code.
- The `id` method on `Expandable<T>` now returns a reference: `&T::Id`. All id types implement `Clone` so 
to achieve the previous behavior, use `.id().clone()`. You can also obtain the id without cloning by consuming an `Expandable<T>` with `into_id`.
- `*Id` types no longer derive `default`. The previous default was an empty string, which will never be a valid id
- Removed the `AsRef<str>` implementation for enums, use `as_str` instead.

Since most of these changes are related to code generation, it is likely there are some
breaking changes we missed here. If so, please open an issue (especially for changes that degrade library ergonomics).

## Non-breaking Changes
- `List<>` types now are paginable. This allows usage such as paginating the external accounts returned 
as `List<ExternalAccount>` from retrieving an account.
- `SearchList<>` types are now paginable.
- The `smart-default` dependency was removed.