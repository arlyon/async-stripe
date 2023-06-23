#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandateOptions {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: MandateOptionsAmountType,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    pub end_date: Option<stripe_types::Timestamp>,
    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: MandateOptionsInterval,
    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: String,
    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: stripe_types::Timestamp,
    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    pub supported_types: Option<Vec<MandateOptionsSupportedTypes>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum MandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl MandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for MandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum MandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl MandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Sporadic => "sporadic",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for MandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum MandateOptionsSupportedTypes {
    India,
}

impl MandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl AsRef<str> for MandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
