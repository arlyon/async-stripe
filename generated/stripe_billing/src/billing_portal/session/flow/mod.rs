#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Flow {
pub after_completion: stripe_billing::billing_portal::session::flow::after_completion::AfterCompletion,
    /// Configuration when `flow.type=subscription_cancel`.
pub subscription_cancel: Option<stripe_billing::billing_portal::session::flow::subscription_cancel::SubscriptionCancel>,
    /// Configuration when `flow.type=subscription_update`.
pub subscription_update: Option<stripe_billing::billing_portal::session::flow::subscription_update::SubscriptionUpdate>,
    /// Configuration when `flow.type=subscription_update_confirm`.
pub subscription_update_confirm: Option<stripe_billing::billing_portal::session::flow::subscription_update_confirm::SubscriptionUpdateConfirm>,
    /// Type of flow that the customer will go through.
#[serde(rename = "type")]
pub type_: FlowType,

}
/// Type of flow that the customer will go through.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FlowType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}

impl FlowType {
    pub fn as_str(self) -> &'static str {
        use FlowType::*;
        match self {
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
        }
    }
}

impl std::str::FromStr for FlowType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FlowType::*;
        match s {
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for FlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for FlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for FlowType"))
    }
}
pub mod after_completion;
pub use after_completion::AfterCompletion;
pub mod subscription_cancel;
pub use subscription_cancel::SubscriptionCancel;
pub mod subscription_update;
pub use subscription_update::SubscriptionUpdate;
pub mod subscription_update_confirm;
pub use subscription_update_confirm::SubscriptionUpdateConfirm;
pub mod discount;
pub use discount::Discount;
pub mod item;
pub use item::Item;
