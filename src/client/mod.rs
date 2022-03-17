mod request_strategy;
mod stripe;

mod base {
    #[cfg(any(
        feature = "runtime-tokio-hyper",
        feature = "runtime-tokio-hyper-rustls",
        feature = "runtime-blocking",
        feature = "runtime-blocking-rustls",
    ))]
    pub mod tokio;

    #[cfg(feature = "runtime-async-std-surf")]
    pub mod async_std;

    #[cfg(any(feature = "runtime-blocking", feature = "runtime-blocking-rustls"))]
    pub mod tokio_blocking;
}

#[cfg(any(feature = "runtime-blocking", feature = "runtime-blocking-rustls"))]
pub(crate) mod config {
    pub use super::base::tokio_blocking::{err, ok};
    pub use super::base::tokio_blocking::{Response, TokioBlockingClient as BaseClient};
}

#[cfg(any(feature = "runtime-tokio-hyper", feature = "runtime-tokio-hyper-rustls"))]
pub(crate) mod config {
    pub(crate) use super::base::tokio::{err, ok};
    pub use super::base::tokio::{Response, TokioClient as BaseClient};
}

#[cfg(feature = "runtime-async-std-surf")]
pub(crate) mod config {
    pub(crate) use super::base::async_std::{err, ok};
    pub use super::base::async_std::{AsyncStdClient as BaseClient, Response};
}

/// An alias for `Result`.
///
/// If `blocking` is enabled, defined as:
///
/// ```rust,ignore
/// type Response<T> = Result<T, Error>;
/// ```
///
/// If the `async` feature is enabled, this type is defined as:
///
/// ```rust,ignore
/// type Response<T> = Box<dyn Future<Result<T, Error>>>;
/// ```
pub use config::Response;

pub use self::stripe::Client;
pub use config::BaseClient;
