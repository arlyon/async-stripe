# [0.40.0](https://github.com/arlyon/async-stripe/compare/v0.39.2...v0.40.0) (2024-09-26)


### Features

* export checkout_session_ext ([e4e7220](https://github.com/arlyon/async-stripe/commit/e4e722007a5164413054a958ea13858afbccaafd)), closes [#614](https://github.com/arlyon/async-stripe/issues/614)

## [0.39.2](https://github.com/arlyon/async-stripe/compare/v0.39.1...v0.39.2) (2024-09-24)


### Bug Fixes

* erroneous deref clippy lint ([1fcbb29](https://github.com/arlyon/async-stripe/commit/1fcbb2958b4b7900f2463cedf2375179411a2333))

## [0.39.1](https://github.com/arlyon/async-stripe/compare/v0.39.0...v0.39.1) (2024-09-03)


### Bug Fixes

* correct docs host and path for API references ([539836d](https://github.com/arlyon/async-stripe/commit/539836d01904742110171a9d6ef877b4cc3f87a3))

# [0.39.0](https://github.com/arlyon/async-stripe/compare/v0.38.1...v0.39.0) (2024-08-31)


### Bug Fixes

* `post` -> `post_form` ([bc39260](https://github.com/arlyon/async-stripe/commit/bc3926039eb50d48cf8f20ddb8110c847e7f7f00))
* Move `promotion_code_ext` to `products` feature ([32435f3](https://github.com/arlyon/async-stripe/commit/32435f38c242ad47d8c992d1178736c460f107f2))
* Re-export ([24f1782](https://github.com/arlyon/async-stripe/commit/24f17823b1812886bda32762f9de66dceb83559b))


### Features

* Create promotion code ([0fde9eb](https://github.com/arlyon/async-stripe/commit/0fde9eb6bc3333bab3bb135709697d114017be4a))

## [0.38.1](https://github.com/arlyon/async-stripe/compare/v0.38.0...v0.38.1) (2024-08-06)


### Bug Fixes

* [#578](https://github.com/arlyon/async-stripe/issues/578) allow arbitrary strings for priceId ([a16bc6e](https://github.com/arlyon/async-stripe/commit/a16bc6e80c1a5e87bf376cbfd6b1f2a8caef992e))

# [0.38.0](https://github.com/arlyon/async-stripe/compare/v0.37.3...v0.38.0) (2024-07-31)


### Features

* add support for TestClock operations ([d792798](https://github.com/arlyon/async-stripe/commit/d792798c3f027e0c57b132ddf168dbd03fcdd926)), closes [#574](https://github.com/arlyon/async-stripe/issues/574)

## [0.37.3](https://github.com/arlyon/async-stripe/compare/v0.37.2...v0.37.3) (2024-07-29)


### Bug Fixes

* linting issue for Rust 1.80 ([9232213](https://github.com/arlyon/async-stripe/commit/9232213c0665622c91b328d1b2ff20e7f9ff7357))

## [0.37.2](https://github.com/arlyon/async-stripe/compare/v0.37.1...v0.37.2) (2024-07-23)


### Bug Fixes

* rtx id prefix ([67ea232](https://github.com/arlyon/async-stripe/commit/67ea2325ba10fd0bc1b5c9e3a3436738caf4a98c))

## [0.37.1](https://github.com/arlyon/async-stripe/compare/v0.37.0...v0.37.1) (2024-05-24)


### Bug Fixes

* Leftover clippy warnings ([888307d](https://github.com/arlyon/async-stripe/commit/888307d23d852ebc2f453e788e7fd682efb9dd6f))
* Run clippy on openapi generator ([c63c197](https://github.com/arlyon/async-stripe/commit/c63c197e7cd6f73c0183345c82be77ef2a4c06f5))

# [0.37.0](https://github.com/arlyon/async-stripe/compare/v0.36.0...v0.37.0) (2024-04-30)


### Bug Fixes

* **customer-ext,PM query:** type field is optional ([362659e](https://github.com/arlyon/async-stripe/commit/362659e3e7ace2706fae172e7936efdc5c17929d))


### Features

* **setup_intent:** add mandate_data for confirm ([51e34d6](https://github.com/arlyon/async-stripe/commit/51e34d6fec2238d27e937f361d9e0b60f1bc564c))
* **setup_intent:** verify microdeposits flow ([15b3663](https://github.com/arlyon/async-stripe/commit/15b3663336561ccf55d78dc66668d28627b4d59b))

# [0.36.0](https://github.com/arlyon/async-stripe/compare/v0.35.3...v0.36.0) (2024-04-30)


### Features

* Add `retrieve_line_items` function for `CheckoutSession` ([5c74b3b](https://github.com/arlyon/async-stripe/commit/5c74b3b13fc30e7f81191c43016ab46c3d8ef9d6))

## [0.35.3](https://github.com/arlyon/async-stripe/compare/v0.35.2...v0.35.3) (2024-04-26)


### Bug Fixes

* Clippy append instead of extend ([1a85b9b](https://github.com/arlyon/async-stripe/commit/1a85b9b9ea3dc940ea86434d3ef9a2a6755b6160))
* Clippy explicit ToString implementation ([15250a9](https://github.com/arlyon/async-stripe/commit/15250a96d19ec142693761c3d4a455a35b1ae780))
* Clippy panic warnings ([552aff2](https://github.com/arlyon/async-stripe/commit/552aff238dd16135d7645f1692b891c75cb0459b))
* Clippys from generated code ([89cce5b](https://github.com/arlyon/async-stripe/commit/89cce5ba442a528939aa54c455eefd9ad27f8b58))
* More clippy warnings ([6de23bb](https://github.com/arlyon/async-stripe/commit/6de23bbee3713cc54bc148513a6c1331f0fb5998))
* Remove useless reference that causes clippy warnings ([2fca98b](https://github.com/arlyon/async-stripe/commit/2fca98be10ba9f5a746d04a46aa7ef1532c00056))

## [0.35.2](https://github.com/arlyon/async-stripe/compare/v0.35.1...v0.35.2) (2024-04-26)


### Bug Fixes

* Clippy warnings ([842bce6](https://github.com/arlyon/async-stripe/commit/842bce6d94654ca0a9ea0a52c899f98055ab531b))
* Implement workaround for ace in the verify-codegen action ([79c1f6f](https://github.com/arlyon/async-stripe/commit/79c1f6f6da182ab80fd02b9e1662aeec5915ec12))
* Semantic release needs minimal rust version ([642ea31](https://github.com/arlyon/async-stripe/commit/642ea3193944048670619fb60ca3d4a973c439e7))

## [0.35.1](https://github.com/arlyon/async-stripe/compare/v0.35.0...v0.35.1) (2024-04-26)


### Bug Fixes

* bump MSRV (closes [#532](https://github.com/arlyon/async-stripe/issues/532)) ([1379a90](https://github.com/arlyon/async-stripe/commit/1379a900090e59ee05611ddfb3b8842261a761e2))
* Unused import ([baf0b98](https://github.com/arlyon/async-stripe/commit/baf0b98a60ff335a0760f3d8e785715d36d044f7))

# [0.35.0](https://github.com/arlyon/async-stripe/compare/v0.34.2...v0.35.0) (2024-04-04)


### Bug Fixes

* add products to features list in cargo.toml and export from resourses ([d26860f](https://github.com/arlyon/async-stripe/commit/d26860f9049dd3a9ef934ad1a5f5ed3e6b365e70))
* export SearchList ([a427943](https://github.com/arlyon/async-stripe/commit/a42794314a5c41b34c349835e705d890cbc62d3e))
* Remove "Some()" from RequestError's display ([e609cad](https://github.com/arlyon/async-stripe/commit/e609cadb1321cd097d9a2fb9fc23d87a13eff1f5))
* rename redirect_url to return_url on ConfirmSetupIntent ([9da5a28](https://github.com/arlyon/async-stripe/commit/9da5a28864215967d39ddb432cc593820a797a9e)), closes [#504](https://github.com/arlyon/async-stripe/issues/504)


### Features

* **webhook_events:** support construct_event with timestamp ([88fe501](https://github.com/arlyon/async-stripe/commit/88fe501f3b735b87b10f80acb227bd05765355b8))

## [0.34.2](https://github.com/arlyon/async-stripe/compare/v0.34.1...v0.34.2) (2024-02-21)


### Bug Fixes

* correctly generate types for optional lists ([63732cc](https://github.com/arlyon/async-stripe/commit/63732cc37f4bc20b4e81a671c11c904a1a600c60))

## [0.34.1](https://github.com/arlyon/async-stripe/compare/v0.34.0...v0.34.1) (2024-02-15)


### Bug Fixes

* fixes an issue with missing renames of Self_ ([9c58038](https://github.com/arlyon/async-stripe/commit/9c5803812af379a5265946afe4c177d3fcd0370e))

# [0.34.0](https://github.com/arlyon/async-stripe/compare/v0.33.1...v0.34.0) (2024-02-15)


### Features

* generate latest changes from OpenApi spec ([a3b0e4d](https://github.com/arlyon/async-stripe/commit/a3b0e4d84c99068e274d8ed7f319d4cc1d3db0cd))

## [0.33.1](https://github.com/arlyon/async-stripe/compare/v0.33.0...v0.33.1) (2024-02-08)


### Bug Fixes

* expose the `auto_advance` field from the `FinalizeInvoiceParams` struct ([ae1a807](https://github.com/arlyon/async-stripe/commit/ae1a807e202f32f3ff8405fdbc1d14aca44c41e8))

# [0.33.0](https://github.com/arlyon/async-stripe/compare/v0.32.0...v0.33.0) (2024-02-02)


### Bug Fixes

* support pdp_ as an id prefix for dispute objects ([be506d6](https://github.com/arlyon/async-stripe/commit/be506d6fc6fa41dd6085d3887abbadbef069432c))


### Features

* generate latest changes from OpenApi spec ([29a457d](https://github.com/arlyon/async-stripe/commit/29a457d280f25ebd7d95e80ad5a4af4172e05682))

# [0.32.0](https://github.com/arlyon/async-stripe/compare/v0.31.2...v0.32.0) (2024-01-27)


### Features

* generate latest changes from OpenApi spec ([19ac377](https://github.com/arlyon/async-stripe/commit/19ac377c82c8b52a44092a3f93174917983d166f))

## [0.31.2](https://github.com/arlyon/async-stripe/compare/v0.31.1...v0.31.2) (2024-01-24)


### Bug Fixes

* **currency:** add BYN and MMK to currency list ([03cddce](https://github.com/arlyon/async-stripe/commit/03cddce1a9c20c25b531aab47e6e47254b6bef60))

## [0.31.1](https://github.com/arlyon/async-stripe/compare/v0.31.0...v0.31.1) (2024-01-24)


### Bug Fixes

* **pagination:** prevent infinite loop caused by clone ([bc20bd4](https://github.com/arlyon/async-stripe/commit/bc20bd46f67ffb384d67d0605cb5126e1f3d6333))

# [0.31.0](https://github.com/arlyon/async-stripe/compare/v0.30.1...v0.31.0) (2024-01-24)


### Features

* generate latest changes from OpenApi spec ([9b3a844](https://github.com/arlyon/async-stripe/commit/9b3a844a94c090fd66ce7d223ccd8ddcd93a5c45))

## [0.30.1](https://github.com/arlyon/async-stripe/compare/v0.30.0...v0.30.1) (2024-01-24)


### Bug Fixes

* **codegen:** generate terminal resource objects ([bf7e117](https://github.com/arlyon/async-stripe/commit/bf7e1178d61d50a83e21199d0c67bc99b5d04f60))

# [0.30.0](https://github.com/arlyon/async-stripe/compare/v0.29.0...v0.30.0) (2024-01-24)


### Bug Fixes

* avoid generating nested currency options ([f020df4](https://github.com/arlyon/async-stripe/commit/f020df4feb4fee89366b5bc15cc596caec697f78))
* typo in checkout.rs ([c55541b](https://github.com/arlyon/async-stripe/commit/c55541b1ee96d91eb29ef900f7292f1337a7c3c3))


### Features

* generate latest changes from OpenApi spec ([d0cbc71](https://github.com/arlyon/async-stripe/commit/d0cbc718a980aedee4b191a401e76a958c3e4bb5))

# [0.29.0](https://github.com/arlyon/async-stripe/compare/v0.28.1...v0.29.0) (2024-01-10)


### Features

* generate latest changes from OpenApi spec ([32e802e](https://github.com/arlyon/async-stripe/commit/32e802ef4d36f8ab495e7c2b6abba29450eb999f))

## [0.28.1](https://github.com/arlyon/async-stripe/compare/v0.28.0...v0.28.1) (2024-01-10)


### Bug Fixes

* add plan prefix to PriceId ([6655058](https://github.com/arlyon/async-stripe/commit/6655058a476adc6551643a947308f13e8ba320d3))

# [0.28.0](https://github.com/arlyon/async-stripe/compare/v0.27.0...v0.28.0) (2023-12-08)


### Features

* generate latest changes from OpenApi spec ([4c25649](https://github.com/arlyon/async-stripe/commit/4c256490d60e8d1e158f8f6ed7300a766c1f449d))

# [0.27.0](https://github.com/arlyon/async-stripe/compare/v0.26.0...v0.27.0) (2023-12-01)


### Bug Fixes

* add support for atxi id prefix ([414d534](https://github.com/arlyon/async-stripe/commit/414d53400eeb35b0b0c483c766375c3d2c15c7c4))


### Features

* added finalize invoice ([c6333bc](https://github.com/arlyon/async-stripe/commit/c6333bce94ae52454bcc90b79987baf96576da83))
* update stripe api ([2cba3fe](https://github.com/arlyon/async-stripe/commit/2cba3fe0083c7e865e27c2795a2dfe712b421466))

# [0.26.0](https://github.com/arlyon/async-stripe/compare/v0.25.2...v0.26.0) (2023-10-31)


### Features

* fixed bool value ([c752f52](https://github.com/arlyon/async-stripe/commit/c752f52c623a6af86b6253b27b03c3da83e46d6e))
* fixed bool value ([153d63d](https://github.com/arlyon/async-stripe/commit/153d63d984482dd67a42c6b16a3ee68106517727))
* fixed mutability ([86aabdd](https://github.com/arlyon/async-stripe/commit/86aabddd70432c13cb3d14eca29dc01b9bbe5155))
* fixed mutability ([5ad76c5](https://github.com/arlyon/async-stripe/commit/5ad76c5971d925aa719445a3ba1774b0a0cb1ab9))
* fixed warnings ([30f0a1e](https://github.com/arlyon/async-stripe/commit/30f0a1e9d6c8c6c1426b0f505d6bdd4b6ed6c349))
* fixed warnings ([e0d0bd8](https://github.com/arlyon/async-stripe/commit/e0d0bd8157257f8c40394de554f3be3c8482720a))
* updated ListPaginator to be generic over type T where T impl PaginableList instead of having separate SearchListPaginator and ListPaginator implementations for types SearchList and List. ([9d49602](https://github.com/arlyon/async-stripe/commit/9d496023128ff5b3b5b16065241afc83fc1ba3cd))
* updated ListPaginator to be generic over type T where T impl PaginableList instead of having separate SearchListPaginator and ListPaginator implementations for types SearchList and List. ([411f82c](https://github.com/arlyon/async-stripe/commit/411f82ce849357587712043e03b2aff7e0a55e3b))

## [0.25.2](https://github.com/arlyon/async-stripe/compare/v0.25.1...v0.25.2) (2023-10-05)


### Bug Fixes

* prevent panic on StatusCode conversion ([9b94228](https://github.com/arlyon/async-stripe/commit/9b942282e556b269c806caba56c8c7026badc33c))

## [0.25.1](https://github.com/arlyon/async-stripe/compare/v0.25.0...v0.25.1) (2023-09-27)


### Bug Fixes

* respect the required and optional fields for EventType and Metadata ([cdabd2e](https://github.com/arlyon/async-stripe/commit/cdabd2eac56cf3b9613f3b4617492684e27446a6))

# [0.25.0](https://github.com/arlyon/async-stripe/compare/v0.24.0...v0.25.0) (2023-09-25)


### Features

* generate latest changes from OpenApi spec ([652a360](https://github.com/arlyon/async-stripe/commit/652a3606f69e84734be7a4f83b1c7826af824437))

# [0.24.0](https://github.com/arlyon/async-stripe/compare/v0.23.0...v0.24.0) (2023-09-21)


### Features

* Generate latest changes from OpenApi spec ([7421960](https://github.com/arlyon/async-stripe/commit/742196050dda3593d272321ad7565451b0012031))
* Generate latest changes from OpenApi spec ([2fedb71](https://github.com/arlyon/async-stripe/commit/2fedb717db50b9438d026d2b4116c3fcdbd5bdc1))

# [0.23.0](https://github.com/arlyon/async-stripe/compare/v0.22.2...v0.23.0) (2023-09-06)


### Bug Fixes

* don't pin time-core for msrv ([00a9a1d](https://github.com/arlyon/async-stripe/commit/00a9a1d0067461f94ce978fd3b1885b3d2b8b6b6))
* Update hyper-rustls to fix RUSTSEC-2023-0052 ([7c31f76](https://github.com/arlyon/async-stripe/commit/7c31f76b0272031efabc7bda29ee12967d67ed10))


### Features

* add support for connect tokens ([b7c5489](https://github.com/arlyon/async-stripe/commit/b7c5489240eed0bd28e720eeda08afaf71bfa0eb))
* add tax calculation api behind a feature ([2266ed1](https://github.com/arlyon/async-stripe/commit/2266ed1196f6c9662d38815583ea266d170ff67f))
* handle currency_options ([1bb8165](https://github.com/arlyon/async-stripe/commit/1bb8165673eabdf157856de170ad47a1066ae362))

## [0.22.2](https://github.com/arlyon/async-stripe/compare/v0.22.1...v0.22.2) (2023-05-11)


### Bug Fixes

* Add sis_ id prefix for usage record summaries ([1042493](https://github.com/arlyon/async-stripe/commit/10424938736b0ce87d3be99fccb053855f3ddd3c))

## [0.22.1](https://github.com/arlyon/async-stripe/compare/v0.22.0...v0.22.1) (2023-05-09)


### Bug Fixes

* Move feature check to `build.rs` ([8d06be0](https://github.com/arlyon/async-stripe/commit/8d06be0f45aa645ce0e491a21cae33d4acc08b43))

# [0.22.0](https://github.com/arlyon/async-stripe/compare/v0.21.0...v0.22.0) (2023-04-18)


### Features

* generate latest changes from OpenApi spec ([a76703c](https://github.com/arlyon/async-stripe/commit/a76703cbb0a81ccba0a419c16518eff7216d1e26))

# [0.21.0](https://github.com/arlyon/async-stripe/compare/v0.20.2...v0.21.0) (2023-04-06)


### Bug Fixes

* **#342:** add documentation regarding idempotency to the main readme ([d28f7df](https://github.com/arlyon/async-stripe/commit/d28f7df6754738884049335e5b40684ff3093236)), closes [#342](https://github.com/arlyon/async-stripe/issues/342)


### Features

* use codegen version of WebhookEvent rather than overriding manually ([8347a6d](https://github.com/arlyon/async-stripe/commit/8347a6d600b11fe8d9e5d82217b9df56f9c800c8))

## [0.20.2](https://github.com/arlyon/async-stripe/compare/v0.20.1...v0.20.2) (2023-04-05)


### Bug Fixes

* generate api version from the codegen ([3006688](https://github.com/arlyon/async-stripe/commit/3006688bfadd5af84691c82f39abeb54dfdb859a))

## [0.20.1](https://github.com/arlyon/async-stripe/compare/v0.20.0...v0.20.1) (2023-03-23)


### Bug Fixes

* Use `HashMap::from` ([0c8dc9d](https://github.com/arlyon/async-stripe/commit/0c8dc9d89267a034c0901e7da15adc79c30050c3))

# [0.20.0](https://github.com/arlyon/async-stripe/compare/v0.19.0...v0.20.0) (2023-03-09)


### Features

* Implement Balance retrieve ([7509c98](https://github.com/arlyon/async-stripe/commit/7509c9878f9c85b91949e7dfeb1e1890a63f9fc2))

# [0.19.0](https://github.com/arlyon/async-stripe/compare/v0.18.4...v0.19.0) (2023-03-05)


### Features

* add CreditNote and CustomerBalanceTransaction ([dc82e59](https://github.com/arlyon/async-stripe/commit/dc82e59cd6e4e2f84b311d8e38c1299e96f5726d))

## [0.18.4](https://github.com/arlyon/async-stripe/compare/v0.18.3...v0.18.4) (2023-03-05)


### Bug Fixes

* add missing prefix for CheckoutSessionItemId ([c6bdb57](https://github.com/arlyon/async-stripe/commit/c6bdb572920d466d6db73d4445e2b1921c01acff))

## [0.18.3](https://github.com/arlyon/async-stripe/compare/v0.18.2...v0.18.3) (2023-02-20)


### Bug Fixes

* reexport duplicate SubscriptionPaymentBehaviour ([dbcff41](https://github.com/arlyon/async-stripe/commit/dbcff4189528483e72b4fa2c00a2396900aa97b8))

## [0.18.2](https://github.com/arlyon/async-stripe/compare/v0.18.1...v0.18.2) (2023-02-04)


### Bug Fixes

* prevent publishing the benchmark crate ([e4be54d](https://github.com/arlyon/async-stripe/commit/e4be54d0dbdec799ddab698f2941fc031b511fd1))

## [0.18.1](https://github.com/arlyon/async-stripe/compare/v0.18.0...v0.18.1) (2023-02-04)


### Bug Fixes

* prevent publishing the openapi crate ([6de846e](https://github.com/arlyon/async-stripe/commit/6de846e922dcf4cf828202f26a085aee6a2147ae))

# [0.18.0](https://github.com/arlyon/async-stripe/compare/v0.17.0...v0.18.0) (2023-02-04)


### Features

* fn to expire checkout sessions ([0368c79](https://github.com/arlyon/async-stripe/commit/0368c79f065cfc179855bc7fc60fa157ac758d1f))

# [0.17.0](https://github.com/arlyon/async-stripe/compare/v0.16.0...v0.17.0) (2023-02-01)


### Features

* improve api doc code to fix / add more coverage ([fecaa2e](https://github.com/arlyon/async-stripe/commit/fecaa2e17b4fb443e44b1f470beddd6659eeb107))

# [0.16.0](https://github.com/arlyon/async-stripe/compare/v0.15.2...v0.16.0) (2023-01-17)


### Bug Fixes

* prevent freeze when hitting network errors in a retry strategy ([d423be2](https://github.com/arlyon/async-stripe/commit/d423be225b457c5c08cc173e85e2dd2a2f3b04d8))


### Features

* add support for webpki (alt. to native-certs) ([c994b1c](https://github.com/arlyon/async-stripe/commit/c994b1caea19b5d7a4e8c0784cfa937127d54462))

## [0.15.2](https://github.com/arlyon/async-stripe/compare/v0.15.1...v0.15.2) (2023-01-17)


### Bug Fixes

* add automated release config ([ac5a87a](https://github.com/arlyon/async-stripe/commit/ac5a87a65d3e78b02cc1624b424fad32c7cd36a6))

# Version 0.15.1 (January 16, 2022)

This is a half-release before we overhaul the codegen in 0.16.
I want to give a special shoutout to @mzeitlin11 who has been
doing some excellent work behind the scenes cleaning up the
codegen.

## Features

- add a number of missing ids to `ids.rs`
- add a number of missing prefixes to `ids.rs`
- restore `serde_path_to_error` to give better error messages when
  parsing responses from the API
- add api to retrieve checkout session
- add api to cancel a setup intent

## Fixes

- export the `generated` modules so that overlapping apis can be
  reached by their path
- handle the `idempotency_error` error variant
- allow blocking client to be used in a multithreaded scenario
- disable connection pooling to prevent API errors

# Version 0.15.0 (June 9, 2022)

This release comes with some improvements to list traversal as well
as dramatic expansion of the api surface when dealing with webhooks!

## Features

- BREAKING: list methods now only borrow their parameters
- Add a streaming api for lazily and ergonomically traversing lists
- Exported the stripe terminal APIs
- Significantly more webhook types now available

## Fixes

- List traversals now properly include parameters when fetching
  later list items

# Version 0.14.1 (May 20, 2022)

This release is a minor patch release to fix an oversight in the API.

## Fixes

- Expose the request strategy API

# Version 0.14.0 (March 22, 2022)

This release has been focused on refining the ergonomics
supporting additional stripe APIs, and adding retries / idempotency.

## Features

- BREAKING: fluent client config api.
- Add the `card` param to the codegen. (@erichCompSci)
- Derive `Default` across the codebase.
- Opt-in idempotency functionality. (@erichCompSci)
- Opt-in retry logic.

In addition we have expanded the test coverage, and will
continue to do so with more examples.

# Version 0.13.0 (January 22, 2021)

First release of async-stripe with swappable runtimes.

## Breaking Changes

- You will now need to select a runtime to use the library.
- There may be small breaks in other places in the api.

# Version 0.12.3 (May 16, 2020)

## Fixes

- Fix failing `Invoice::upcoming` by adding a `is_none` representation to InvoiceId (@bryanburgers)

# Version 0.12.2 (May 15, 2020)

## Features

- Enable RustTLS behind a feature flag (@kiljacken)

## Fixes

- Add missing values to `PaymentMethodType` and `PaymentIntentMethodType` (@ThouCheese)

# Version 0.12.1 (February 6, 2020)

## Fixes

- Fixed deserialization of `ChargeId`s with `py_` prefix in `PaymentIntent` event(s).

# Version 0.12.0 (February 3, 2020)

## Breaking Changes

- Updated to a newer version of the Stripe API.
- The `PaymentIntent<___>Params` structs have been renamed to be consistent with the autogenerated API

  > e.g. `PaymentIntentCreateParams` has become `CreatePaymentIntent`

## Features

- The `app-info` header can now be set/customized with the `Client::set_app_info` method.

## Improvements

- New feature flags have been introduced to disable unused parts of the stripe APIs.
  This helps reduce binary bloat and compile time.

- The async client has been updated to use `std::future::Future`.
  The `async` feature flag is deprecated.
  Prefer `{ default-features = false, features = ["full"] }` instead.
- Set default stripe api version to the latest version supported by the library.
  This is intended to avoid bugs where a later version of stripe is enabled by a customer.

## Feature Flags

By default the `full` stripe api is enabled.

To reduce code size, disable default features and enable just the APIs you use:

```toml
# Example: Core-only (enough to create a `Charge` or `Card` or `Customer`)
stripe-rust = { version = "*", default-features = false, features = ["blocking"] }

# Example: Support for "Subscriptions" and "Invoices"
stripe-rust = { version = "*", default-features = false, features = ["blocking", "billing"] }
```

Refer to the [Stripe API docs](https://stripe.com/docs/api) to determine
which APIs are included as part of each feature flag.

# Version 0.11.0 (September 11, 2019)

## Features

- The `stripe-version` header can now be set/customized with the `Client::with_headers` method.

## Improvements

- The `Default` trait is now derived for all structs that have only defaultable parameters.
- `Webhook::construct_event` takes payload, signature, and secret args by ref (`&str`) instead of value.
- The `payment_method` and `confirmation_method` fields were added to `PaymentIntentCreateParams`.
- The `payment_method_details` structure now supports the `card` data field.

## Fixes

- Boolean fields that may be `null`, are now `Option` instead of `serde(default)`.
- The `Customer` variant was added to the `EventObject` enum.
- The `PaymentIntentCaptureParams` are now correctly `pub`.
- Fixed parsing a negative customer `account_balance`.

# Version 0.10.5 (July 25, 2019)

## Breaking Changes

- The `PaymentSourceParams` struct was edit to only include a `TokenId` and `SourceId` as per improvements to the API.
- The `charge.source` field to create a `Charge` was fixed to accept the new struct `ChargeSourceParams`.

# Version 0.10.4 (July 12, 2019)

## Improvements

- Added `id` field to `Event`

## Fixes

- Fixes a regression deriving `Eq` and `PartialEq` for `PaymentIntent` enums.

# Version 0.10.3 (June 21, 2019)

## Fixes

- Fixed deserialization of `CheckoutSessionCompleted` events.
- Added missing `Invoice` events.

# Version 0.10.2 (June 13, 2019)

## Fixes

- Fixed the `InvoiceLineItemId` type to correctly handle subscription items.

# Version 0.10.0 / 0.10.1 (June 4, 2019)

This version uses code generation to generate the stripe API according to the openapi spec.

This contains major breaking changes from previous versions, both
because many structures were out of date and because some changes
were made to make things easier to generate.

## New Features

- Many new APIs have now been implemented (mostly CRUD); some requests are still missing
  because not all requests are automatically implemented. All previously implemented
  requests are still implemented (even if they couldn't be code-generated).

- Many requests now take the `expand` parameter which controls whether more data should
  be returned for `Expandable<T>` fields. Previously these fields were just id types.

## Improvements

- Fields and requests have descriptive documentation rather than just referring to the stripe API docs.
- Ids and fields are more strictly typed (newtypes vs raw strings).
- Ids no longer need to be allocated and are cheaply clonable (in _almost_ all cases).
- Enum types now implement `Display` and `AsRef<str>`.

# Version 0.9.2 (April 7, 2019)

## Fixes / Improvements

- Add missing `product` to `Plan`.
- Add missing `receipt_url` to `Charge`.
- The deprecated `closed` field in `Invoice` is now optional.
- Fix allowing the `description` field in `Payout` to be optional.
- Fix allowing a `SubscriptionItem` quantity to be `None` (e.g. when using metered pricing API).
- Fix signature checking for received Webhooks.
- Allow the timestamp to be manually specified when verifying webhook signature.

# Version 0.9.1 (Feb 4, 2019)

## Fixes

- Detach source data structure is a new `DetachedSource` enum
- The `object` field was removed from `PaymentIntent`, `Refund` and `Payout`.
- Fields in `PaymentIntent` were updated to correctly be optional

# Version 0.9.0 (Jan 17, 2019)

## New Features

- Added an `async` feature flag which removes `stripe::Client` and adds `stripe::async::Client`.

## Breaking Changes

- The `stripe::Params` typed was renamed to `Headers` to avoid confusion
  with other `FooParams` types and the `params` module.

# Version 0.8.0 (Jan 15, 2019)

## New Features

- Added `verify_bank_account` to `Customer` resource.
- Added BankAccount as a variant of `PaymentSource`.
- Add the [PaymentIntents](https://stripe.com/docs/payments/payment-intents) resources, apis and events.

## Breaking Changes

- Minimum Rust version required is 1.31.1.
- Moved `retrieve_source` and `detach_source` to `Customer` resource from `Source`.
- The `Error::Conversion` enum variant has been replaced by `Error::Serialize` and `Error::Deserialize`.
- See also **Fixes / Improvements**.

## Fixes / Improvements

- TokenId has become an enum to account for different Token options
- Updated fields on `BankAccount` resource so all are correct.
- The `Payout` and `Refund` types now use enums instead of Strings.
- The `WebhookError` type is now externally public.

# Version 0.7.2

## Fixes

- Fix sending `metadata` (and other nested params) as part of a POST request;
  e.g. `Customer::create`, `Customer::update`, etc.

# Version 0.7.0 and 0.7.1

## Fixes

- The type of `Customer.default_source` has changed from `SourceId` to `PaymentSourceId`.
- The type of `Customer.sources` has changed from `List<Source>` to `List<PaymentSource>`.

# Version 0.6.0

## New Features / Improvements

- The `List<T>` type has new `next` and `get_all` methods for pagination.
- Struct types more consistently implement `Clone`.
- Enum types more consistently implement `Copy`, `Eq`, and `Hash`.
- More source details (e.g. `ach_credit_transfer`, etc) were added to `Source`.
- The blocking api is now backed by `reqwest` instead of `hyper:0.10` so that
  the library is no longer pinned to outdated dependencies.
- Added the "webhooks" feature flag for processing events (enabled by default)
  to convert `hmac` and `sha2` to optional dependencies.

## Breaking Changes

- (Mostly from 0.5.x series) `Charge`, `Card`, and `Source` have been updated
  to use new enum values rather than `String`.
- The `SourceType` enum used by **ChargeListParams** has been renamed to
  `SourceFilterType` to remove a conflict with the `SourceType` enum used by
  the type field of **Source**.

## Fixes

- All `Address` fields have been updated to be optional.
- The `Client-Id` param has been added to params to make support for
  impersonating **Connect Accounts** compatible with newer stripe api versions.

# Version 0.5.3

## Changes

- Updated `Charge` to use enum types rather than `&str` where possible.

# Version 0.5.2

## Changes

- Fixed many issues with the `Source` object.
- Updated `Card` to use enum types rather than `&str` where possible.
- Added many new fields to `Card`.

# Version 0.5.1

## Changes

- Bumped dependency versions

# Version 0.5.0 (Sep 26, 2018)

## New Features

- Implemented the [sources resource](https://stripe.com/docs/api#sources) and
  added the `Source` type .

## Breaking Changes

- The `CustomerSourceParams` struct was renamed to `PaymentSourceParams`.
- The `Source` enum was renamed to `PaymentSource` (there is a new `Source` struct).
- Source and token fields (like `token: &str`) now use `source: SourceId` or `token: TokenId` instead.

## General Improvements

- Added missing `default_source` field to `Customer`.
- The `Deserialize` and `Serialize` traits have now been implemented for all param and resource structs.
- The `Copy`, `Clone`, `Eq`, `PartialEq`, and `Hash` traits have been derived for flat enum types.

# Version 0.4.7 (Jun 11, 2018)

## Changes

- Fix, adds all missing error code variants to `stripe::error::ErrorCode`.

# Version 0.4.6 (Jun 9, 2018)

## Changes

- Fixed field `nickname` of `Plan` can be null.

# Version 0.4.5 (Feb 20, 2018)

## Changes

- Fixed field `nickname` of `Plan` should be `name`

# Version 0.4.4 (Jan 4, 2018)

## Breaking Changes

- The `balance_transaction` field of `Charge` is now `Option<_>`.

# Version 0.4.2 (August 20, 2017)

## Changes

- Added basic crate documentation
- Added the `invoice.upcoming` event

# Version 0.4.1 (August 8, 2017)

## Changes

- Add RangeQuery struct for created/date filters in list requests
- Implement `Customer::list` request (ie. "GET /customers"); Thanks @pocket7878
- Add missing params to `InvoiceListParams` for `Invoice::list`

# Version 0.4.0 (August 2, 2017)

## Breaking Changes

- Upgrade Serde to v1.0

# Version 0.3.3 (August 1, 2017)

## Changes

- Add support for authentication using the Stripe-Account header
- Implement `Event` struct for implementing a Stripe Webhook (only some events)
- Implement `Invoice::update` request (ie. POST "/invoices/{invoice_id}")
- Implement `InvoiceItem::create` request (ie. POST "/invoiceitems")
- Fix `Subscription::cancel` request (previously used wrong method/path)

# Version 0.3.2 (June 17, 2017)

## Changes

- Implemented the `Invoice::list` API (ie. "/invoices")
- Fixed a request encoding error for `Invoice::pay` (ie. "/invoices/{invoice_id}/pay")
- Fixed fields on `Subscription` that could be null but were missing `Optional<..>`
- Added missing `last4` field to `Card` resource
- Changed the repository URL

# Version 0.3.1 (Mar 31, 2017)

## Changes

- Made rust docs slightly easier to read
- Fixed incorrect method name for `Plan` resource
- Use `native-tls` by default and add feature-flag supporting `openssl`

# Version 0.3.0 (Mar 28, 2017)

## New Features

- Implemented the [charges resource](https://stripe.com/docs/api#charges) and
  added the `Charge` type.

## Breaking Changes

- The `Resource::get` requests have been renamed to `Resource::retrieve`.
- The `Subscription::cancel` request now expects `CancelParams` instead of
  expecting a one-off function argument.
- Removed the `blame` method and `Blame` type from errors.
- The `Error::Decode` and `Error::Encode` variants have been combined into a
  single `Error::Conversion` variant.

## General Improvements

- The `Error` type now has easier to read error messages and will parses JSON
  errors returned by Stripe into a structured error type (`RequestError`).
- Added Travis CI for automated testing on GitHub.
- Added reference documentation to implemented resources (but still no docs at
  crate/module level nor a "Getting started" guide)

# Version 0.2.0 (Mar 17, 2017)

## New Features

- Added `Client` which is created once with a stripe private key and is
  intended to be re-used for multiple requests. It implements `Sync` so you
  can share it among multiple threads.
- Added new strongly-typed `Currency` type following the example of
  https://github.com/stripe/stripe-go.
- `Customer::create`/`CustomerParams` now support using `CardParams` as a
  source instead of just tokens.
- Implemented the [sources API](https://stripe.com/docs/api#sources) in the `Source` type.

## Breaking Changes

- All `Params` types now use `&str` fields instead of `String`s.
- Requests used to require a `stripe_key: &str` as their final argument but
  now use a `&Client` as the first argument instead.
- Stripe tokens used to be directly used as a source in `CustomerParams` but
  now must be used with `CustomerSourceParams::Token("tok_xyzABC123")`.

## General Improvements

- Added remaining fields to `Params` types after switching from
  `serde_urlencoded` to `serde_qs` to support nested params.
- Added the `create_customer` example
- Types that implement `Deserialize` now also implement `Debug`.
- Types that implement `Serialize` now also implement `Default`.
