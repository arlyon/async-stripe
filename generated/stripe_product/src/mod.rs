#![recursion_limit = "256"]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_html_tags)]
extern crate self as stripe_product;
pub use stripe_types::Coupon;
pub mod coupon;
pub use stripe_types::CouponAppliesTo;
pub use stripe_types::CouponCurrencyOption;
pub use stripe_types::CurrencyOption;
pub use stripe_types::CustomUnitAmount;
pub use stripe_types::DeletedCoupon;
pub use stripe_types::DeletedDiscount;
pub use stripe_types::DeletedProduct;
pub use stripe_types::Discount;
pub use stripe_types::LineItemsDiscountAmount;
pub use stripe_types::LineItemsTaxAmount;
pub use stripe_types::PackageDimensions;
pub use stripe_types::Price;
pub mod price;
pub use stripe_types::PriceTier;
pub use stripe_types::Product;
pub mod product;
pub use stripe_types::ProductFeature;
pub use stripe_types::PromotionCode;
pub mod promotion_code;
pub use stripe_types::PromotionCodeCurrencyOption;
pub use stripe_types::PromotionCodesResourceRestrictions;
pub use stripe_types::Recurring;
pub use stripe_types::ShippingRate;
pub mod shipping_rate;
pub use stripe_types::TaxProductResourceTaxCode;
pub mod tax_product_resource_tax_code;
pub use stripe_types::TaxRate;
pub mod tax_rate;
pub use stripe_types::TransformQuantity;
