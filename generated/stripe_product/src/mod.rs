#![recursion_limit = "256"]
extern crate self as stripe_product;
pub use stripe_types::coupon::*;
pub mod coupon;
pub use stripe_types::coupon_applies_to::*;
pub use stripe_types::coupon_currency_option::*;
pub use stripe_types::currency_option::*;
pub use stripe_types::custom_unit_amount::*;
pub use stripe_types::deleted_coupon::*;
pub use stripe_types::deleted_discount::*;
pub use stripe_types::deleted_product::*;
pub use stripe_types::discount::*;
pub use stripe_types::line_items_discount_amount::*;
pub use stripe_types::line_items_tax_amount::*;
pub use stripe_types::package_dimensions::*;
pub use stripe_types::price::*;
pub mod price;
pub use stripe_types::price_tier::*;
pub use stripe_types::product::*;
pub mod product;
pub use stripe_types::promotion_code::*;
pub mod promotion_code;
pub use stripe_types::promotion_code_currency_option::*;
pub use stripe_types::promotion_codes_resource_restrictions::*;
pub use stripe_types::recurring::*;
pub use stripe_types::shipping_rate::*;
pub mod shipping_rate;
pub use stripe_types::tax_product_resource_tax_code::*;
pub mod tax_product_resource_tax_code;
pub use stripe_types::tax_rate::*;
pub mod tax_rate;
pub use stripe_types::transform_quantity::*;
