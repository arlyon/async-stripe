/// Type of flow that the customer will go through.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalFlowsFlowType {
    CustomerUpdate,
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalFlowsFlowType {
    pub fn as_str(&self) -> &str {
        use PortalFlowsFlowType::*;
        match self {
            CustomerUpdate => "customer_update",
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalFlowsFlowType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsFlowType::*;
        match s {
            "customer_update" => Ok(CustomerUpdate),
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PortalFlowsFlowType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalFlowsFlowType)).finish_non_exhaustive()
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
impl miniserde::Deserialize for PortalFlowsFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PortalFlowsFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsFlowType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PortalFlowsFlowType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
