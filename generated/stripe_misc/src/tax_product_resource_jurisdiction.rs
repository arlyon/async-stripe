#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceJurisdiction {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// A human-readable name for the jurisdiction imposing the tax.
    pub display_name: String,
    /// Indicates the level of the jurisdiction imposing the tax.
    pub level: TaxProductResourceJurisdictionLevel,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
}
/// Indicates the level of the jurisdiction imposing the tax.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceJurisdictionLevel {
    City,
    Country,
    County,
    District,
    State,
}
impl TaxProductResourceJurisdictionLevel {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceJurisdictionLevel::*;
        match self {
            City => "city",
            Country => "country",
            County => "county",
            District => "district",
            State => "state",
        }
    }
}

impl std::str::FromStr for TaxProductResourceJurisdictionLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceJurisdictionLevel::*;
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
impl std::fmt::Display for TaxProductResourceJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceJurisdictionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceJurisdictionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TaxProductResourceJurisdictionLevel")
        })
    }
}
