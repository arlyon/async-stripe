/// A Tax Transaction records the tax collected from or refunded to your customer.
///
/// Related guide: [Calculate tax in your custom payment flow](https://stripe.com/docs/tax/custom#tax-transaction).
///
/// For more details see <<https://stripe.com/docs/api/tax/transactions/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxTransaction {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of an existing [Customer](https://stripe.com/docs/api/customers/object) used for the resource.
    pub customer: Option<String>,
    pub customer_details: stripe_misc::TaxProductResourceCustomerDetails,
    /// Unique identifier for the transaction.
    pub id: stripe_misc::TaxTransactionId,
    /// The tax collected or refunded, by line item.
    pub line_items: Option<stripe_types::List<stripe_misc::TaxTransactionLineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A custom unique identifier, such as 'myOrder_123'.
    pub reference: String,
    /// If `type=reversal`, contains information about what was reversed.
    pub reversal: Option<stripe_misc::TaxProductResourceTaxTransactionResourceReversal>,
    /// The shipping cost details for the transaction.
    pub shipping_cost: Option<stripe_misc::TaxProductResourceTaxTransactionShippingCost>,
    /// Timestamp of date at which the tax rules and rates in effect applies for the calculation.
    pub tax_date: stripe_types::Timestamp,
    /// If `reversal`, this transaction reverses an earlier transaction.
    #[serde(rename = "type")]
    pub type_: TaxTransactionType,
}
/// If `reversal`, this transaction reverses an earlier transaction.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxTransactionType {
    Reversal,
    Transaction,
}
impl TaxTransactionType {
    pub fn as_str(self) -> &'static str {
        use TaxTransactionType::*;
        match self {
            Reversal => "reversal",
            Transaction => "transaction",
        }
    }
}

impl std::str::FromStr for TaxTransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxTransactionType::*;
        match s {
            "reversal" => Ok(Reversal),
            "transaction" => Ok(Transaction),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TaxTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxTransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxTransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxTransactionType"))
    }
}
impl stripe_types::Object for TaxTransaction {
    type Id = stripe_misc::TaxTransactionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(TaxTransactionId);
