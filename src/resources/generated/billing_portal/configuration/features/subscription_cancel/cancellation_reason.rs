#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    pub options: Vec<CancellationReasonOptions>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CancellationReason {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Which cancellation reasons will be given as options to the customer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl CancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomerService => "customer_service",
            Self::LowQuality => "low_quality",
            Self::MissingFeatures => "missing_features",
            Self::Other => "other",
            Self::SwitchedService => "switched_service",
            Self::TooComplex => "too_complex",
            Self::TooExpensive => "too_expensive",
            Self::Unused => "unused",
        }
    }
}

impl AsRef<str> for CancellationReasonOptions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
