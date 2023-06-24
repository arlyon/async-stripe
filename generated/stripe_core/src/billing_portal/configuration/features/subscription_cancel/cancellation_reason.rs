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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for CancellationReasonOptions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "customer_service" => Ok(Self::CustomerService),
            "low_quality" => Ok(Self::LowQuality),
            "missing_features" => Ok(Self::MissingFeatures),
            "other" => Ok(Self::Other),
            "switched_service" => Ok(Self::SwitchedService),
            "too_complex" => Ok(Self::TooComplex),
            "too_expensive" => Ok(Self::TooExpensive),
            "unused" => Ok(Self::Unused),

            _ => Err(()),
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
impl serde::Serialize for CancellationReasonOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CancellationReasonOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CancellationReasonOptions"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CancellationReasonOptions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CancellationReasonOptions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CancellationReasonOptions::from_str(s)?);
        Ok(())
    }
}
