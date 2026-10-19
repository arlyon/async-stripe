/// Select how to calculate the end of the invoice item period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSubscriptionAddInvoiceItemsPeriodEndType {
    MinItemPeriodEnd,
    Timestamp,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSubscriptionAddInvoiceItemsPeriodEndType {
    pub fn as_str(&self) -> &str {
        use CreateSubscriptionAddInvoiceItemsPeriodEndType::*;
        match self {
            MinItemPeriodEnd => "min_item_period_end",
            Timestamp => "timestamp",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSubscriptionAddInvoiceItemsPeriodEndType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionAddInvoiceItemsPeriodEndType::*;
        match s {
            "min_item_period_end" => Ok(MinItemPeriodEnd),
            "timestamp" => Ok(Timestamp),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSubscriptionAddInvoiceItemsPeriodEndType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSubscriptionAddInvoiceItemsPeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateSubscriptionAddInvoiceItemsPeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionAddInvoiceItemsPeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateSubscriptionAddInvoiceItemsPeriodEndType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateSubscriptionAddInvoiceItemsPeriodEndType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionAddInvoiceItemsPeriodEndType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
