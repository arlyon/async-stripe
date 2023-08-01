#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Jurisdiction {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// A human-readable name for the jurisdiction imposing the tax.
    pub display_name: String,
    /// Indicates the level of the jurisdiction imposing the tax.
    pub level: JurisdictionLevel,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
}
/// Indicates the level of the jurisdiction imposing the tax.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum JurisdictionLevel {
    City,
    Country,
    County,
    District,
    State,
}

impl JurisdictionLevel {
    pub fn as_str(self) -> &'static str {
        use JurisdictionLevel::*;
        match self {
            City => "city",
            Country => "country",
            County => "county",
            District => "district",
            State => "state",
        }
    }
}

impl std::str::FromStr for JurisdictionLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use JurisdictionLevel::*;
        match s {
            "city" => Ok(City),
            "country" => Ok(Country),
            "county" => Ok(County),
            "district" => Ok(District),
            "state" => Ok(State),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for JurisdictionLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for JurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for JurisdictionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for JurisdictionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for JurisdictionLevel"))
    }
}
