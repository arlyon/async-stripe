// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Object};
use crate::resources::{Address, Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructions".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructions {

    pub bank_transfer: FundingInstructionsBankTransfer,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The `funding_type` of the returned instructions.
    pub funding_type: FundingInstructionsFundingType,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl Object for FundingInstructions {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "funding_instructions"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransfer {

    /// The country of the bank account to fund.
    pub country: String,

    /// A list of financial addresses that can be used to fund a particular balance.
    pub financial_addresses: Vec<FundingInstructionsBankTransferFinancialAddress>,

    /// The bank_transfer type.
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransferFinancialAddress {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<FundingInstructionsBankTransferAbaRecord>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<FundingInstructionsBankTransferIbanRecord>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<FundingInstructionsBankTransferSortCodeRecord>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<FundingInstructionsBankTransferSpeiRecord>,

    /// The payment networks supported by this FinancialAddress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<FundingInstructionsBankTransferSwiftRecord>,

    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferFinancialAddressType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<FundingInstructionsBankTransferZenginRecord>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransferAbaRecord {

    pub account_holder_address: Address,

    /// The account holder name.
    pub account_holder_name: String,

    /// The ABA account number.
    pub account_number: String,

    /// The account type.
    pub account_type: String,

    pub bank_address: Address,

    /// The bank name.
    pub bank_name: String,

    /// The ABA routing number.
    pub routing_number: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransferIbanRecord {

    pub account_holder_address: Address,

    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,

    pub bank_address: Address,

    /// The BIC/SWIFT code of the account.
    pub bic: String,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// The IBAN of the account.
    pub iban: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransferSortCodeRecord {

    pub account_holder_address: Address,

    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,

    /// The account number.
    pub account_number: String,

    pub bank_address: Address,

    /// The six-digit sort code.
    pub sort_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransferSpeiRecord {

    pub account_holder_address: Address,

    /// The account holder name.
    pub account_holder_name: String,

    pub bank_address: Address,

    /// The three-digit bank code.
    pub bank_code: String,

    /// The short banking institution name.
    pub bank_name: String,

    /// The CLABE number.
    pub clabe: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransferSwiftRecord {

    pub account_holder_address: Address,

    /// The account holder name.
    pub account_holder_name: String,

    /// The account number.
    pub account_number: String,

    /// The account type.
    pub account_type: String,

    pub bank_address: Address,

    /// The bank name.
    pub bank_name: String,

    /// The SWIFT code.
    pub swift_code: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransferZenginRecord {

    pub account_holder_address: Address,

    /// The account holder name.
    pub account_holder_name: Option<String>,

    /// The account number.
    pub account_number: Option<String>,

    /// The bank account type.
    ///
    /// In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,

    pub bank_address: Address,

    /// The bank code of the account.
    pub bank_code: Option<String>,

    /// The bank name of the account.
    pub bank_name: Option<String>,

    /// The branch code of the account.
    pub branch_code: Option<String>,

    /// The branch name of the account.
    pub branch_name: Option<String>,
}

/// An enum representing the possible values of an `FundingInstructionsBankTransferFinancialAddress`'s `supported_networks` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    Ach,
    Bacs,
    DomesticWireUs,
    Fps,
    Sepa,
    Spei,
    Swift,
    Zengin,
}

impl FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::Ach => "ach",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::Bacs => "bacs",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::DomesticWireUs => "domestic_wire_us",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::Fps => "fps",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::Sepa => "sepa",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::Spei => "spei",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::Swift => "swift",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworks::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn default() -> Self {
        Self::Ach
    }
}

/// An enum representing the possible values of an `FundingInstructionsBankTransferFinancialAddress`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Aba,
    Iban,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

impl FundingInstructionsBankTransferFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            FundingInstructionsBankTransferFinancialAddressType::Aba => "aba",
            FundingInstructionsBankTransferFinancialAddressType::Iban => "iban",
            FundingInstructionsBankTransferFinancialAddressType::SortCode => "sort_code",
            FundingInstructionsBankTransferFinancialAddressType::Spei => "spei",
            FundingInstructionsBankTransferFinancialAddressType::Swift => "swift",
            FundingInstructionsBankTransferFinancialAddressType::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferFinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FundingInstructionsBankTransferFinancialAddressType {
    fn default() -> Self {
        Self::Aba
    }
}

/// An enum representing the possible values of an `FundingInstructionsBankTransfer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}

impl FundingInstructionsBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            FundingInstructionsBankTransferType::EuBankTransfer => "eu_bank_transfer",
            FundingInstructionsBankTransferType::JpBankTransfer => "jp_bank_transfer",
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FundingInstructionsBankTransferType {
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

/// An enum representing the possible values of an `FundingInstructions`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsFundingType {
    BankTransfer,
}

impl FundingInstructionsFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            FundingInstructionsFundingType::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for FundingInstructionsFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FundingInstructionsFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}
