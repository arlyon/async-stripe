// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::TaxIdId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::Customer;
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "tax_id".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxId {
    /// Unique identifier for the object.
    pub id: TaxIdId,

    /// Two-letter ISO code representing the country of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// ID of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Type of the tax ID, one of `eu_vat`, `nz_gst`, `au_abn`, or `unknown`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<TaxIdType>,

    /// Value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<TaxIdVerification>,
}

impl Object for TaxId {
    type Id = TaxIdId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax_id"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxIdVerification {
    /// Verification status, one of `pending`, `unavailable`, `unverified`, or `verified`.
    pub status: TaxIdVerificationStatus,

    /// Verified address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_address: Option<String>,

    /// Verified name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
}

/// An enum representing the possible values of an `TaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    AuAbn,
    EuVat,
    NzGst,
    Unknown,
}

/// An enum representing the possible values of an `TaxIdVerification`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdVerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
}
