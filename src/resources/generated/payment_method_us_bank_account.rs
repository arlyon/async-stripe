// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_us_bank_account".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodUsBankAccount {

    /// Account holder type: individual or company.
    pub account_holder_type: Option<PaymentMethodUsBankAccountAccountHolderType>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<PaymentMethodUsBankAccountAccountType>,

    /// The name of the bank.
    pub bank_name: Option<String>,

    /// The ID of the Financial Connections Account used to create the payment method.
    pub financial_connections_account: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Contains information about US bank account networks that can be used.
    pub networks: Option<UsBankAccountNetworks>,

    /// Routing number of the bank account.
    pub routing_number: Option<String>,

    /// Contains information about the future reusability of this PaymentMethod.
    pub status_details: Option<PaymentMethodUsBankAccountStatusDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodUsBankAccountStatusDetails {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<PaymentMethodUsBankAccountBlocked>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodUsBankAccountBlocked {

    /// The ACH network code that resulted in this block.
    pub network_code: Option<PaymentMethodUsBankAccountBlockedNetworkCode>,

    /// The reason why this PaymentMethod's fingerprint has been blocked.
    pub reason: Option<PaymentMethodUsBankAccountBlockedReason>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UsBankAccountNetworks {

    /// The preferred network.
    pub preferred: Option<String>,

    /// All supported networks.
    pub supported: Vec<UsBankAccountNetworksSupported>,
}

/// An enum representing the possible values of an `PaymentMethodUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountAccountHolderType::Company => "company",
            PaymentMethodUsBankAccountAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `PaymentMethodUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}

impl PaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountAccountType::Checking => "checking",
            PaymentMethodUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `PaymentMethodUsBankAccountBlocked`'s `network_code` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountBlockedNetworkCode {
    #[serde(rename = "R02")]
    R02,
    #[serde(rename = "R03")]
    R03,
    #[serde(rename = "R04")]
    R04,
    #[serde(rename = "R05")]
    R05,
    #[serde(rename = "R07")]
    R07,
    #[serde(rename = "R08")]
    R08,
    #[serde(rename = "R10")]
    R10,
    #[serde(rename = "R11")]
    R11,
    #[serde(rename = "R16")]
    R16,
    #[serde(rename = "R20")]
    R20,
    #[serde(rename = "R29")]
    R29,
    #[serde(rename = "R31")]
    R31,
}

impl PaymentMethodUsBankAccountBlockedNetworkCode {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountBlockedNetworkCode::R02 => "R02",
            PaymentMethodUsBankAccountBlockedNetworkCode::R03 => "R03",
            PaymentMethodUsBankAccountBlockedNetworkCode::R04 => "R04",
            PaymentMethodUsBankAccountBlockedNetworkCode::R05 => "R05",
            PaymentMethodUsBankAccountBlockedNetworkCode::R07 => "R07",
            PaymentMethodUsBankAccountBlockedNetworkCode::R08 => "R08",
            PaymentMethodUsBankAccountBlockedNetworkCode::R10 => "R10",
            PaymentMethodUsBankAccountBlockedNetworkCode::R11 => "R11",
            PaymentMethodUsBankAccountBlockedNetworkCode::R16 => "R16",
            PaymentMethodUsBankAccountBlockedNetworkCode::R20 => "R20",
            PaymentMethodUsBankAccountBlockedNetworkCode::R29 => "R29",
            PaymentMethodUsBankAccountBlockedNetworkCode::R31 => "R31",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountBlockedNetworkCode {
    fn default() -> Self {
        Self::R02
    }
}

/// An enum representing the possible values of an `PaymentMethodUsBankAccountBlocked`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodUsBankAccountBlockedReason {
    BankAccountClosed,
    BankAccountFrozen,
    BankAccountInvalidDetails,
    BankAccountRestricted,
    BankAccountUnusable,
    DebitNotAuthorized,
}

impl PaymentMethodUsBankAccountBlockedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodUsBankAccountBlockedReason::BankAccountClosed => "bank_account_closed",
            PaymentMethodUsBankAccountBlockedReason::BankAccountFrozen => "bank_account_frozen",
            PaymentMethodUsBankAccountBlockedReason::BankAccountInvalidDetails => "bank_account_invalid_details",
            PaymentMethodUsBankAccountBlockedReason::BankAccountRestricted => "bank_account_restricted",
            PaymentMethodUsBankAccountBlockedReason::BankAccountUnusable => "bank_account_unusable",
            PaymentMethodUsBankAccountBlockedReason::DebitNotAuthorized => "debit_not_authorized",
        }
    }
}

impl AsRef<str> for PaymentMethodUsBankAccountBlockedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodUsBankAccountBlockedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodUsBankAccountBlockedReason {
    fn default() -> Self {
        Self::BankAccountClosed
    }
}

/// An enum representing the possible values of an `UsBankAccountNetworks`'s `supported` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UsBankAccountNetworksSupported {
    Ach,
    UsDomesticWire,
}

impl UsBankAccountNetworksSupported {
    pub fn as_str(self) -> &'static str {
        match self {
            UsBankAccountNetworksSupported::Ach => "ach",
            UsBankAccountNetworksSupported::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for UsBankAccountNetworksSupported {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UsBankAccountNetworksSupported {
    fn default() -> Self {
        Self::Ach
    }
}
