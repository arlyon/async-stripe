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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Type of the tax ID, one of `au_abn`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_qst`, `ch_vat`, `es_cif`, `eu_vat`, `hk_br`, `in_gst`, `jp_cn`, `kr_brn`, `li_uid`, `mx_rfc`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ru_inn`, `sg_gst`, `sg_uen`, `th_vat`, `tw_vat`, `us_ein`, or `za_vat`.
    ///
    /// Note that some legacy tax IDs have type `unknown`.
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
    /// Verification status, one of `pending`, `verified`, `unverified`, or `unavailable`.
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
    BrCnpj,
    BrCpf,
    CaBn,
    CaQst,
    ChVat,
    EsCif,
    EuVat,
    HkBr,
    InGst,
    JpCn,
    KrBrn,
    LiUid,
    MxRfc,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    RuInn,
    SgGst,
    SgUen,
    ThVat,
    TwVat,
    Unknown,
    UsEin,
    ZaVat,
}

impl TaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdType::AuAbn => "au_abn",
            TaxIdType::BrCnpj => "br_cnpj",
            TaxIdType::BrCpf => "br_cpf",
            TaxIdType::CaBn => "ca_bn",
            TaxIdType::CaQst => "ca_qst",
            TaxIdType::ChVat => "ch_vat",
            TaxIdType::EsCif => "es_cif",
            TaxIdType::EuVat => "eu_vat",
            TaxIdType::HkBr => "hk_br",
            TaxIdType::InGst => "in_gst",
            TaxIdType::JpCn => "jp_cn",
            TaxIdType::KrBrn => "kr_brn",
            TaxIdType::LiUid => "li_uid",
            TaxIdType::MxRfc => "mx_rfc",
            TaxIdType::MyItn => "my_itn",
            TaxIdType::MySst => "my_sst",
            TaxIdType::NoVat => "no_vat",
            TaxIdType::NzGst => "nz_gst",
            TaxIdType::RuInn => "ru_inn",
            TaxIdType::SgGst => "sg_gst",
            TaxIdType::SgUen => "sg_uen",
            TaxIdType::ThVat => "th_vat",
            TaxIdType::TwVat => "tw_vat",
            TaxIdType::Unknown => "unknown",
            TaxIdType::UsEin => "us_ein",
            TaxIdType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for TaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
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
