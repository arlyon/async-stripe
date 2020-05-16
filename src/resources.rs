// Builtin types
mod currency;
mod types;
pub use self::currency::*;
pub use self::types::*;

// Core Resources
mod balance;
mod balance_transaction;
mod balance_transaction_ext;
mod charge;
mod charge_ext;
mod customer;
mod customer_ext;
mod dispute;
mod file;
mod file_link;
mod mandate;
mod payment_intent;
mod payment_source;
mod payout;
mod payout_ext;
mod platform_tax_fee;
mod product;
mod refund;
mod reserve_transaction;
mod setup_intent;
mod tax_deducted_at_source;
mod token;
mod token_ext;
pub use self::balance::*;
pub use self::balance_transaction::*;
pub use self::balance_transaction_ext::*;
pub use self::charge::*;
pub use self::charge_ext::*;
pub use self::customer::*;
pub use self::customer_ext::*;
pub use self::dispute::*;
pub use self::file::*;
pub use self::file_link::*;
pub use self::mandate::*;
pub use self::payment_intent::*;
pub use self::payment_source::*;
pub use self::payout::*;
pub use self::payout_ext::*;
pub use self::platform_tax_fee::*;
pub use self::product::*;
pub use self::refund::*;
pub use self::reserve_transaction::*;
pub use self::setup_intent::*;
pub use self::tax_deducted_at_source::*;
pub use self::token::*;
pub use self::token_ext::*;

// Payment Methods
mod alipay_account;
mod bank_account;
mod bank_account_ext;
mod card;
mod payment_method;
mod payment_method_ext;
mod source;
mod source_ext;
pub use self::alipay_account::*;
pub use self::bank_account::*;
pub use self::bank_account_ext::*;
pub use self::card::*;
pub use self::payment_method::*;
pub use self::payment_method_ext::*;
pub use self::source::*;
pub use self::source_ext::*;

// Events
#[cfg(feature = "events")]
mod event;
#[cfg(feature = "events")]
pub use self::event::*;

// Checkout
#[cfg(feature = "checkout")]
mod checkout_session;
#[cfg(feature = "checkout")]
mod item;
#[cfg(feature = "checkout")]
pub use self::checkout_session::*;
#[cfg(feature = "checkout")]
pub use self::item::*;

// Billing
#[cfg(feature = "billing")]
mod coupon;
#[cfg(feature = "billing")]
mod discount;
#[cfg(feature = "billing")]
mod invoice;
#[cfg(feature = "billing")]
mod invoice_ext;
#[cfg(feature = "billing")]
mod invoiceitem;
#[cfg(feature = "billing")]
mod line_item;
#[cfg(feature = "billing")]
mod line_item_ext;
#[cfg(feature = "billing")]
mod plan;
#[cfg(feature = "billing")]
mod price;
#[cfg(feature = "billing")]
mod subscription;
#[cfg(feature = "billing")]
mod subscription_ext;
#[cfg(feature = "billing")]
mod subscription_item;
#[cfg(feature = "billing")]
mod subscription_schedule;
#[cfg(feature = "billing")]
mod tax_id;
#[cfg(feature = "billing")]
mod tax_rate;
#[cfg(feature = "billing")]
pub use self::coupon::*;
#[cfg(feature = "billing")]
pub use self::discount::*;
#[cfg(feature = "billing")]
pub use self::invoice::*;
#[cfg(feature = "billing")]
pub use self::invoice_ext::*;
#[cfg(feature = "billing")]
pub use self::invoiceitem::*;
#[cfg(feature = "billing")]
pub use self::line_item::*;
#[cfg(feature = "billing")]
pub use self::line_item_ext::*;
#[cfg(feature = "billing")]
pub use self::plan::*;
#[cfg(feature = "billing")]
pub use self::price::*;
#[cfg(feature = "billing")]
pub use self::subscription::*;
#[cfg(feature = "billing")]
pub use self::subscription_ext::*;
#[cfg(feature = "billing")]
pub use self::subscription_item::*;
#[cfg(feature = "billing")]
pub use self::subscription_schedule::*;
#[cfg(feature = "billing")]
pub use self::tax_id::*;
#[cfg(feature = "billing")]
pub use self::tax_rate::*;

