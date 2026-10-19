#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingMeterCustomerMappingType {
    ById,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingMeterCustomerMappingType {
    pub fn as_str(&self) -> &str {
        use CreateBillingMeterCustomerMappingType::*;
        match self {
            ById => "by_id",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingMeterCustomerMappingType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingMeterCustomerMappingType::*;
        match s {
            "by_id" => Ok(ById),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingMeterCustomerMappingType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateBillingMeterCustomerMappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateBillingMeterCustomerMappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingMeterCustomerMappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateBillingMeterCustomerMappingType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateBillingMeterCustomerMappingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateBillingMeterCustomerMappingType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreateBillingMeterCustomerMappingType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreateBillingMeterCustomerMappingType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreateBillingMeterCustomerMappingType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingMeterCustomerMappingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
