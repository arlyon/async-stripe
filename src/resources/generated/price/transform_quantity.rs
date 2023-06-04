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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
