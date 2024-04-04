#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShippingRateCurrencyOption {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: ShippingRateCurrencyOptionTaxBehavior,
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ShippingRateCurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl ShippingRateCurrencyOptionTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use ShippingRateCurrencyOptionTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ShippingRateCurrencyOptionTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShippingRateCurrencyOptionTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ShippingRateCurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ShippingRateCurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ShippingRateCurrencyOptionTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShippingRateCurrencyOptionTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ShippingRateCurrencyOptionTaxBehavior")
        })
    }
}
