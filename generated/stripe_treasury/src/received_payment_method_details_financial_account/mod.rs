#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    /// The FinancialAccount ID.
    pub id: String,
    /// The rails the ReceivedCredit was sent over.
    ///
    /// A FinancialAccount can only send funds over `stripe`.
    pub network: ReceivedPaymentMethodDetailsFinancialAccountNetwork,
}
/// The rails the ReceivedCredit was sent over.
///
/// A FinancialAccount can only send funds over `stripe`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}

impl ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use ReceivedPaymentMethodDetailsFinancialAccountNetwork::*;
        match self {
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ReceivedPaymentMethodDetailsFinancialAccountNetwork::*;
        match s {
            "stripe" => Ok(Stripe),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedPaymentMethodDetailsFinancialAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for ReceivedPaymentMethodDetailsFinancialAccountNetwork"))
    }
}
