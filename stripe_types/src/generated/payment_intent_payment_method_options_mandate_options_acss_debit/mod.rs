#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    /// A URL for custom mandate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,
    /// Payment schedule for the mandate.
    pub payment_schedule:
        Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,
    /// Transaction type of the mandate.
    pub transaction_type:
        Option<PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match self {
            Combined => "combined",
            Interval => "interval",
            Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::*;
        match s {
            "combined" => Ok(Combined),
            "interval" => Ok(Interval),
            "sporadic" => Ok(Sporadic),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule"))
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
}

impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    pub fn as_str(self) -> &'static str {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match self {
            Business => "business",
            Personal => "personal",
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::*;
        match s {
            "business" => Ok(Business),
            "personal" => Ok(Personal),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType"))
    }
}
