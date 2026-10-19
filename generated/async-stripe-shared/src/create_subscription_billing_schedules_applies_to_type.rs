/// Controls which subscription items the billing schedule applies to.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateSubscriptionBillingSchedulesAppliesToType {
    Price,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateSubscriptionBillingSchedulesAppliesToType {
    pub fn as_str(&self) -> &str {
        use CreateSubscriptionBillingSchedulesAppliesToType::*;
        match self {
            Price => "price",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateSubscriptionBillingSchedulesAppliesToType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionBillingSchedulesAppliesToType::*;
        match s {
            "price" => Ok(Price),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateSubscriptionBillingSchedulesAppliesToType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateSubscriptionBillingSchedulesAppliesToType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateSubscriptionBillingSchedulesAppliesToType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateSubscriptionBillingSchedulesAppliesToType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateSubscriptionBillingSchedulesAppliesToType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateSubscriptionBillingSchedulesAppliesToType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateSubscriptionBillingSchedulesAppliesToType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreateSubscriptionBillingSchedulesAppliesToType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CreateSubscriptionBillingSchedulesAppliesToType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreateSubscriptionBillingSchedulesAppliesToType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionBillingSchedulesAppliesToType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
