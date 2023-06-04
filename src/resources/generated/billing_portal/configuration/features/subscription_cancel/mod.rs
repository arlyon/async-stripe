#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SubscriptionCancel {
pub cancellation_reason: crate::billing_portal::configuration::features::subscription_cancel::cancellation_reason::CancellationReason,
    /// Whether the feature is enabled.
pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
pub mode: SubscriptionCancelMode,
    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`.
pub proration_behavior: SubscriptionCancelProrationBehavior,

}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SubscriptionCancel {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Whether to create prorations when canceling subscriptions.
///
/// Possible values are `none` and `create_prorations`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
pub mod cancellation_reason;
pub use cancellation_reason::CancellationReason;
