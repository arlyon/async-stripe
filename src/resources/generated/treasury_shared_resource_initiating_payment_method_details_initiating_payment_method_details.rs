// ======================================
// This file was automatically generated.
// ======================================

use crate::resources::{TreasurySharedResourceBillingDetails};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {

    /// Set when `type` is `balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance>,

    pub billing_details: TreasurySharedResourceBillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<ReceivedPaymentMethodDetailsFinancialAccount>,

    /// Set when `type` is `issuing_card`.
    ///
    /// This is an [Issuing Card](https://stripe.com/docs/api#issuing_cards) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<String>,

    /// Polymorphic type matching the originating money movement's source.
    ///
    /// This can be an external account, a Stripe balance, or a FinancialAccount.
    #[serde(rename = "type")]
    pub type_: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {

    /// The FinancialAccount ID.
    pub id: String,

    /// The rails the ReceivedCredit was sent over.
    ///
    /// A FinancialAccount can only send funds over `stripe`.
    pub network: ReceivedPaymentMethodDetailsFinancialAccountNetwork,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount {

    /// Bank name.
    pub bank_name: Option<String>,

    /// The last four digits of the bank account number.
    pub last4: Option<String>,

    /// The routing number for the bank account.
    pub routing_number: Option<String>,
}

/// An enum representing the possible values of an `ReceivedPaymentMethodDetailsFinancialAccount`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}

impl ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            ReceivedPaymentMethodDetailsFinancialAccountNetwork::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn default() -> Self {
        Self::Stripe
    }
}

/// An enum representing the possible values of an `TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails`'s `balance` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
}

impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::Payments => "payments",
        }
    }
}

impl AsRef<str> for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn default() -> Self {
        Self::Payments
    }
}

/// An enum representing the possible values of an `TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}

impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Balance => "balance",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::FinancialAccount => "financial_account",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::IssuingCard => "issuing_card",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Stripe => "stripe",
            TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn default() -> Self {
        Self::Balance
    }
}
