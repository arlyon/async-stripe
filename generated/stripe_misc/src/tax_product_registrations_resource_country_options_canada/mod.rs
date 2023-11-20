#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsCanada {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_standard:
        Option<stripe_misc::TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,
    /// Type of registration in Canada.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsCanadaType,
}
/// Type of registration in Canada.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsCanadaType {
    ProvinceStandard,
    Simplified,
    Standard,
}

impl TaxProductRegistrationsResourceCountryOptionsCanadaType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsCanadaType::*;
        match self {
            ProvinceStandard => "province_standard",
            Simplified => "simplified",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsCanadaType::*;
        match s {
            "province_standard" => Ok(ProvinceStandard),
            "simplified" => Ok(Simplified),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceCountryOptionsCanadaType",
            )
        })
    }
}
