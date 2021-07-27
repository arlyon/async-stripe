//! generated module
//!
//! Contains the generated impls we use. This code
//! is automatically generated from the openapi spec
//! and should not be changed manually. To update the
//! spec, use cargo make.
//!
//! It is possible more files are generated than are
//! listed as modules here. These are modules that
//! have not yet been exposed by the client.

#[path = "generated"]
pub mod core {
    pub mod balance;
    pub mod balance_transaction;
    pub mod charge;
    pub mod customer;
    pub mod dispute;
    pub mod file;
    pub mod file_link;
    pub mod mandate;
    pub mod payment_intent;
    pub mod payout;
    pub mod platform_tax_fee;
    pub mod product;
    pub mod refund;
    pub mod reserve_transaction;
    pub mod setup_attempt;
    pub mod setup_intent;
    pub mod tax_code;
    pub mod tax_deducted_at_source;
    pub mod token;
}

#[path = "generated"]
pub mod payment {
    pub mod alipay_account;
    pub mod bank_account;
    pub mod card;
    pub mod payment_method;
    pub mod source;
}

#[path = "generated"]
#[cfg(feature = "checkout")]
pub mod checkout {
    pub mod checkout_session;
    pub mod item;
}

#[path = "generated"]
#[cfg(feature = "billing")]
pub mod billing {
    pub mod coupon;
    pub mod discount;
    pub mod invoice;
    pub mod invoiceitem;
    pub mod line_item;
    pub mod plan;
    pub mod price;
    pub mod promotion_code;
    pub mod quote;
    pub mod subscription;
    pub mod subscription_item;
    pub mod subscription_schedule;
    pub mod tax_id;
    pub mod tax_rate;
}

#[path = "generated"]
#[cfg(feature = "connect")]
pub mod connect {
    pub mod account;
    pub mod account_link;
    pub mod application;
    pub mod application_fee;
    pub mod connect_collection_transfer;
    pub mod fee_refund;
    pub mod person;
    pub mod recipient;
    pub mod topup;
    pub mod transfer;
    pub mod transfer_reversal;
}

#[path = "generated"]
#[cfg(feature = "fraud")]
pub mod fraud {
    pub mod review;
}

#[path = "generated"]
#[cfg(feature = "issuing")]
pub mod issuing {
    pub mod issuing_authorization;
    pub mod issuing_card;
    pub mod issuing_cardholder;
    pub mod issuing_dispute;
    pub mod issuing_transaction;
}

#[path = "generated"]
#[cfg(feature = "orders")]
pub mod orders {
    pub mod order;
    pub mod order_item;
    pub mod order_return;
    pub mod sku;
}

#[cfg(feature = "sigma")]
pub mod scheduled_query_run;

#[cfg(feature = "events")]
pub mod event;

#[path = "generated"]
#[cfg(feature = "webhook-endpoints")]
pub mod webhook_endpoints {
    pub mod webhook_endpoint;
}

#[cfg(not(feature = "full"))]
pub mod placeholders;
