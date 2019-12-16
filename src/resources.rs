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
mod payment_intent;
mod payment_source;
mod payout;
mod payout_ext;
mod platform_tax_fee;
mod product;
mod refund;
mod reserve_transaction;
mod setup_intent;
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
pub use self::payment_intent::*;
pub use self::payment_source::*;
pub use self::payout::*;
pub use self::payout_ext::*;
pub use self::platform_tax_fee::*;
pub use self::product::*;
pub use self::refund::*;
pub use self::reserve_transaction::*;
pub use self::setup_intent::*;
pub use self::token::*;
pub use self::token_ext::*;

// Payment Methods
mod alipay_account;
mod bank_account;
mod bank_account_ext;
mod card;
mod payment_method;
mod source;
mod source_ext;
pub use self::alipay_account::*;
pub use self::bank_account::*;
pub use self::bank_account_ext::*;
pub use self::card::*;
pub use self::payment_method::*;
pub use self::source::*;
pub use self::source_ext::*;

// Events
#[cfg(feature = "events")]
mod event;
#[cfg(feature = "events")]
pub use self::event::*;

// Unstable everything else
#[cfg(feature = "unstable-everything-else")]
mod account;
#[cfg(feature = "unstable-everything-else")]
mod application;
#[cfg(feature = "unstable-everything-else")]
mod application_fee;
#[cfg(feature = "unstable-everything-else")]
mod checkout_session;
#[cfg(feature = "unstable-everything-else")]
mod connect_collection_transfer;
#[cfg(feature = "unstable-everything-else")]
mod coupon;
#[cfg(feature = "unstable-everything-else")]
mod discount;
#[cfg(feature = "unstable-everything-else")]
mod fee_refund;
#[cfg(feature = "unstable-everything-else")]
mod invoice;
#[cfg(feature = "unstable-everything-else")]
mod invoice_ext;
#[cfg(feature = "unstable-everything-else")]
mod invoiceitem;
#[cfg(feature = "unstable-everything-else")]
mod issuing_authorization;
#[cfg(feature = "unstable-everything-else")]
mod issuing_authorization_ext;
#[cfg(feature = "unstable-everything-else")]
mod issuing_card;
#[cfg(feature = "unstable-everything-else")]
mod issuing_card_ext;
#[cfg(feature = "unstable-everything-else")]
mod issuing_cardholder;
#[cfg(feature = "unstable-everything-else")]
mod issuing_dispute;
#[cfg(feature = "unstable-everything-else")]
mod issuing_dispute_ext;
#[cfg(feature = "unstable-everything-else")]
mod issuing_merchant_data;
#[cfg(feature = "unstable-everything-else")]
mod issuing_transaction;
#[cfg(feature = "unstable-everything-else")]
mod issuing_transaction_ext;
#[cfg(feature = "unstable-everything-else")]
mod line_item;
#[cfg(feature = "unstable-everything-else")]
mod line_item_ext;
#[cfg(feature = "unstable-everything-else")]
mod order;
#[cfg(feature = "unstable-everything-else")]
mod order_ext;
#[cfg(feature = "unstable-everything-else")]
mod order_item;
#[cfg(feature = "unstable-everything-else")]
mod order_return;
#[cfg(feature = "unstable-everything-else")]
mod person;
#[cfg(feature = "unstable-everything-else")]
mod plan;
#[cfg(feature = "unstable-everything-else")]
mod recipient;
#[cfg(feature = "unstable-everything-else")]
mod review;
#[cfg(feature = "unstable-everything-else")]
mod review_ext;
#[cfg(feature = "unstable-everything-else")]
mod scheduled_query_run;
#[cfg(feature = "unstable-everything-else")]
mod sku;
#[cfg(feature = "unstable-everything-else")]
mod subscription;
#[cfg(feature = "unstable-everything-else")]
mod subscription_ext;
#[cfg(feature = "unstable-everything-else")]
mod subscription_item;
#[cfg(feature = "unstable-everything-else")]
mod subscription_schedule;
#[cfg(feature = "unstable-everything-else")]
mod tax_id;
#[cfg(feature = "unstable-everything-else")]
mod tax_rate;
#[cfg(feature = "unstable-everything-else")]
mod topup;
#[cfg(feature = "unstable-everything-else")]
mod transfer;
#[cfg(feature = "unstable-everything-else")]
mod transfer_reversal;
#[cfg(feature = "unstable-everything-else")]
mod webhook_endpoint;
#[cfg(feature = "unstable-everything-else")]
mod webhook_endpoint_ext;
#[cfg(feature = "unstable-everything-else")]
pub use self::account::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::application::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::application_fee::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::checkout_session::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::connect_collection_transfer::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::coupon::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::discount::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::fee_refund::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::invoice::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::invoice_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::invoiceitem::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_authorization::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_authorization_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_card::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_card_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_cardholder::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_dispute::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_dispute_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_merchant_data::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_transaction::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::issuing_transaction_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::line_item::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::line_item_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::order::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::order_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::order_item::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::order_return::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::person::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::plan::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::recipient::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::review::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::review_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::scheduled_query_run::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::sku::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::subscription::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::subscription_ext::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::subscription_item::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::subscription_schedule::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::tax_id::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::tax_rate::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::topup::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::transfer::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::transfer_reversal::*;
#[cfg(feature = "unstable-everything-else")]
pub use self::webhook_endpoint::*;
#[cfg(feature = "unstable-everything-else")]
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

