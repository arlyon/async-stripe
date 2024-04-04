#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsEuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme,
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme"))
    }
}
