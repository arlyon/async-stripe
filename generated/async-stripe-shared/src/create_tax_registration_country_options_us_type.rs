#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxRegistrationCountryOptionsUsType {
    LocalAmusementTax,
    LocalLeaseTax,
    MassTransitParkingTax,
    ParkingTax,
    StateCommunicationsTax,
    StateRetailDeliveryFee,
    StateSalesTax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTaxRegistrationCountryOptionsUsType {
    pub fn as_str(&self) -> &str {
        use CreateTaxRegistrationCountryOptionsUsType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            MassTransitParkingTax => "mass_transit_parking_tax",
            ParkingTax => "parking_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateRetailDeliveryFee => "state_retail_delivery_fee",
            StateSalesTax => "state_sales_tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUsType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "mass_transit_parking_tax" => Ok(MassTransitParkingTax),
            "parking_tax" => Ok(ParkingTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_retail_delivery_fee" => Ok(StateRetailDeliveryFee),
            "state_sales_tax" => Ok(StateSalesTax),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTaxRegistrationCountryOptionsUsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateTaxRegistrationCountryOptionsUsType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateTaxRegistrationCountryOptionsUsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreateTaxRegistrationCountryOptionsUsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CreateTaxRegistrationCountryOptionsUsType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreateTaxRegistrationCountryOptionsUsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsUsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
