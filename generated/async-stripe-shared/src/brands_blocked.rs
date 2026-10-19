/// The card brands to block.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BrandsBlocked {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BrandsBlocked {
    pub fn as_str(&self) -> &str {
        use BrandsBlocked::*;
        match self {
            AmericanExpress => "american_express",
            DiscoverGlobalNetwork => "discover_global_network",
            Mastercard => "mastercard",
            Visa => "visa",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BrandsBlocked {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BrandsBlocked::*;
        match s {
            "american_express" => Ok(AmericanExpress),
            "discover_global_network" => Ok(DiscoverGlobalNetwork),
            "mastercard" => Ok(Mastercard),
            "visa" => Ok(Visa),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "BrandsBlocked");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BrandsBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BrandsBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BrandsBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BrandsBlocked)).finish_non_exhaustive()
    }
}
impl serde::Serialize for BrandsBlocked {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BrandsBlocked {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BrandsBlocked> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BrandsBlocked::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BrandsBlocked);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BrandsBlocked {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
