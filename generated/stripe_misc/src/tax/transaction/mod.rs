/// A Tax Transaction records the tax collected from or refunded to your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom#tax-transaction).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,
    pub customer_details: stripe_misc::customer_details::CustomerDetails,
    /// Unique identifier for the transaction.
    pub id: stripe_misc::tax::transaction::TaxTransactionId,
    /// The tax collected or refunded, by line item.
    pub line_items:
        stripe_types::List<stripe_misc::tax::transaction_line_item::TransactionLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransactionObject,
    /// A custom unique identifier, such as 'myOrder_123'.
    pub reference: String,
    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<stripe_misc::reversal::Reversal>,
    /// The shipping cost details for the transaction.
    pub shipping_cost: Option<stripe_misc::tax::transaction_shipping_cost::TransactionShippingCost>,
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: stripe_types::Timestamp,
    /// If `reversal`, this transaction reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: TransactionType,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionObject {
    TaxTransaction,
}

impl TransactionObject {
    pub fn as_str(self) -> &'static str {
        use TransactionObject::*;
        match self {
            TaxTransaction => "tax.transaction",
        }
    }
}

impl std::str::FromStr for TransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionObject::*;
        match s {
            "tax.transaction" => Ok(TaxTransaction),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TransactionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransactionObject"))
    }
}
/// If `reversal`, this transaction reverses an earlier transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionType {
    Reversal,
    Transaction,
}

impl TransactionType {
    pub fn as_str(self) -> &'static str {
        use TransactionType::*;
        match self {
            Reversal => "reversal",
            Transaction => "transaction",
        }
    }
}

impl std::str::FromStr for TransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TransactionType::*;
        match s {
            "reversal" => Ok(Reversal),
            "transaction" => Ok(Transaction),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TransactionType"))
    }
}
impl stripe_types::Object for Transaction {
    type Id = stripe_misc::tax::transaction::TaxTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxTransactionId);
pub mod requests;
