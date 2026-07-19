# async-stripe-miniserde

Vendored fork of [miniserde](https://github.com/dtolnay/miniserde), specialized
for async-stripe's deserialization needs. Not intended for use outside this
workspace.

## Why a fork

Upstream miniserde is a small, push-based JSON deserializer. async-stripe uses
it as the primary deserialization path because it monomorphizes very little and
keeps generated code small. We diverged from upstream to add a few capabilities
the codegen needs:

- `Visitor::wants_raw` / `Visitor::raw` for polymorphic enum dispatch
  (peeking the `"object"` tag of a JSON object before choosing a variant).
- `Deserialize::WANTS_RAW` const so wrapper types (`Option<T>`, `Box<T>`) can
  const-fold the raw path away when `T` doesn't need it.
- Stripe-domain helpers (`peek_object_tag`, `peek_deleted_flag`) for the
  polymorphic-tag and deleted-or-base patterns the Stripe API uses.
- Removed the `mini-internal` derive crate. The codegen hand-writes
  `Deserialize` impls; nothing in the workspace uses `#[derive(Deserialize)]`.

## License

MIT OR Apache-2.0, same as upstream. See `LICENSE-MIT` and `LICENSE-APACHE`.
Original copyright holder: David Tolnay.
