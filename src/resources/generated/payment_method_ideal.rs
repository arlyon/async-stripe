// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_ideal".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodIdeal {

    /// The customer's bank, if provided.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<PaymentMethodIdealBank>,

    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<PaymentMethodIdealBic>,
}

/// An enum representing the possible values of an `PaymentMethodIdeal`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
}

impl PaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodIdealBank::AbnAmro => "abn_amro",
            PaymentMethodIdealBank::AsnBank => "asn_bank",
            PaymentMethodIdealBank::Bunq => "bunq",
            PaymentMethodIdealBank::Handelsbanken => "handelsbanken",
            PaymentMethodIdealBank::Ing => "ing",
            PaymentMethodIdealBank::Knab => "knab",
            PaymentMethodIdealBank::Moneyou => "moneyou",
            PaymentMethodIdealBank::N26 => "n26",
            PaymentMethodIdealBank::Nn => "nn",
            PaymentMethodIdealBank::Rabobank => "rabobank",
            PaymentMethodIdealBank::Regiobank => "regiobank",
            PaymentMethodIdealBank::Revolut => "revolut",
            PaymentMethodIdealBank::SnsBank => "sns_bank",
            PaymentMethodIdealBank::TriodosBank => "triodos_bank",
            PaymentMethodIdealBank::VanLanschot => "van_lanschot",
            PaymentMethodIdealBank::Yoursafe => "yoursafe",
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}

/// An enum representing the possible values of an `PaymentMethodIdeal`'s `bic` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBic {
    #[serde(rename = "ABNANL2A")]
    Abnanl2a,
    #[serde(rename = "ASNBNL21")]
    Asnbnl21,
    #[serde(rename = "BITSNL2A")]
    Bitsnl2a,
    #[serde(rename = "BUNQNL2A")]
    Bunqnl2a,
    #[serde(rename = "FVLBNL22")]
    Fvlbnl22,
    #[serde(rename = "HANDNL2A")]
    Handnl2a,
    #[serde(rename = "INGBNL2A")]
    Ingbnl2a,
    #[serde(rename = "KNABNL2H")]
    Knabnl2h,
    #[serde(rename = "MOYONL21")]
    Moyonl21,
    #[serde(rename = "NNBANL2G")]
    Nnbanl2g,
    #[serde(rename = "NTSBDEB1")]
    Ntsbdeb1,
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
    #[serde(rename = "REVOIE23")]
    Revoie23,
    #[serde(rename = "REVOLT21")]
    Revolt21,
    #[serde(rename = "SNSBNL2A")]
    Snsbnl2a,
    #[serde(rename = "TRIONL2U")]
    Trionl2u,
}

impl PaymentMethodIdealBic {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodIdealBic::Abnanl2a => "ABNANL2A",
            PaymentMethodIdealBic::Asnbnl21 => "ASNBNL21",
            PaymentMethodIdealBic::Bitsnl2a => "BITSNL2A",
            PaymentMethodIdealBic::Bunqnl2a => "BUNQNL2A",
            PaymentMethodIdealBic::Fvlbnl22 => "FVLBNL22",
            PaymentMethodIdealBic::Handnl2a => "HANDNL2A",
            PaymentMethodIdealBic::Ingbnl2a => "INGBNL2A",
            PaymentMethodIdealBic::Knabnl2h => "KNABNL2H",
            PaymentMethodIdealBic::Moyonl21 => "MOYONL21",
            PaymentMethodIdealBic::Nnbanl2g => "NNBANL2G",
            PaymentMethodIdealBic::Ntsbdeb1 => "NTSBDEB1",
            PaymentMethodIdealBic::Rabonl2u => "RABONL2U",
            PaymentMethodIdealBic::Rbrbnl21 => "RBRBNL21",
            PaymentMethodIdealBic::Revoie23 => "REVOIE23",
            PaymentMethodIdealBic::Revolt21 => "REVOLT21",
            PaymentMethodIdealBic::Snsbnl2a => "SNSBNL2A",
            PaymentMethodIdealBic::Trionl2u => "TRIONL2U",
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodIdealBic {
    fn default() -> Self {
        Self::Abnanl2a
    }
}
