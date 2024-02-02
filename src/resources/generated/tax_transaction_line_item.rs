// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TaxTransactionLineItemId};
use crate::params::{Metadata, Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceTaxTransactionLineItem".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxTransactionLineItem {
    /// Unique identifier for the object.
    pub id: TaxTransactionLineItemId,

    /// The line item amount in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// If `tax_behavior=inclusive`, then this amount includes taxes.
    /// Otherwise, taxes were calculated on top of this amount.
    pub amount: i64,

    /// The amount of tax calculated for this line item, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_tax: i64,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// The ID of an existing [Product](https://stripe.com/docs/api/products/object).
    pub product: Option<String>,

    /// The number of units of the item being purchased.
    ///
    /// For reversals, this is the quantity reversed.
    pub quantity: u64,

    /// A custom identifier for this line item in the transaction.
    pub reference: String,

    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<TaxProductResourceTaxTransactionLineItemResourceReversal>,

    /// Specifies whether the `amount` includes taxes.
    ///
    /// If `tax_behavior=inclusive`, then the amount includes taxes.
    pub tax_behavior: TaxTransactionLineItemTaxBehavior,

    /// The [tax code](https://stripe.com/docs/tax/tax-categories) ID used for this resource.
    pub tax_code: String,

    /// If `reversal`, this line item reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: TaxTransactionLineItemType,
}

impl Object for TaxTransactionLineItem {
    type Id = TaxTransactionLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax.transaction_line_item"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversal {

    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: String,
}

/// An enum representing the possible values of an `TaxTransactionLineItem`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxTransactionLineItemTaxBehavior {
    Exclusive,
    Inclusive,
}

impl TaxTransactionLineItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxTransactionLineItemTaxBehavior::Exclusive => "exclusive",
            TaxTransactionLineItemTaxBehavior::Inclusive => "inclusive",
        }
    }
}

impl AsRef<str> for TaxTransactionLineItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxTransactionLineItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxTransactionLineItemTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `TaxTransactionLineItem`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxTransactionLineItemType {
    Reversal,
    Transaction,
}

impl TaxTransactionLineItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxTransactionLineItemType::Reversal => "reversal",
            TaxTransactionLineItemType::Transaction => "transaction",
        }
    }
}

impl AsRef<str> for TaxTransactionLineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxTransactionLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxTransactionLineItemType {
    fn default() -> Self {
        Self::Reversal
    }
}
