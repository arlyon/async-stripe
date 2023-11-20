#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefault {
    /// Type of registration in `country`.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsDefaultType,
}
/// Type of registration in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultType {
    Standard,
}

impl TaxProductRegistrationsResourceCountryOptionsDefaultType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsDefaultType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsDefaultType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceCountryOptionsDefaultType",
            )
        })
    }
}
