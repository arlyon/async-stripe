#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TransactionShippingCost {
    /// The shipping amount in integer cents.
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for shipping, in integer cents.
    pub amount_tax: i64,
    /// The ID of an existing [ShippingRate](https://stripe.com/docs/api/shipping_rates/object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    /// Specifies whether the `amount` includes taxes.
    ///
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TransactionShippingCostTaxBehavior,
    /// Detailed account of taxes relevant to shipping cost.
    ///
    /// (It is not populated for the transaction resource object and will be removed in the next API version.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_breakdown: Option<Vec<stripe_misc::line_item_tax_breakdown::LineItemTaxBreakdown>>,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for shipping.
    pub tax_code: String,
}
/// Specifies whether the `amount` includes taxes.
///
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionShippingCostTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TransactionShippingCostTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TransactionShippingCostTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for TransactionShippingCostTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionShippingCostTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TransactionShippingCostTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionShippingCostTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TransactionShippingCostTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionShippingCostTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TransactionShippingCostTaxBehavior")
        })
    }
}
