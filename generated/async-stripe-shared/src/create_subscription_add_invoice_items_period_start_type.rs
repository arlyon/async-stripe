/// Select how to calculate the start of the invoice item period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSubscriptionAddInvoiceItemsPeriodStartType {
    MaxItemPeriodStart,
    Now,
    Timestamp,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSubscriptionAddInvoiceItemsPeriodStartType {
    pub fn as_str(&self) -> &str {
        use CreateSubscriptionAddInvoiceItemsPeriodStartType::*;
        match self {
            MaxItemPeriodStart => "max_item_period_start",
            Now => "now",
            Timestamp => "timestamp",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSubscriptionAddInvoiceItemsPeriodStartType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionAddInvoiceItemsPeriodStartType::*;
        match s {
            "max_item_period_start" => Ok(MaxItemPeriodStart),
            "now" => Ok(Now),
            "timestamp" => Ok(Timestamp),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSubscriptionAddInvoiceItemsPeriodStartType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSubscriptionAddInvoiceItemsPeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateSubscriptionAddInvoiceItemsPeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionAddInvoiceItemsPeriodStartType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateSubscriptionAddInvoiceItemsPeriodStartType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateSubscriptionAddInvoiceItemsPeriodStartType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionAddInvoiceItemsPeriodStartType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
