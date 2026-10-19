/// Select how to calculate the end of the invoice item period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    MinItemPeriodEnd,
    PhaseEnd,
    Timestamp,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    pub fn as_str(&self) -> &str {
        use CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType::*;
        match self {
            MinItemPeriodEnd => "min_item_period_end",
            PhaseEnd => "phase_end",
            Timestamp => "timestamp",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType::*;
        match s {
            "min_item_period_end" => Ok(MinItemPeriodEnd),
            "phase_end" => Ok(PhaseEnd),
            "timestamp" => Ok(Timestamp),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionSchedulePhasesAddInvoiceItemsPeriodEndType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
