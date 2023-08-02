#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Defaults {
    /// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
    pub tax_behavior: Option<DefaultsTaxBehavior>,
    /// Default [tax code](https://stripe.com/docs/tax/tax-categories) used to classify your products and prices.
    pub tax_code: Option<String>,
}
/// Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to specify whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// If the item's price has a tax behavior set, it will take precedence over the default tax behavior.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
}

impl DefaultsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use DefaultsTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            InferredByCurrency => "inferred_by_currency",
        }
    }
}

impl std::str::FromStr for DefaultsTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DefaultsTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "inferred_by_currency" => Ok(InferredByCurrency),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DefaultsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DefaultsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DefaultsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DefaultsTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DefaultsTaxBehavior"))
    }
}
