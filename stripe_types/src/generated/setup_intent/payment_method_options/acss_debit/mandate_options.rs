#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandateOptions {
    /// A URL for custom mandate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<MandateOptionsDefaultFor>>,
    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: Option<MandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type: Option<MandateOptionsTransactionType>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl MandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for MandateOptionsDefaultFor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invoice" => Ok(Self::Invoice),
            "subscription" => Ok(Self::Subscription),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for MandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for MandateOptionsDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateOptionsDefaultFor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for MandateOptionsDefaultFor"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptionsDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateOptionsDefaultFor> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsDefaultFor::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl MandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for MandateOptionsPaymentSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combined" => Ok(Self::Combined),
            "interval" => Ok(Self::Interval),
            "sporadic" => Ok(Self::Sporadic),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for MandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for MandateOptionsPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateOptionsPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for MandateOptionsPaymentSchedule")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptionsPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateOptionsPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsPaymentSchedule::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MandateOptionsTransactionType {
    Business,
    Personal,
}

impl MandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl std::str::FromStr for MandateOptionsTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business" => Ok(Self::Business),
            "personal" => Ok(Self::Personal),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for MandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for MandateOptionsTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateOptionsTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for MandateOptionsTransactionType")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateOptionsTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateOptionsTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateOptionsTransactionType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
