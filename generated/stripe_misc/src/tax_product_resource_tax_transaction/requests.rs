#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveTaxProductResourceTaxTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveTaxProductResourceTaxTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveTaxProductResourceTaxTransaction<'a> {
    /// Retrieves a Tax `Transaction` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        transaction: &stripe_misc::tax_product_resource_tax_transaction::TaxTransactionId,
    ) -> stripe::Response<stripe_misc::TaxProductResourceTaxTransaction> {
        client
            .get_query(&format!("/tax/transactions/{transaction}", transaction = transaction), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxProductResourceTaxTransaction<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The line item amounts to reverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [CreateReversalTaxProductResourceTaxTransactionLineItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// If `partial`, the provided line item or shipping cost amounts are reversed.
    ///
    /// If `full`, the original transaction is fully reversed.
    pub mode: CreateReversalTaxProductResourceTaxTransactionMode,
    /// The ID of the Transaction to partially or fully reverse.
    pub original_transaction: &'a str,
    /// A custom identifier for this reversal, such as `myOrder_123-refund_1`, which must be unique across all transactions.
    ///
    /// The reference helps identify this reversal transaction in exported [tax reports](https://stripe.com/docs/tax/reports).
    pub reference: &'a str,
    /// The shipping cost to reverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateReversalTaxProductResourceTaxTransactionShippingCost>,
}
impl<'a> CreateReversalTaxProductResourceTaxTransaction<'a> {
    pub fn new(
        mode: CreateReversalTaxProductResourceTaxTransactionMode,
        original_transaction: &'a str,
        reference: &'a str,
    ) -> Self {
        Self {
            expand: Default::default(),
            line_items: Default::default(),
            metadata: Default::default(),
            mode,
            original_transaction,
            reference,
            shipping_cost: Default::default(),
        }
    }
}
/// The line item amounts to reverse.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxProductResourceTaxTransactionLineItems<'a> {
    /// The amount to reverse, in negative integer cents.
    pub amount: i64,
    /// The amount of tax to reverse, in negative integer cents.
    pub amount_tax: i64,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: &'a str,
    /// The quantity reversed.
    ///
    /// Appears in [tax exports](https://stripe.com/docs/tax/reports), but does not affect the amount of tax reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A custom identifier for this line item in the reversal transaction, such as 'L1-refund'.
    pub reference: &'a str,
}
impl<'a> CreateReversalTaxProductResourceTaxTransactionLineItems<'a> {
    pub fn new(
        amount: i64,
        amount_tax: i64,
        original_line_item: &'a str,
        reference: &'a str,
    ) -> Self {
        Self {
            amount,
            amount_tax,
            metadata: Default::default(),
            original_line_item,
            quantity: Default::default(),
            reference,
        }
    }
}
/// If `partial`, the provided line item or shipping cost amounts are reversed.
///
/// If `full`, the original transaction is fully reversed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateReversalTaxProductResourceTaxTransactionMode {
    Full,
    Partial,
}

impl CreateReversalTaxProductResourceTaxTransactionMode {
    pub fn as_str(self) -> &'static str {
        use CreateReversalTaxProductResourceTaxTransactionMode::*;
        match self {
            Full => "full",
            Partial => "partial",
        }
    }
}

impl std::str::FromStr for CreateReversalTaxProductResourceTaxTransactionMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateReversalTaxProductResourceTaxTransactionMode::*;
        match s {
            "full" => Ok(Full),
            "partial" => Ok(Partial),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateReversalTaxProductResourceTaxTransactionMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateReversalTaxProductResourceTaxTransactionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateReversalTaxProductResourceTaxTransactionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateReversalTaxProductResourceTaxTransactionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The shipping cost to reverse.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxProductResourceTaxTransactionShippingCost {
    /// The amount to reverse, in negative integer cents.
    pub amount: i64,
    /// The amount of tax to reverse, in negative integer cents.
    pub amount_tax: i64,
}
impl CreateReversalTaxProductResourceTaxTransactionShippingCost {
    pub fn new(amount: i64, amount_tax: i64) -> Self {
        Self { amount, amount_tax }
    }
}
impl<'a> CreateReversalTaxProductResourceTaxTransaction<'a> {
    /// Partially or fully reverses a previously created `Transaction`.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_misc::TaxProductResourceTaxTransaction> {
        client.send_form("/tax/transactions/create_reversal", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFromCalculationTaxProductResourceTaxTransaction<'a> {
    /// Tax Calculation ID to be used as input when creating the transaction.
    pub calculation: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// A custom order or sale identifier, such as 'myOrder_123'.
    ///
    /// Must be unique across all transactions, including reversals.
    pub reference: &'a str,
}
impl<'a> CreateFromCalculationTaxProductResourceTaxTransaction<'a> {
    pub fn new(calculation: &'a str, reference: &'a str) -> Self {
        Self { calculation, expand: Default::default(), metadata: Default::default(), reference }
    }
}
impl<'a> CreateFromCalculationTaxProductResourceTaxTransaction<'a> {
    /// Creates a Tax `Transaction` from a calculation.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_misc::TaxProductResourceTaxTransaction> {
        client.send_form(
            "/tax/transactions/create_from_calculation",
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsTaxProductResourceTaxTransaction<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListLineItemsTaxProductResourceTaxTransaction<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> stripe::PaginationParams for ListLineItemsTaxProductResourceTaxTransaction<'a> {}
impl<'a> ListLineItemsTaxProductResourceTaxTransaction<'a> {
    /// Retrieves the line items of a committed standalone transaction as a collection.
    pub fn send(
        &self,
        client: &stripe::Client,
        transaction: &stripe_misc::tax_product_resource_tax_transaction::TaxTransactionId,
    ) -> stripe::Response<stripe_types::List<stripe_misc::TaxProductResourceTaxTransactionLineItem>>
    {
        client.get_query(
            &format!("/tax/transactions/{transaction}/line_items", transaction = transaction),
            self,
        )
    }
    pub fn paginate(
        self,
        transaction: &stripe_misc::tax_product_resource_tax_transaction::TaxTransactionId,
    ) -> stripe::ListPaginator<stripe_misc::TaxProductResourceTaxTransactionLineItem> {
        stripe::ListPaginator::from_params(
            &format!("/tax/transactions/{transaction}/line_items", transaction = transaction),
            self,
        )
    }
}
