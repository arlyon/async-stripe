#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalFlowsFlow {
    pub after_completion: stripe_billing::PortalFlowsFlowAfterCompletion,
    /// Configuration when `flow.type=subscription_cancel`.
    pub subscription_cancel: Option<stripe_billing::PortalFlowsFlowSubscriptionCancel>,
    /// Configuration when `flow.type=subscription_update`.
    pub subscription_update: Option<stripe_billing::PortalFlowsFlowSubscriptionUpdate>,
    /// Configuration when `flow.type=subscription_update_confirm`.
    pub subscription_update_confirm: Option<stripe_billing::PortalFlowsFlowSubscriptionUpdateConfirm>,
    /// Type of flow that the customer will go through.
    #[serde(rename = "type")]
    pub type_: PortalFlowsFlowType,
}
/// Type of flow that the customer will go through.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalFlowsFlowType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}

impl PortalFlowsFlowType {
    pub fn as_str(self) -> &'static str {
        use PortalFlowsFlowType::*;
        match self {
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
        }
    }
}

impl std::str::FromStr for PortalFlowsFlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsFlowType::*;
        match s {
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PortalFlowsFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalFlowsFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PortalFlowsFlowType"))
    }
}
