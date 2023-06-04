#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Payment {
    /// ID of the payment intent associated with this order.
    ///
    /// Null when the order is `open`.
    pub payment_intent: Option<crate::Expandable<crate::payment_intent::PaymentIntent>>,
    /// Settings describing how the order should configure generated PaymentIntents.
    pub settings: Option<crate::order::payment::settings::Settings>,
    /// The status of the underlying payment associated with this order, if any.
    ///
    /// Null when the order is `open`.
    pub status: Option<PaymentStatus>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Payment {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The status of the underlying payment associated with this order, if any.
///
/// Null when the order is `open`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Canceled,
    Complete,
    NotRequired,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
}

impl PaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Complete => "complete",
            Self::NotRequired => "not_required",
            Self::Processing => "processing",
            Self::RequiresAction => "requires_action",
            Self::RequiresCapture => "requires_capture",
            Self::RequiresConfirmation => "requires_confirmation",
            Self::RequiresPaymentMethod => "requires_payment_method",
        }
    }
}

impl AsRef<str> for PaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
pub mod settings;
pub use settings::Settings;
