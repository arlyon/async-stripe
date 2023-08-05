/// Defines how a subscription behaves when a free trial ends.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsTrialsResourceEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod,
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionsTrialsResourceEndBehaviorMissingPaymentMethod",
            )
        })
    }
}
