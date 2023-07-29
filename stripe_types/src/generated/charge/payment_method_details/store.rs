#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Store {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<StoreChain>,
}
/// The name of the convenience store chain where the payment was completed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}

impl StoreChain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Familymart => "familymart",
            Self::Lawson => "lawson",
            Self::Ministop => "ministop",
            Self::Seicomart => "seicomart",
        }
    }
}

impl std::str::FromStr for StoreChain {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "familymart" => Ok(Self::Familymart),
            "lawson" => Ok(Self::Lawson),
            "ministop" => Ok(Self::Ministop),
            "seicomart" => Ok(Self::Seicomart),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for StoreChain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for StoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for StoreChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StoreChain {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for StoreChain"))
    }
}
