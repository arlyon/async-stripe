#![feature(generic_associated_types)]
#![feature(try_trait_v2)]

pub mod base_client;
pub mod error;
pub mod ids;
pub mod params;
pub mod request_strategy;
pub mod stripe;
pub mod version;

use std::{future::Future, pin::Pin};

pub use base_client::BaseClient;
use error::StripeError;
pub use stripe::Client;

#[cfg(not(feature = "is_sync"))]
pub type Response<T> = Pin<Box<dyn Future<Output = Result<T, StripeError>> + Send>>;

#[cfg(feature = "is_sync")]
pub type Response<T> = Result<T, StripeError>;
