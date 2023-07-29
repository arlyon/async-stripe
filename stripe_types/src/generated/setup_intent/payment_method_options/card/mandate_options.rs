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
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for MandateOptionsAmountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed" => Ok(Self::Fixed),
            "maximum" => Ok(Self::Maximum),

            _ => Err(()),
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
impl serde::Serialize for MandateOptionsAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateOptionsAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for MandateOptionsAmountType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptionsAmountType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateOptionsAmountType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsAmountType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for MandateOptionsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "day" => Ok(Self::Day),
            "month" => Ok(Self::Month),
            "sporadic" => Ok(Self::Sporadic),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),

            _ => Err(()),
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
impl serde::Serialize for MandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateOptionsInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for MandateOptionsInterval"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptionsInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateOptionsInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsInterval::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for MandateOptionsSupportedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "india" => Ok(Self::India),

            _ => Err(()),
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
impl serde::Serialize for MandateOptionsSupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateOptionsSupportedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for MandateOptionsSupportedTypes"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptionsSupportedTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateOptionsSupportedTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsSupportedTypes::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
