/// Represents a reader action to refund a payment.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RefundPaymentAction {
    /// The amount being refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Charge that is being refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<stripe_types::Expandable<stripe_types::charge::Charge>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Payment intent that is being refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent:
        Option<stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>>,
    /// The reason for the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundPaymentActionReason>,
    /// Unique identifier for the refund object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<stripe_types::Expandable<stripe_types::refund::Refund>>,
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    ///
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    ///
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    /// A transfer can be reversed only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}
/// The reason for the refund.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RefundPaymentActionReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

impl RefundPaymentActionReason {
    pub fn as_str(self) -> &'static str {
        use RefundPaymentActionReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for RefundPaymentActionReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RefundPaymentActionReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "requested_by_customer" => Ok(RequestedByCustomer),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for RefundPaymentActionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefundPaymentActionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for RefundPaymentActionReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RefundPaymentActionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for RefundPaymentActionReason"))
    }
}
