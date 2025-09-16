# Contributing to async-stripe

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Test it: `cargo test --features runtime-blocking`
4. Lint it: `cargo +1.82.0 clippy --all --all-targets -- -D warnings`
5. Commit your changes: `git commit -am 'Add some feature'`
6. Push to the branch: `git push origin my-new-feature`
7. Submit a pull request :D

We use `rustfmt` to keep our codebase consistently formatted. Please ensure that
you have correctly formatted your code (most editors will do this automatically
when saving) or it may not pass the CI tests.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as in the README, without any
additional terms or conditions.

## Coding standards

These are requirements we have that we have not yet lifted to the level of
automatic enforcement.

### Import grouping

In each file the imports should be grouped into at most 4 groups in the
following order:

1. stdlib
2. non-repository local crates
3. repository local other crates
4. this crate

Separate each group with a blank line, and rustfmt will sort into a canonical
order. Any file that is not grouped like this can be rearranged whenever the
file is touched - we're not precious about having it done in a separate commit,
though that is helpful.

### Clippy lints

We ask that contributors keep the clippy status clean. Minimally, run `cargo clippy`
before submitting code. Clippy is also run in GitHub Actions.

### rustfmt

It is expected that code is uniformly formatted. Before submitting code, make sure
to run `cargo fmt` to make sure it conforms to the standard.

## Code Generation
This library is (mostly) authored via code generation by parsing the OpenAPI specification for Stripe.
It consists of 3 main pieces:
- `async-stripe`: The definition of the `Stripe` client
- `async-stripe-types`: Core type definitions, used a ton in generated code
- `generated/*`: Generated crates which implement `Stripe` API requests and related types.
- `async-stripe-webhook`: Glue code for parsing and validating `Stripe` webhook events and generated
code for deserializing the events themselves.

No changes should be made to code in a `generated/*` folder. If you'd like to change that
code, please see the `README` in the `openapi` crate which explains the code generation process
in more detail.

If you'd like to update the version of the OpenAPI specification being used to generated code, you
can run (in the `openapi` directory)
```sh
cargo run --release -- --fetch latest
```

This will automatically pull the latest OpenAPI spec, generate new code, format it, and copy it into
library.

## Testing

To run the tests, you will need
to run a [`stripe-mock`](https://github.com/stripe/stripe-mock) server and select a runtime. CI runs tests against all runtimes, but it is encouraged you test your changes locally against a few runtimes first.

```sh
docker run --rm -d -it -p 12111-12112:12111-12112 stripe/stripe-mock:v0.185.0
cargo test --features runtime-blocking
```

## Communication

It is encouraged to open an issue before you create a PR as a place for pre-implementation
discussion. If you're unsure about your contribution or simply want to ask a question about anything just open an issue and we'll chat.

## Architecture

This project makes wide use of code generation, with API types generated directly from the stripe
openapi spec. This is what the openapi tool does. All generated code ends up in the handily named
generated folder which is exposed based on features specified.

In some cases, it is helpful to have additional logic associated with a datatype to, for example,
capture a create `Charge` object. This additional impl goes in the `charge_ext.rs` file in the
`resources` folder, to provide a clean seperation between generated and hand maintained files.
If you notice that logic is missing, please add it to (or create) the appropriate `ext` file.

## Versioning

Due to the nature of the generated crates, all crates are versioned together. To bump the versions, will need to
do the following steps:

1. Bump the version in `Cargo.toml`
2. Run the code generation with the new version `cd openapi && cargo run -- <version>`
3. Update any version requirements in the manually managed packages
4. Commit the changes
5. Publish the changes `cargo workspaces publish --from-git --allow-branch next`