// Connect
#[cfg(feature = "connect")]
mod account;
#[cfg(feature = "connect")]
mod application;
#[cfg(feature = "connect")]
mod application_fee;
#[cfg(feature = "connect")]
mod connect_collection_transfer;
#[cfg(feature = "connect")]
mod fee_refund;
#[cfg(feature = "connect")]
mod person;
#[cfg(feature = "connect")]
mod recipient;
#[cfg(feature = "connect")]
mod topup;
#[cfg(feature = "connect")]
mod transfer;
#[cfg(feature = "connect")]
mod transfer_reversal;
#[cfg(feature = "connect")]
pub use self::account::*;
#[cfg(feature = "connect")]
pub use self::application::*;
#[cfg(feature = "connect")]
pub use self::application_fee::*;
#[cfg(feature = "connect")]
pub use self::connect_collection_transfer::*;
#[cfg(feature = "connect")]
pub use self::fee_refund::*;
#[cfg(feature = "connect")]
pub use self::person::*;
#[cfg(feature = "connect")]
pub use self::recipient::*;
#[cfg(feature = "connect")]
pub use self::topup::*;
#[cfg(feature = "connect")]
pub use self::transfer::*;
#[cfg(feature = "connect")]
pub use self::transfer_reversal::*;

// Fraud
#[cfg(feature = "fraud")]
mod review;
#[cfg(feature = "fraud")]
mod review_ext;
#[cfg(feature = "fraud")]
pub use self::review::*;
#[cfg(feature = "fraud")]
pub use self::review_ext::*;

// Issuing
#[cfg(feature = "issuing")]
mod issuing_authorization;
#[cfg(feature = "issuing")]
mod issuing_authorization_ext;
#[cfg(feature = "issuing")]
mod issuing_card;
#[cfg(feature = "issuing")]
mod issuing_card_ext;
#[cfg(feature = "issuing")]
mod issuing_cardholder;
#[cfg(feature = "issuing")]
mod issuing_dispute;
#[cfg(feature = "issuing")]
mod issuing_dispute_ext;
#[cfg(feature = "issuing")]
mod issuing_merchant_data;
#[cfg(feature = "issuing")]
mod issuing_transaction;
#[cfg(feature = "issuing")]
mod issuing_transaction_ext;
#[cfg(feature = "issuing")]
pub use self::issuing_authorization::*;
#[cfg(feature = "issuing")]
pub use self::issuing_authorization_ext::*;
#[cfg(feature = "issuing")]
pub use self::issuing_card::*;
#[cfg(feature = "issuing")]
pub use self::issuing_card_ext::*;
#[cfg(feature = "issuing")]
pub use self::issuing_cardholder::*;
#[cfg(feature = "issuing")]
pub use self::issuing_dispute::*;
#[cfg(feature = "issuing")]
pub use self::issuing_dispute_ext::*;
#[cfg(feature = "issuing")]
pub use self::issuing_merchant_data::*;
#[cfg(feature = "issuing")]
pub use self::issuing_transaction::*;
#[cfg(feature = "issuing")]
pub use self::issuing_transaction_ext::*;

// Orders
#[cfg(feature = "orders")]
mod order;
#[cfg(feature = "orders")]
mod order_ext;
#[cfg(feature = "orders")]
mod order_item;
#[cfg(feature = "orders")]
mod order_return;
#[cfg(feature = "orders")]
mod sku;
#[cfg(feature = "orders")]
pub use self::order::*;
#[cfg(feature = "orders")]
pub use self::order_ext::*;
#[cfg(feature = "orders")]
pub use self::order_item::*;
#[cfg(feature = "orders")]
pub use self::order_return::*;
#[cfg(feature = "orders")]
pub use self::sku::*;

#[cfg(feature = "sigma")]
mod scheduled_query_run;
#[cfg(feature = "sigma")]
pub use self::scheduled_query_run::*;

// Not-yet-implemented feature flags
#[cfg(feature = "webhook-endpoints")]
mod webhook_endpoint;
#[cfg(feature = "webhook-endpoints")]
mod webhook_endpoint_ext;
#[cfg(feature = "webhook-endpoints")]
pub use self::webhook_endpoint::*;
#[cfg(feature = "webhook-endpoints")]
pub use self::webhook_endpoint_ext::*;

// Fallback types
#[cfg(not(feature = "full"))]
mod placeholders;
#[cfg(not(feature = "full"))]
pub use self::placeholders::*;

#[cfg(not(feature = "account"))]
#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct CompanyParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}

#[cfg(not(feature = "account"))]
#[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct PersonParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}
