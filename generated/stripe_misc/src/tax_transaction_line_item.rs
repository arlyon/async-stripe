#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxTransactionLineItem {
    /// The line item amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for this line item, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_tax: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxTransactionLineItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The ID of an existing [Product](https://stripe.com/docs/api/products/object).
    pub product: Option<String>,
    /// The number of units of the item being purchased. For reversals, this is the quantity reversed.
    pub quantity: u64,
    /// A custom identifier for this line item in the transaction.
    pub reference: String,
    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<stripe_misc::TaxProductResourceTaxTransactionLineItemResourceReversal>,
    /// Specifies whether the `amount` includes taxes.
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxTransactionLineItemTaxBehavior,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
    /// If `reversal`, this line item reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: TaxTransactionLineItemType,
}
/// Specifies whether the `amount` includes taxes.
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxTransactionLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}
impl TaxTransactionLineItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxTransactionLineItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for TaxTransactionLineItemTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxTransactionLineItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxTransactionLineItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxTransactionLineItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TaxTransactionLineItemTaxBehavior")
        })
    }
}
/// If `reversal`, this line item reverses an earlier transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxTransactionLineItemType {
    Reversal,
    Transaction,
}
impl TaxTransactionLineItemType {
    pub fn as_str(self) -> &'static str {
        use TaxTransactionLineItemType::*;
        match self {
            Reversal => "reversal",
            Transaction => "transaction",
        }
    }
}

impl std::str::FromStr for TaxTransactionLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxTransactionLineItemType::*;
        match s {
            "reversal" => Ok(Reversal),
            "transaction" => Ok(Transaction),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxTransactionLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxTransactionLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxTransactionLineItemType"))
    }
}
impl stripe_types::Object for TaxTransactionLineItem {
    type Id = stripe_misc::TaxTransactionLineItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxTransactionLineItemId);
