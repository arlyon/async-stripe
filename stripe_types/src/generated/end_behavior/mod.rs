/// Defines how a subscription behaves when a free trial ends.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: EndBehaviorMissingPaymentMethod,
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl EndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use EndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr for EndBehaviorMissingPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use EndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for EndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for EndBehaviorMissingPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EndBehaviorMissingPaymentMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for EndBehaviorMissingPaymentMethod")
        })
    }
}
