// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::AlipayAccountId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Currency, Customer};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "AlipayAccount".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlipayAccount {
    /// Unique identifier for the object.
    pub id: AlipayAccountId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Uniquely identifies the account and will be the same across all Alipay account objects that are linked to the same Alipay account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// If the Alipay account object is not reusable, the exact amount that you can create a charge for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_amount: Option<i64>,

    /// If the Alipay account object is not reusable, the exact currency that you can create a charge for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_currency: Option<Currency>,

    /// True if you can create multiple payments using this account.
    ///
    /// If the account is reusable, then you can freely choose the amount of each payment.
    #[serde(default)]
    pub reusable: bool,

    /// Whether this Alipay account object has ever been used for a payment.
    #[serde(default)]
    pub used: bool,

    /// The username for the Alipay account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl Object for AlipayAccount {
    type Id = AlipayAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "alipay_account"
    }
}
