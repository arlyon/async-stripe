#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionCancel {
pub cancellation_reason: stripe_billing::billing_portal::configuration::features::subscription_cancel::cancellation_reason::CancellationReason,
    /// Whether the feature is enabled.
pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
pub mode: SubscriptionCancelMode,
    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`.
pub proration_behavior: SubscriptionCancelProrationBehavior,

}
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

impl SubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtPeriodEnd => "at_period_end",
            Self::Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for SubscriptionCancelMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "at_period_end" => Ok(Self::AtPeriodEnd),
            "immediately" => Ok(Self::Immediately),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionCancelMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SubscriptionCancelMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionCancelMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SubscriptionCancelMode"))
    }
}
/// Whether to create prorations when canceling subscriptions.
///
/// Possible values are `none` and `create_prorations`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl SubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for SubscriptionCancelProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always_invoice" => Ok(Self::AlwaysInvoice),
            "create_prorations" => Ok(Self::CreateProrations),
            "none" => Ok(Self::None),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SubscriptionCancelProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SubscriptionCancelProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionCancelProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SubscriptionCancelProrationBehavior")
        })
    }
}
pub mod cancellation_reason;
pub use cancellation_reason::CancellationReason;
