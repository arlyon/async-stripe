#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InitiatingPaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Set when `type` is `balance`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for InitiatingPaymentMethodDetailsBalance {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "payments" => Ok(Self::Payments),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for InitiatingPaymentMethodDetailsBalance")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InitiatingPaymentMethodDetailsBalance {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InitiatingPaymentMethodDetailsBalance> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(InitiatingPaymentMethodDetailsBalance::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
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
        match self {
            Self::Balance => "balance",
            Self::FinancialAccount => "financial_account",
            Self::IssuingCard => "issuing_card",
            Self::Stripe => "stripe",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for InitiatingPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "balance" => Ok(Self::Balance),
            "financial_account" => Ok(Self::FinancialAccount),
            "issuing_card" => Ok(Self::IssuingCard),
            "stripe" => Ok(Self::Stripe),
            "us_bank_account" => Ok(Self::UsBankAccount),

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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for InitiatingPaymentMethodDetailsType")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InitiatingPaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<InitiatingPaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(InitiatingPaymentMethodDetailsType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod financial_account;
pub use financial_account::FinancialAccount;
pub mod billing_details;
pub use billing_details::BillingDetails;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
