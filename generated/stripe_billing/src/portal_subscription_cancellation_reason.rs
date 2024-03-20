#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalSubscriptionCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    pub options: Vec<PortalSubscriptionCancellationReasonOptions>,
}
/// Which cancellation reasons will be given as options to the customer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PortalSubscriptionCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}
impl PortalSubscriptionCancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        use PortalSubscriptionCancellationReasonOptions::*;
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

impl std::str::FromStr for PortalSubscriptionCancellationReasonOptions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionCancellationReasonOptions::*;
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
impl std::fmt::Display for PortalSubscriptionCancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalSubscriptionCancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PortalSubscriptionCancellationReasonOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PortalSubscriptionCancellationReasonOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PortalSubscriptionCancellationReasonOptions",
            )
        })
    }
}
