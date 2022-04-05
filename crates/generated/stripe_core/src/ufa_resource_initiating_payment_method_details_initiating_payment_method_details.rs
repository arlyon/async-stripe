// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::resources::UfaResourceBillingDetails;

/// The resource representing a Stripe "UFAResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    /// Set when `type` is `balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance:
        Option<UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance>,

    pub billing_details: UfaResourceBillingDetails,

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
    pub type_: UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UfaResourceInitiatingPaymentMethodDetailsUsBankAccount>,
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
pub struct UfaResourceInitiatingPaymentMethodDetailsUsBankAccount {
    /// Bank name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// The last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// The routing number for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// An enum representing the possible values of an `UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails`'s `balance` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
}

impl UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    pub fn as_str(self) -> &'static str {
        match self {
            UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance::Payments => "payments",
        }
    }
}

impl AsRef<str> for UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn default() -> Self {
        Self::Payments
    }
}

/// An enum representing the possible values of an `UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}

impl UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Balance => "balance",
            UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::FinancialAccount => "financial_account",
            UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::IssuingCard => "issuing_card",
            UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::Stripe => "stripe",
            UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UfaResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn default() -> Self {
        Self::Balance
    }
}
