// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_fpx".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodFpx {

    /// Account holder type, if provided.
    ///
    /// Can be one of `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodFpxAccountHolderType>,

    /// The customer's bank, if provided.
    ///
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
    pub bank: PaymentMethodFpxBank,
}

/// An enum representing the possible values of an `PaymentMethodFpx`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodFpxAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodFpxAccountHolderType::Company => "company",
            PaymentMethodFpxAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodFpxAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `PaymentMethodFpx`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl PaymentMethodFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodFpxBank::AffinBank => "affin_bank",
            PaymentMethodFpxBank::Agrobank => "agrobank",
            PaymentMethodFpxBank::AllianceBank => "alliance_bank",
            PaymentMethodFpxBank::Ambank => "ambank",
            PaymentMethodFpxBank::BankIslam => "bank_islam",
            PaymentMethodFpxBank::BankMuamalat => "bank_muamalat",
            PaymentMethodFpxBank::BankOfChina => "bank_of_china",
            PaymentMethodFpxBank::BankRakyat => "bank_rakyat",
            PaymentMethodFpxBank::Bsn => "bsn",
            PaymentMethodFpxBank::Cimb => "cimb",
            PaymentMethodFpxBank::DeutscheBank => "deutsche_bank",
            PaymentMethodFpxBank::HongLeongBank => "hong_leong_bank",
            PaymentMethodFpxBank::Hsbc => "hsbc",
            PaymentMethodFpxBank::Kfh => "kfh",
            PaymentMethodFpxBank::Maybank2e => "maybank2e",
            PaymentMethodFpxBank::Maybank2u => "maybank2u",
            PaymentMethodFpxBank::Ocbc => "ocbc",
            PaymentMethodFpxBank::PbEnterprise => "pb_enterprise",
            PaymentMethodFpxBank::PublicBank => "public_bank",
            PaymentMethodFpxBank::Rhb => "rhb",
            PaymentMethodFpxBank::StandardChartered => "standard_chartered",
            PaymentMethodFpxBank::Uob => "uob",
        }
    }
}

impl AsRef<str> for PaymentMethodFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodFpxBank {
    fn default() -> Self {
        Self::AffinBank
    }
}
