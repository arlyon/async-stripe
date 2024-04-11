#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTaxTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTaxTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTaxTransaction<'a> {
    /// Retrieves a Tax `Transaction` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        transaction: &stripe_misc::TaxTransactionId,
    ) -> stripe::Response<stripe_misc::TaxTransaction> {
        client.get_query(&format!("/tax/transactions/{transaction}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsTaxTransaction<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListLineItemsTaxTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListLineItemsTaxTransaction<'a> {
    /// Retrieves the line items of a committed standalone transaction as a collection.
    pub fn send(
        &self,
        client: &stripe::Client,
        transaction: &stripe_misc::TaxTransactionId,
    ) -> stripe::Response<stripe_types::List<stripe_misc::TaxTransactionLineItem>> {
        client.get_query(&format!("/tax/transactions/{transaction}/line_items"), self)
    }
    pub fn paginate(
        self,
        transaction: &stripe_misc::TaxTransactionId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_misc::TaxTransactionLineItem>> {
        stripe::ListPaginator::from_list_params(
            &format!("/tax/transactions/{transaction}/line_items"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFromCalculationTaxTransaction<'a> {
    /// Tax Calculation ID to be used as input when creating the transaction.
    pub calculation: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A custom order or sale identifier, such as 'myOrder_123'.
    /// Must be unique across all transactions, including reversals.
    pub reference: &'a str,
}
impl<'a> CreateFromCalculationTaxTransaction<'a> {
    pub fn new(calculation: &'a str, reference: &'a str) -> Self {
        Self { calculation, expand: None, metadata: None, reference }
    }
}
impl<'a> CreateFromCalculationTaxTransaction<'a> {
    /// Creates a Tax `Transaction` from a calculation.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_misc::TaxTransaction> {
        client.send_form(
            "/tax/transactions/create_from_calculation",
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A flat amount to reverse across the entire transaction, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    /// This value represents the total amount to refund from the transaction, including taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// The line item amounts to reverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [CreateReversalTaxTransactionLineItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// If `partial`, the provided line item or shipping cost amounts are reversed.
    /// If `full`, the original transaction is fully reversed.
    pub mode: CreateReversalTaxTransactionMode,
    /// The ID of the Transaction to partially or fully reverse.
    pub original_transaction: &'a str,
    /// A custom identifier for this reversal, such as `myOrder_123-refund_1`, which must be unique across all transactions.
    /// The reference helps identify this reversal transaction in exported [tax reports](https://stripe.com/docs/tax/reports).
    pub reference: &'a str,
    /// The shipping cost to reverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateReversalTaxTransactionShippingCost>,
}
impl<'a> CreateReversalTaxTransaction<'a> {
    pub fn new(
        mode: CreateReversalTaxTransactionMode,
        original_transaction: &'a str,
        reference: &'a str,
    ) -> Self {
        Self {
            expand: None,
            flat_amount: None,
            line_items: None,
            metadata: None,
            mode,
            original_transaction,
            reference,
            shipping_cost: None,
        }
    }
}
/// The line item amounts to reverse.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxTransactionLineItems<'a> {
    /// The amount to reverse, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    pub amount: i64,
    /// The amount of tax to reverse, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    pub amount_tax: i64,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: &'a str,
    /// The quantity reversed.
    /// Appears in [tax exports](https://stripe.com/docs/tax/reports), but does not affect the amount of tax reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A custom identifier for this line item in the reversal transaction, such as 'L1-refund'.
    pub reference: &'a str,
}
impl<'a> CreateReversalTaxTransactionLineItems<'a> {
    pub fn new(
        amount: i64,
        amount_tax: i64,
        original_line_item: &'a str,
        reference: &'a str,
    ) -> Self {
        Self { amount, amount_tax, metadata: None, original_line_item, quantity: None, reference }
    }
}
/// If `partial`, the provided line item or shipping cost amounts are reversed.
/// If `full`, the original transaction is fully reversed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateReversalTaxTransactionMode {
    Full,
    Partial,
}
impl CreateReversalTaxTransactionMode {
    pub fn as_str(self) -> &'static str {
        use CreateReversalTaxTransactionMode::*;
        match self {
            Full => "full",
            Partial => "partial",
        }
    }
}

impl std::str::FromStr for CreateReversalTaxTransactionMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateReversalTaxTransactionMode::*;
        match s {
            "full" => Ok(Full),
            "partial" => Ok(Partial),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateReversalTaxTransactionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateReversalTaxTransactionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateReversalTaxTransactionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateReversalTaxTransactionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateReversalTaxTransactionMode")
        })
    }
}
/// The shipping cost to reverse.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxTransactionShippingCost {
    /// The amount to reverse, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    pub amount: i64,
    /// The amount of tax to reverse, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    pub amount_tax: i64,
}
impl CreateReversalTaxTransactionShippingCost {
    pub fn new(amount: i64, amount_tax: i64) -> Self {
        Self { amount, amount_tax }
    }
}
impl<'a> CreateReversalTaxTransaction<'a> {
    /// Partially or fully reverses a previously created `Transaction`.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_misc::TaxTransaction> {
        client.send_form("/tax/transactions/create_reversal", self, http_types::Method::Post)
    }
}
