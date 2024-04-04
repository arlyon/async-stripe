// Needed for `json!` usage in tests
#![recursion_limit = "256"]
mod deser;
mod mock;
mod price;

#[cfg(feature = "async")]
mod async_tests;
#[cfg(feature = "blocking")]
mod blocking;
// NB: pagination utils ideally could be used for blocking tests as well, but tricky because the `MockServer` is async
// and the blocking client unconditionally creates its own runtime already
#[cfg(feature = "async")]
mod pagination_utils;
