#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OutboundPaymentsPaymentMethodDetails {
    pub billing_details: stripe_treasury::TreasurySharedResourceBillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
    /// The type of the payment method used in the OutboundPayment.
    #[serde(rename = "type")]
    pub type_: OutboundPaymentsPaymentMethodDetailsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}
/// The type of the payment method used in the OutboundPayment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutboundPaymentsPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
}

impl OutboundPaymentsPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        use OutboundPaymentsPaymentMethodDetailsType::*;
        match self {
            FinancialAccount => "financial_account",
            UsBankAccount => "us_bank_account",
        }
    }
}

impl std::str::FromStr for OutboundPaymentsPaymentMethodDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OutboundPaymentsPaymentMethodDetailsType::*;
        match s {
            "financial_account" => Ok(FinancialAccount),
            "us_bank_account" => Ok(UsBankAccount),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for OutboundPaymentsPaymentMethodDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OutboundPaymentsPaymentMethodDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for OutboundPaymentsPaymentMethodDetailsType"))
    }
}
