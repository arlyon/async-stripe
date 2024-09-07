use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveTaxTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTaxTransactionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a Tax `Transaction` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveTaxTransaction {
    inner: RetrieveTaxTransactionBuilder,
    transaction: stripe_misc::TaxTransactionId,
}
impl RetrieveTaxTransaction {
    /// Construct a new `RetrieveTaxTransaction`.
    pub fn new(transaction: impl Into<stripe_misc::TaxTransactionId>) -> Self {
        Self { transaction: transaction.into(), inner: RetrieveTaxTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTaxTransaction {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveTaxTransaction {
    type Output = stripe_misc::TaxTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(StripeMethod::Get, format!("/tax/transactions/{transaction}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListLineItemsTaxTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListLineItemsTaxTransactionBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Retrieves the line items of a committed standalone transaction as a collection.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListLineItemsTaxTransaction {
    inner: ListLineItemsTaxTransactionBuilder,
    transaction: stripe_misc::TaxTransactionId,
}
impl ListLineItemsTaxTransaction {
    /// Construct a new `ListLineItemsTaxTransaction`.
    pub fn new(transaction: impl Into<stripe_misc::TaxTransactionId>) -> Self {
        Self { transaction: transaction.into(), inner: ListLineItemsTaxTransactionBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl ListLineItemsTaxTransaction {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::TaxTransactionLineItem>>
    {
        let transaction = &self.transaction;

        stripe_client_core::ListPaginator::new_list(
            format!("/tax/transactions/{transaction}/line_items"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListLineItemsTaxTransaction {
    type Output = stripe_types::List<stripe_misc::TaxTransactionLineItem>;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/tax/transactions/{transaction}/line_items"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateFromCalculationTaxTransactionBuilder {
    calculation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    reference: String,
}
impl CreateFromCalculationTaxTransactionBuilder {
    fn new(calculation: impl Into<String>, reference: impl Into<String>) -> Self {
        Self {
            calculation: calculation.into(),
            expand: None,
            metadata: None,
            reference: reference.into(),
        }
    }
}
/// Creates a Tax `Transaction` from a calculation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateFromCalculationTaxTransaction {
    inner: CreateFromCalculationTaxTransactionBuilder,
}
impl CreateFromCalculationTaxTransaction {
    /// Construct a new `CreateFromCalculationTaxTransaction`.
    pub fn new(calculation: impl Into<String>, reference: impl Into<String>) -> Self {
        Self {
            inner: CreateFromCalculationTaxTransactionBuilder::new(
                calculation.into(),
                reference.into(),
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl CreateFromCalculationTaxTransaction {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateFromCalculationTaxTransaction {
    type Output = stripe_misc::TaxTransaction;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax/transactions/create_from_calculation")
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateReversalTaxTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<Vec<CreateReversalTaxTransactionLineItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    mode: CreateReversalTaxTransactionMode,
    original_transaction: String,
    reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_cost: Option<CreateReversalTaxTransactionShippingCost>,
}
impl CreateReversalTaxTransactionBuilder {
    fn new(
        mode: impl Into<CreateReversalTaxTransactionMode>,
        original_transaction: impl Into<String>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            expand: None,
            flat_amount: None,
            line_items: None,
            metadata: None,
            mode: mode.into(),
            original_transaction: original_transaction.into(),
            reference: reference.into(),
            shipping_cost: None,
        }
    }
}
/// The line item amounts to reverse.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxTransactionLineItems {
    /// The amount to reverse, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    pub amount: i64,
    /// The amount of tax to reverse, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    pub amount_tax: i64,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: String,
    /// The quantity reversed.
    /// Appears in [tax exports](https://stripe.com/docs/tax/reports), but does not affect the amount of tax reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A custom identifier for this line item in the reversal transaction, such as 'L1-refund'.
    pub reference: String,
}
impl CreateReversalTaxTransactionLineItems {
    pub fn new(
        amount: impl Into<i64>,
        amount_tax: impl Into<i64>,
        original_line_item: impl Into<String>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            amount_tax: amount_tax.into(),
            metadata: None,
            original_line_item: original_line_item.into(),
            quantity: None,
            reference: reference.into(),
        }
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
    pub fn new(amount: impl Into<i64>, amount_tax: impl Into<i64>) -> Self {
        Self { amount: amount.into(), amount_tax: amount_tax.into() }
    }
}
/// Partially or fully reverses a previously created `Transaction`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateReversalTaxTransaction {
    inner: CreateReversalTaxTransactionBuilder,
}
impl CreateReversalTaxTransaction {
    /// Construct a new `CreateReversalTaxTransaction`.
    pub fn new(
        mode: impl Into<CreateReversalTaxTransactionMode>,
        original_transaction: impl Into<String>,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateReversalTaxTransactionBuilder::new(
                mode.into(),
                original_transaction.into(),
                reference.into(),
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A flat amount to reverse across the entire transaction, in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) in negative.
    /// This value represents the total amount to refund from the transaction, including taxes.
    pub fn flat_amount(mut self, flat_amount: impl Into<i64>) -> Self {
        self.inner.flat_amount = Some(flat_amount.into());
        self
    }
    /// The line item amounts to reverse.
    pub fn line_items(
        mut self,
        line_items: impl Into<Vec<CreateReversalTaxTransactionLineItems>>,
    ) -> Self {
        self.inner.line_items = Some(line_items.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The shipping cost to reverse.
    pub fn shipping_cost(
        mut self,
        shipping_cost: impl Into<CreateReversalTaxTransactionShippingCost>,
    ) -> Self {
        self.inner.shipping_cost = Some(shipping_cost.into());
        self
    }
}
impl CreateReversalTaxTransaction {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateReversalTaxTransaction {
    type Output = stripe_misc::TaxTransaction;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax/transactions/create_reversal")
            .form(&self.inner)
    }
}
