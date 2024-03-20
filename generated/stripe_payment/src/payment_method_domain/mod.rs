#[cfg(feature = "payment_method_domain")]
mod requests;
pub(crate) mod types;
#[cfg(feature = "payment_method_domain")]
pub use requests::*;
