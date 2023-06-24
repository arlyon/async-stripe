#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Networks {
    /// The preferred network.
    pub preferred: Option<String>,
    /// All supported networks.
    pub supported: Vec<NetworksSupported>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Networks {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// All supported networks.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NetworksSupported {
    Ach,
    UsDomesticWire,
}

impl NetworksSupported {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr for NetworksSupported {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach" => Ok(Self::Ach),
            "us_domestic_wire" => Ok(Self::UsDomesticWire),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for NetworksSupported {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for NetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for NetworksSupported {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NetworksSupported {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for NetworksSupported"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for NetworksSupported {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<NetworksSupported> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(NetworksSupported::from_str(s)?);
        Ok(())
    }
}
