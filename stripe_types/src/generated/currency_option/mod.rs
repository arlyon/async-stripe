#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CurrencyOption {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: CurrencyOptionTaxBehavior,
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CurrencyOptionTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CurrencyOptionTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CurrencyOptionTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CurrencyOptionTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CurrencyOptionTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CurrencyOptionTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CurrencyOptionTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CurrencyOptionTaxBehavior"))
    }
}
