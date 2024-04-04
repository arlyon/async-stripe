#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsEurope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsEuStandard>,
    /// Type of registration in an EU country.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsEuropeType,
}
/// Type of registration in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsEuropeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl TaxProductRegistrationsResourceCountryOptionsEuropeType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsEuropeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsEuropeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceCountryOptionsEuropeType",
            )
        })
    }
}
