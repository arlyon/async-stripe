#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ServiceType {
    FullService,
    NonFuelTransaction,
    SelfService,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ServiceType {
    pub fn as_str(&self) -> &str {
        use ServiceType::*;
        match self {
            FullService => "full_service",
            NonFuelTransaction => "non_fuel_transaction",
            SelfService => "self_service",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ServiceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ServiceType::*;
        match s {
            "full_service" => Ok(FullService),
            "non_fuel_transaction" => Ok(NonFuelTransaction),
            "self_service" => Ok(SelfService),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ServiceType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ServiceType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ServiceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ServiceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ServiceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ServiceType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ServiceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ServiceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
