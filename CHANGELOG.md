# Version 0.4.1 (August 8, 2017)

## Changes

 * Add RangeQuery struct for created/date filters in list requests 
 * Implement `Customer::list` request (ie. "GET /customers"); Thanks @pocket7878
 * Add missing params to `InvoiceListParams` for `Invoice::list`

# Version 0.4.0 (August 2, 2017)

## Breaking Changes

 * Upgrade Serde to v1.0

# Version 0.3.3 (August 1, 2017)

## Changes

 * Add support for authentication using the Stripe-Account header
 * Implement `Event` struct for implementing a Stripe Webhook (only some events)
 * Implement `Invoice::update` request (ie. POST "/invoices/{invoice_id}")
 * Implement `InvoiceItem::create` request (ie. POST "/invoiceitems")
 * Fix `Subscription::cancel` request (previously used wrong method/path)

# Version 0.3.2 (June 17, 2017)

## Changes

 * Implemented the `Invoice::list` API (ie. "/invoices")
 * Fixed a request encoding error for `Invoice::pay` (ie. "/invoices/{invoice_id}/pay")
 * Fixed fields on `Subscription` that could be null but were missing `Optional<..>`
 * Added missing `last4` field to `Card` resource
 * Changed the repository URL

# Version 0.3.1 (Mar 31, 2017)

## Changes

 * Made rust docs slightly easier to read
 * Fixed incorrect method name for `Plan` resource
 * Use `native-tls` by default and add feature-flag supporting `openssl`

# Version 0.3.0 (Mar 28, 2017)

## New Features

  * Implemented the [charges resource](https://stripe.com/docs/api#charges) and
    added the `Charge` type.

## Breaking Changes

  * The `Resource::get` requests have been renamed to `Resource::retrieve`.
  * The `Subscription::cancel` request now expects `CancelParams` instead of
    expecting a one-off function argument.
  * Removed the `blame` method and `Blame` type from errors.
  * The `Error::Decode` and `Error::Encode` variants have been combined into a
    single `Error::Conversion` variant.

## General Improvements

 * The `Error` type now has easier to read error messages and will parses JSON
   errors returned by Stripe into a structured error type (`RequestError`).
 * Added Travis CI for automated testing on GitHub.
 * Added reference documentation to implemented resources (but still no docs at
   crate/module level nor a "Getting started" guide)


# Version 0.2.0 (Mar 17, 2017)

## New Features

  * Added `Client` which is created once with a stripe private key and is
    intended to be re-used for multiple requests. It implements `Sync` so you
    can share it among multiple threads.
  * Added new strongly-typed `Currency` type following the example of
    https://github.com/stripe/stripe-go.
  * `Customer::create`/`CustomerParams` now support using `CardParams` as a
    source instead of just tokens.
  * Implemented the [sources API](https://stripe.com/docs/api#sources) in the `Source` type.

## Breaking Changes

  * All `Params` types now use `&str` fields instead of `String`s.
  * Requests used to require a `stripe_key: &str` as their final argument but
    now use a `&Client` as the first argument instead.
  * Stripe tokens used to be directly used as a source in `CustomerParams` but
    now must be used with `CustomerSource::Token("tok_xyzABC123")`.

## General Improvements

  * Added remaining fields to `Params` types after switching from
    `serde_urlencoded` to `serde_qs` to support nested params.
  * Added the `create_customer` example
  * Types that implement `Deserialize` now also implement `Debug`.
  * Types that implement `Serialize` now also implement `Default`.
