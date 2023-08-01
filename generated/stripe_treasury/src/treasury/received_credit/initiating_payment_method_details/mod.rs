#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InitiatingPaymentMethodDetails {
    /// Set when `type` is `balance`.
#[serde(skip_serializing_if = "Option::is_none")]
pub balance: Option<InitiatingPaymentMethodDetailsBalance>,
pub billing_details: stripe_treasury::treasury::received_credit::initiating_payment_method_details::billing_details::BillingDetails,
#[serde(skip_serializing_if = "Option::is_none")]
pub financial_account: Option<stripe_treasury::treasury::received_credit::initiating_payment_method_details::financial_account::FinancialAccount>,
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
pub us_bank_account: Option<stripe_treasury::treasury::received_credit::initiating_payment_method_details::us_bank_account::UsBankAccount>,

}
/// Set when `type` is `balance`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InitiatingPaymentMethodDetailsBalance {
    Payments,
}

impl InitiatingPaymentMethodDetailsBalance {
    pub fn as_str(self) -> &'static str {
        use InitiatingPaymentMethodDetailsBalance::*;
        match self {
            Payments => "payments",
        }
    }
}

impl std::str::FromStr for InitiatingPaymentMethodDetailsBalance {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InitiatingPaymentMethodDetailsBalance::*;
        match s {
            "payments" => Ok(Payments),
            _ => Err(()),
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
impl serde::Serialize for InitiatingPaymentMethodDetailsBalance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InitiatingPaymentMethodDetailsBalance {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for InitiatingPaymentMethodDetailsBalance")
        })
    }
}
/// Polymorphic type matching the originating money movement's source.
///
/// This can be an external account, a Stripe balance, or a FinancialAccount.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}

impl InitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use InitiatingPaymentMethodDetailsType::*;
        match self {
            Balance => "balance",
            FinancialAccount => "financial_account",
            IssuingCard => "issuing_card",
            Stripe => "stripe",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for InitiatingPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InitiatingPaymentMethodDetailsType::*;
        match s {
            "balance" => Ok(Balance),
            "financial_account" => Ok(FinancialAccount),
            "issuing_card" => Ok(IssuingCard),
            "stripe" => Ok(Stripe),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
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
impl serde::Serialize for InitiatingPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InitiatingPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for InitiatingPaymentMethodDetailsType")
        })
    }
}
pub mod financial_account;
pub use financial_account::FinancialAccount;
pub mod billing_details;
pub use billing_details::BillingDetails;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
