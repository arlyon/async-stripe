// Needed for `json!` usage in tests
#![recursion_limit = "256"]
mod mock;

mod deser;

#[cfg(feature = "async")]
mod async_tests;
#[cfg(feature = "blocking")]
mod blocking;
