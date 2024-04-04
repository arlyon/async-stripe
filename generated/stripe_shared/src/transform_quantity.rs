#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TransformQuantity {
    /// Divide usage by this number.
    pub divide_by: i64,
    /// After division, either round the result `up` or `down`.
    pub round: TransformQuantityRound,
}
/// After division, either round the result `up` or `down`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TransformQuantityRound {
    Down,
    Up,
}
impl TransformQuantityRound {
    pub fn as_str(self) -> &'static str {
        use TransformQuantityRound::*;
        match self {
            Down => "down",
            Up => "up",
        }
    }
}

impl std::str::FromStr for TransformQuantityRound {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransformQuantityRound::*;
        match s {
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TransformQuantityRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TransformQuantityRound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransformQuantityRound"))
    }
}
