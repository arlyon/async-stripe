//! resources module
//!
//! This module exposes various elements of the
//! stripe api depending on the features exposed.
//!
//! Some of these modules are hand-written, and
//! some are generated.

mod currency;
mod generated;
mod placeholders;
mod types;

#[path = "resources"]
mod core {
    // pub mod balance_transaction_ext;
    // pub mod charge_ext;
    // pub mod customer_ext;
    // pub mod payment_intent_ext;
    // pub mod payment_source;
    // pub mod payout_ext;
    // pub mod placeholders;
    // pub mod setup_intent_ext;
    // pub mod token_ext;
}

#[path = "resources"]
mod payment {
    // pub mod bank_account_ext;
    // pub mod card;
    // pub mod payment_method_ext;
    // pub mod source_ext;
}

#[cfg(feature = "events")]
mod webhook_events;

#[path = "resources"]
#[cfg(feature = "billing")]
mod billing {
    pub mod invoice_ext;
    pub mod line_item_ext;
    pub mod subscription_ext;
}

#[path = "resources"]
#[cfg(feature = "connect")]
mod connect {
    pub mod login_links_ext;
}

#[path = "resources"]
#[cfg(feature = "fraud")]
mod fraud {
    pub mod review_ext;
}

#[path = "resources"]
#[cfg(feature = "issuing")]
mod issuing {
    pub mod issuing_authorization_ext;
    pub mod issuing_card_ext;
    pub mod issuing_dispute_ext;
    pub mod issuing_merchant_data;
    pub mod issuing_transaction_ext;
}

#[path = "resources"]
#[cfg(feature = "orders")]
mod orders {
    pub mod order_ext;
}

#[path = "resources"]
#[cfg(feature = "webhook-endpoints")]
mod webhook_endpoints {
    pub mod webhook_endpoint_ext;
}

#[rustfmt::skip]
pub use {
    currency::*,
    types::*,

    self::core::{
        // balance_transaction_ext::*,
        // charge_ext::*,
        // customer_ext::*,
        // payment_intent_ext::*,
        // payment_source::*,
        // placeholders::*,
        // payout_ext::*,
        // token_ext::*,
        // setup_intent_ext::*,
    },

    payment::{
        // bank_account_ext::*,
        // card::*,
        // payment_method_ext::*,
        // source_ext::*
    },
};

#[rustfmt::skip]
#[cfg(feature = "events")]
pub use {
    webhook_events::*,
};

#[rustfmt::skip]
#[cfg(feature = "billing")]
pub use {
    billing::{
        invoice_ext::*,
        line_item_ext::*,
        subscription_ext::*
    },
};

#[rustfmt::skip]
#[cfg(feature = "connect")]
pub use {
    connect::{
        login_links_ext::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "fraud")]
pub use {
    fraud::review_ext::*,
};

#[rustfmt::skip]
#[cfg(feature = "issuing")]
pub use {
    issuing::{
        issuing_authorization_ext::*,
        issuing_card_ext::*,
        issuing_dispute_ext::*,
        issuing_merchant_data::*,
        issuing_transaction_ext::*,
    },
};

#[rustfmt::skip]
#[cfg(feature = "orders")]
pub use {
    orders::order_ext::*,
};

#[rustfmt::skip]
#[cfg(feature = "webhook-endpoints")]
pub use {
    webhook_endpoints::webhook_endpoint_ext::*,
};

#[cfg(not(feature = "full"))]
pub use generated::placeholders::*;

/// this struct is just a stub for code not using the "connect" feature
/// see https://github.com/arlyon/async-stripe/issues/49 for more context
/// if there are more features that requires a fully fledged CompanyParams
/// we probably need to update the code generation and move to a shared place
#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CompanyParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}

/// this struct is just a stub for code not using the "connect" feature
/// see https://github.com/arlyon/async-stripe/issues/49 for more context
/// if there are more features that requires a fully fledged PersonParams
/// we probably need to update the code generation and move to a shared place
#[cfg(not(feature = "connect"))]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PersonParams {
    #[serde(default)]
    pub metadata: crate::params::Metadata,
}
