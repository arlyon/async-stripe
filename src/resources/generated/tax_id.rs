// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::TaxIdId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::Customer;
use crate::TaxIdType;

/// The resource representing a Stripe "tax_id".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxId {
    /// Unique identifier for the object.
    pub id: TaxIdId,

    /// Two-letter ISO code representing the country of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// ID of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<Expandable<Customer>>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<Box<bool>>,

    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `es_cif`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `th_vat`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    ///
    /// Note that some legacy tax IDs have type `unknown`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Box<TaxIdType>>,

    /// Value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<String>>,

    /// Tax ID verification information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<TaxIdVerification>>,
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
    /// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
    pub status: TaxIdVerificationStatus,

    /// Verified address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_address: Option<Box<String>>,

    /// Verified name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Box<String>>,
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

impl TaxIdVerificationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdVerificationStatus::Pending => "pending",
            TaxIdVerificationStatus::Unavailable => "unavailable",
            TaxIdVerificationStatus::Unverified => "unverified",
            TaxIdVerificationStatus::Verified => "verified",
        }
    }
}

impl AsRef<str> for TaxIdVerificationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdVerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
