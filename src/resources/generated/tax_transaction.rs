// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TaxTransactionId};
use crate::params::{List, Metadata, Object, Timestamp};
use crate::resources::{Currency, TaxProductResourceCustomerDetails, TaxProductResourceShippingCost, TaxTransactionLineItem};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductResourceTaxTransaction".
///
/// For more details see <https://stripe.com/docs/api/tax/transactions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxTransaction {
    /// Unique identifier for the transaction.
    pub id: TaxTransactionId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,

    pub customer_details: TaxProductResourceCustomerDetails,

    /// The tax collected or refunded, by line item.
    pub line_items: List<TaxTransactionLineItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// A custom unique identifier, such as 'myOrder_123'.
    pub reference: String,

    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<TaxProductResourceTaxTransactionResourceReversal>,

    /// The shipping cost details for the transaction.
    pub shipping_cost: Option<TaxProductResourceShippingCost>,

    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: Timestamp,

    /// If `reversal`, this transaction reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: TaxTransactionType,
}

impl Object for TaxTransaction {
    type Id = TaxTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax.transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductResourceTaxTransactionResourceReversal {

    /// The `id` of the reversed `Transaction` object.
    pub original_transaction: Option<String>,
}

/// An enum representing the possible values of an `TaxTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxTransactionType {
    Reversal,
    Transaction,
}

impl TaxTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxTransactionType::Reversal => "reversal",
            TaxTransactionType::Transaction => "transaction",
        }
    }
}

impl AsRef<str> for TaxTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxTransactionType {
    fn default() -> Self {
        Self::Reversal
    }
}
