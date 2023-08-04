#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: PaymentMethodOptionsCardMandateOptionsAmountType,
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
    pub interval: PaymentMethodOptionsCardMandateOptionsInterval,
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
    pub supported_types: Option<Vec<PaymentMethodOptionsCardMandateOptionsSupportedTypes>>,
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl PaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCardMandateOptionsAmountType::*;
        match self {
            Fixed => "fixed",
            Maximum => "maximum",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardMandateOptionsAmountType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardMandateOptionsAmountType::*;
        match s {
            "fixed" => Ok(Fixed),
            "maximum" => Ok(Maximum),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardMandateOptionsAmountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsCardMandateOptionsAmountType",
            )
        })
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl PaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCardMandateOptionsInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Sporadic => "sporadic",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardMandateOptionsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardMandateOptionsInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "sporadic" => Ok(Sporadic),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCardMandateOptionsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardMandateOptionsInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsCardMandateOptionsInterval",
            )
        })
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match self {
            India => "india",
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsCardMandateOptionsSupportedTypes::*;
        match s {
            "india" => Ok(India),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodOptionsCardMandateOptionsSupportedTypes",
            )
        })
    }
}
