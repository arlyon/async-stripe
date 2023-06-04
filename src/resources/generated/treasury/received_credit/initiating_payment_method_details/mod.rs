#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InitiatingPaymentMethodDetails {
    /// Set when `type` is `balance`.
#[serde(skip_serializing_if = "Option::is_none")]
pub balance: Option<InitiatingPaymentMethodDetailsBalance>,
pub billing_details: crate::treasury::received_credit::initiating_payment_method_details::billing_details::BillingDetails,
#[serde(skip_serializing_if = "Option::is_none")]
pub financial_account: Option<crate::treasury::received_credit::initiating_payment_method_details::financial_account::FinancialAccount>,
    /// Set when `type` is `issuing_card`.
    ///
    /// This is an [Issuing Card](https://stripe.com/docs/api#issuing_cards) ID.
#[serde(skip_serializing_if = "Option::is_none")]
pub issuing_card: Option<String>,
    /// Polymorphic type matching the originating money movement's source.
    ///
    /// This can be an external account, a Stripe balance, or a FinancialAccount.
#[serde(rename = "type")]
pub type_: InitiatingPaymentMethodDetailsType,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_account: Option<crate::treasury::received_credit::initiating_payment_method_details::us_bank_account::UsBankAccount>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InitiatingPaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Set when `type` is `balance`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum InitiatingPaymentMethodDetailsBalance {
    Payments,
}

impl InitiatingPaymentMethodDetailsBalance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Payments => "payments",
        }
    }
}

impl AsRef<str> for InitiatingPaymentMethodDetailsBalance {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InitiatingPaymentMethodDetailsBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Polymorphic type matching the originating money movement's source.
///
/// This can be an external account, a Stripe balance, or a FinancialAccount.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum InitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}

impl InitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balance => "balance",
            Self::FinancialAccount => "financial_account",
            Self::IssuingCard => "issuing_card",
            Self::Stripe => "stripe",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for InitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InitiatingPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod financial_account;
pub use financial_account::FinancialAccount;
pub mod billing_details;
pub use billing_details::BillingDetails;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
