#![recursion_limit = "256"]
extern crate self as stripe_product;

#[cfg(feature = "min-ser")]
miniserde::make_place!(Place);
pub mod shipping_rate;
pub use shipping_rate::ShippingRate;
pub mod sku;
pub use sku::Sku;
