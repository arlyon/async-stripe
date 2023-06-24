#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AcssDebit {
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<AcssDebitDefaultFor>>,
    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule: AcssDebitPaymentSchedule,
    /// Transaction type of the mandate.
    pub transaction_type: AcssDebitTransactionType,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebit {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitDefaultFor {
    Invoice,
    Subscription,
}

impl AcssDebitDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for AcssDebitDefaultFor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invoice" => Ok(Self::Invoice),
            "subscription" => Ok(Self::Subscription),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AcssDebitDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitDefaultFor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitDefaultFor"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitDefaultFor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitDefaultFor> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitDefaultFor::from_str(s)?);
        Ok(())
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl AcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for AcssDebitPaymentSchedule {
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

impl AsRef<str> for AcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitPaymentSchedule"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitPaymentSchedule::from_str(s)?);
        Ok(())
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcssDebitTransactionType {
    Business,
    Personal,
}

impl AcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl std::str::FromStr for AcssDebitTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business" => Ok(Self::Business),
            "personal" => Ok(Self::Personal),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for AcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for AcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AcssDebitTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AcssDebitTransactionType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AcssDebitTransactionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<AcssDebitTransactionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AcssDebitTransactionType::from_str(s)?);
        Ok(())
    }
}
