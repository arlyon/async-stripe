#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    /// A URL for custom mandate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor>>,
    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule:
        Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type:
        Option<SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
}
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    Invoice,
    Subscription,
}

impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::*;
        match self {
            Invoice => "invoice",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor::*;
        match s {
            "invoice" => Ok(Invoice),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultFor"))
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule"))
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}

impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType"))
    }
}
