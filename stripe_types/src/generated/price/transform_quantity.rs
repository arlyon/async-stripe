#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransformQuantity {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: TransformQuantityRound,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransformQuantity {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// After division, either round the result `up` or `down`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransformQuantityRound {
    Down,
    Up,
}

impl TransformQuantityRound {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Down => "down",
            Self::Up => "up",
        }
    }
}

impl std::str::FromStr for TransformQuantityRound {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TransformQuantityRound {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransformQuantityRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TransformQuantityRound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransformQuantityRound {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransformQuantityRound"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransformQuantityRound {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TransformQuantityRound> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransformQuantityRound::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
