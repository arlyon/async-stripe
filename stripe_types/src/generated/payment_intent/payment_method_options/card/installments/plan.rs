#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Plan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: Option<u64>,
    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: Option<PlanInterval>,
    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: PlanType,
}
/// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
/// One of `month`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanInterval {
    Month,
}

impl PlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Month => "month",
        }
    }
}

impl std::str::FromStr for PlanInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "month" => Ok(Self::Month),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PlanInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlanInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanInterval"))
    }
}
/// Type of installment plan, one of `fixed_count`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlanType {
    FixedCount,
}

impl PlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedCount => "fixed_count",
        }
    }
}

impl std::str::FromStr for PlanType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_count" => Ok(Self::FixedCount),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PlanType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlanType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PlanType"))
    }
}
