#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxProductResourceTaxTransactionLineItem {
    /// The line item amount in integer cents.
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,
    /// The amount of tax calculated for this line item, in integer cents.
    pub amount_tax: i64,
    /// Unique identifier for the object.
    pub id: stripe_misc::tax_product_resource_tax_transaction_line_item::TaxTransactionLineItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The ID of an existing [Product](https://stripe.com/docs/api/products/object).
    pub product: Option<String>,
    /// The number of units of the item being purchased.
    ///
    /// For reversals, this is the quantity reversed.
    pub quantity: u64,
    /// A custom identifier for this line item in the transaction.
    pub reference: String,
    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<stripe_misc::TaxProductResourceTaxTransactionLineItemResourceReversal>,
    /// Specifies whether the `amount` includes taxes.
    ///
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxProductResourceTaxTransactionLineItemTaxBehavior,
    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,
    /// If `reversal`, this line item reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: TaxProductResourceTaxTransactionLineItemType,
}
/// Specifies whether the `amount` includes taxes.
///
/// If `tax_behavior=inclusive`, then the amount includes taxes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxTransactionLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TaxProductResourceTaxTransactionLineItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxTransactionLineItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxTransactionLineItemTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxTransactionLineItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxTransactionLineItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxTransactionLineItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxTransactionLineItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceTaxTransactionLineItemTaxBehavior",
            )
        })
    }
}
/// If `reversal`, this line item reverses an earlier transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductResourceTaxTransactionLineItemType {
    Reversal,
    Transaction,
}

impl TaxProductResourceTaxTransactionLineItemType {
    pub fn as_str(self) -> &'static str {
        use TaxProductResourceTaxTransactionLineItemType::*;
        match self {
            Reversal => "reversal",
            Transaction => "transaction",
        }
    }
}

impl std::str::FromStr for TaxProductResourceTaxTransactionLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceTaxTransactionLineItemType::*;
        match s {
            "reversal" => Ok(Reversal),
            "transaction" => Ok(Transaction),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxProductResourceTaxTransactionLineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductResourceTaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceTaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxProductResourceTaxTransactionLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxProductResourceTaxTransactionLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductResourceTaxTransactionLineItemType",
            )
        })
    }
}
impl stripe_types::Object for TaxProductResourceTaxTransactionLineItem {
    type Id = stripe_misc::tax_product_resource_tax_transaction_line_item::TaxTransactionLineItemId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(TaxTransactionLineItemId);
