#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsBankAccountNetworks {
    /// The preferred network.
    pub preferred: Option<String>,
    /// All supported networks.
    pub supported: Vec<UsBankAccountNetworksSupported>,
}
/// All supported networks.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UsBankAccountNetworksSupported {
    Ach,
    UsDomesticWire,
}
impl UsBankAccountNetworksSupported {
    pub fn as_str(self) -> &'static str {
        use UsBankAccountNetworksSupported::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for UsBankAccountNetworksSupported {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UsBankAccountNetworksSupported::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UsBankAccountNetworksSupported {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UsBankAccountNetworksSupported {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UsBankAccountNetworksSupported")
        })
    }
}
