#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    /// Token of the FinancialAccount.
    pub id: String,
    /// The rails used to send funds.
    pub network: OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}
/// The rails used to send funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    Stripe,
}

impl OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::*;
        match self {
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork::*;
        match s {
            "stripe" => Ok(Stripe),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork"))
    }
}
impl stripe_types::Object for OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    type Id = String;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
