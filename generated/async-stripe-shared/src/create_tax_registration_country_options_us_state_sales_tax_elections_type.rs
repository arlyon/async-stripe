/// The type of the election for the state sales tax registration.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    LocalUseTax,
    SimplifiedSellersUseTax,
    SingleLocalUseTax,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    pub fn as_str(&self) -> &str {
        use CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType::*;
        match self {
            LocalUseTax => "local_use_tax",
            SimplifiedSellersUseTax => "simplified_sellers_use_tax",
            SingleLocalUseTax => "single_local_use_tax",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType::*;
        match s {
            "local_use_tax" => Ok(LocalUseTax),
            "simplified_sellers_use_tax" => Ok(SimplifiedSellersUseTax),
            "single_local_use_tax" => Ok(SingleLocalUseTax),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
