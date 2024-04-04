#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsSimplified {
    /// Type of registration in `country`.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsSimplifiedType,
}
/// Type of registration in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    Simplified,
}
impl TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsSimplifiedType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsSimplifiedType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceCountryOptionsSimplifiedType",
            )
        })
    }
}
