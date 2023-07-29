#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Bancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    pub preferred_language: BancontactPreferredLanguage,
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl BancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl std::str::FromStr for BancontactPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "nl" => Ok(Self::Nl),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for BancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for BancontactPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BancontactPreferredLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for BancontactPreferredLanguage"))
    }
}
