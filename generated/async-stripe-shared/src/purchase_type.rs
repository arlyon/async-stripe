#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PurchaseType {
    FuelAndNonFuelPurchase,
    FuelPurchase,
    NonFuelPurchase,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PurchaseType {
    pub fn as_str(&self) -> &str {
        use PurchaseType::*;
        match self {
            FuelAndNonFuelPurchase => "fuel_and_non_fuel_purchase",
            FuelPurchase => "fuel_purchase",
            NonFuelPurchase => "non_fuel_purchase",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PurchaseType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PurchaseType::*;
        match s {
            "fuel_and_non_fuel_purchase" => Ok(FuelAndNonFuelPurchase),
            "fuel_purchase" => Ok(FuelPurchase),
            "non_fuel_purchase" => Ok(NonFuelPurchase),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PurchaseType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PurchaseType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PurchaseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PurchaseType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PurchaseType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PurchaseType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PurchaseType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PurchaseType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
