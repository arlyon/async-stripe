#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Payment {
    /// ID of the payment intent associated with this order.
    ///
    /// Null when the order is `open`.
    pub payment_intent:
        Option<stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>>,
    /// Settings describing how the order should configure generated PaymentIntents.
    pub settings: Option<stripe_misc::order::payment::settings::Settings>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for PaymentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "complete" => Ok(Self::Complete),
            "not_required" => Ok(Self::NotRequired),
            "processing" => Ok(Self::Processing),
            "requires_action" => Ok(Self::RequiresAction),
            "requires_capture" => Ok(Self::RequiresCapture),
            "requires_confirmation" => Ok(Self::RequiresConfirmation),
            "requires_payment_method" => Ok(Self::RequiresPaymentMethod),

            _ => Err(()),
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
impl serde::Serialize for PaymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentStatus"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
pub mod settings;
pub use settings::Settings;
