#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CancellationDetails {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    pub comment: Option<String>,
    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    pub feedback: Option<CancellationDetailsFeedback>,
    /// Why this subscription was canceled.
    pub reason: Option<CancellationDetailsReason>,
}
/// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}
impl CancellationDetailsFeedback {
    pub fn as_str(self) -> &'static str {
        use CancellationDetailsFeedback::*;
        match self {
            CustomerService => "customer_service",
            LowQuality => "low_quality",
            MissingFeatures => "missing_features",
            Other => "other",
            SwitchedService => "switched_service",
            TooComplex => "too_complex",
            TooExpensive => "too_expensive",
            Unused => "unused",
        }
    }
}

impl std::str::FromStr for CancellationDetailsFeedback {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancellationDetailsFeedback::*;
        match s {
            "customer_service" => Ok(CustomerService),
            "low_quality" => Ok(LowQuality),
            "missing_features" => Ok(MissingFeatures),
            "other" => Ok(Other),
            "switched_service" => Ok(SwitchedService),
            "too_complex" => Ok(TooComplex),
            "too_expensive" => Ok(TooExpensive),
            "unused" => Ok(Unused),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CancellationDetailsFeedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CancellationDetailsFeedback {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CancellationDetailsFeedback"))
    }
}
/// Why this subscription was canceled.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CancellationDetailsReason {
    CancellationRequested,
    PaymentDisputed,
    PaymentFailed,
}
impl CancellationDetailsReason {
    pub fn as_str(self) -> &'static str {
        use CancellationDetailsReason::*;
        match self {
            CancellationRequested => "cancellation_requested",
            PaymentDisputed => "payment_disputed",
            PaymentFailed => "payment_failed",
        }
    }
}

impl std::str::FromStr for CancellationDetailsReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancellationDetailsReason::*;
        match s {
            "cancellation_requested" => Ok(CancellationRequested),
            "payment_disputed" => Ok(PaymentDisputed),
            "payment_failed" => Ok(PaymentFailed),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CancellationDetailsReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CancellationDetailsReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CancellationDetailsReason"))
    }
}
