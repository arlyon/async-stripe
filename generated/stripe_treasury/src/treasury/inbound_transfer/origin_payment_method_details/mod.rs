#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OriginPaymentMethodDetails {
pub billing_details: stripe_treasury::treasury::received_credit::initiating_payment_method_details::billing_details::BillingDetails,
    /// The type of the payment method used in the InboundTransfer.
#[serde(rename = "type")]
pub type_: OriginPaymentMethodDetailsType,
#[serde(skip_serializing_if = "Option::is_none")]
pub us_bank_account: Option<stripe_treasury::treasury::inbound_transfer::origin_payment_method_details::us_bank_account::UsBankAccount>,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OriginPaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The type of the payment method used in the InboundTransfer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OriginPaymentMethodDetailsType {
    UsBankAccount,
}

impl OriginPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for OriginPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "us_bank_account" => Ok(Self::UsBankAccount),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for OriginPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OriginPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for OriginPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OriginPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for OriginPaymentMethodDetailsType")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OriginPaymentMethodDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<OriginPaymentMethodDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(OriginPaymentMethodDetailsType::from_str(s)?);
        Ok(())
    }
}
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
