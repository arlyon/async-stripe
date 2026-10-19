use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cardholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::IssuingTransactionType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingTransactionBuilder").finish_non_exhaustive()
    }
}
impl ListIssuingTransactionBuilder {
    fn new() -> Self {
        Self {
            card: None,
            cardholder: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
        }
    }
}
/// Returns a list of Issuing `Transaction` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListIssuingTransaction {
    inner: ListIssuingTransactionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingTransaction").finish_non_exhaustive()
    }
}
impl ListIssuingTransaction {
    /// Construct a new `ListIssuingTransaction`.
    pub fn new() -> Self {
        Self { inner: ListIssuingTransactionBuilder::new() }
    }
    /// Only return transactions that belong to the given card.
    pub fn card(mut self, card: impl Into<String>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Only return transactions that belong to the given cardholder.
    pub fn cardholder(mut self, cardholder: impl Into<String>) -> Self {
        self.inner.cardholder = Some(cardholder.into());
        self
    }
    /// Only return transactions that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
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
    /// Only return transactions that have the given type. One of `capture` or `refund`.
    pub fn type_(mut self, type_: impl Into<stripe_shared::IssuingTransactionType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl Default for ListIssuingTransaction {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingTransaction {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingTransaction>>
    {
        stripe_client_core::ListPaginator::new_list("/issuing/transactions", &self.inner)
    }
}

impl StripeRequest for ListIssuingTransaction {
    type Output = stripe_types::List<stripe_shared::IssuingTransaction>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/transactions").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingTransactionBuilder").finish_non_exhaustive()
    }
}
impl RetrieveIssuingTransactionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Transaction` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveIssuingTransaction {
    inner: RetrieveIssuingTransactionBuilder,
    transaction: stripe_shared::IssuingTransactionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingTransaction").finish_non_exhaustive()
    }
}
impl RetrieveIssuingTransaction {
    /// Construct a new `RetrieveIssuingTransaction`.
    pub fn new(transaction: impl Into<stripe_shared::IssuingTransactionId>) -> Self {
        Self { transaction: transaction.into(), inner: RetrieveIssuingTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingTransaction {
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

impl StripeRequest for RetrieveIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/transactions/{transaction}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingTransactionBuilder").finish_non_exhaustive()
    }
}
impl UpdateIssuingTransactionBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the specified Issuing `Transaction` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingTransaction {
    inner: UpdateIssuingTransactionBuilder,
    transaction: stripe_shared::IssuingTransactionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingTransaction").finish_non_exhaustive()
    }
}
impl UpdateIssuingTransaction {
    /// Construct a new `UpdateIssuingTransaction`.
    pub fn new(transaction: impl Into<stripe_shared::IssuingTransactionId>) -> Self {
        Self { transaction: transaction.into(), inner: UpdateIssuingTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
impl UpdateIssuingTransaction {
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

impl StripeRequest for UpdateIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/transactions/{transaction}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RefundIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundIssuingTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundIssuingTransactionBuilder").finish_non_exhaustive()
    }
}
impl RefundIssuingTransactionBuilder {
    fn new() -> Self {
        Self { expand: None, refund_amount: None }
    }
}
/// Refund a test-mode Transaction.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RefundIssuingTransaction {
    inner: RefundIssuingTransactionBuilder,
    transaction: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundIssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundIssuingTransaction").finish_non_exhaustive()
    }
}
impl RefundIssuingTransaction {
    /// Construct a new `RefundIssuingTransaction`.
    pub fn new(transaction: impl Into<String>) -> Self {
        Self { transaction: transaction.into(), inner: RefundIssuingTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The total amount to attempt to refund.
    /// This amount is in the provided currency, or defaults to the cards currency, and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
}
impl RefundIssuingTransaction {
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

impl StripeRequest for RefundIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/transactions/{transaction}/refund"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateForceCaptureIssuingTransactionBuilder {
    amount: i64,
    card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_data: Option<MerchantDataSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purchase_details: Option<PurchaseDetailsSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateForceCaptureIssuingTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateForceCaptureIssuingTransactionBuilder").finish_non_exhaustive()
    }
}
impl CreateForceCaptureIssuingTransactionBuilder {
    fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self {
            amount: amount.into(),
            card: card.into(),
            currency: None,
            expand: None,
            merchant_data: None,
            purchase_details: None,
        }
    }
}
/// Allows the user to capture an arbitrary amount, also known as a forced capture.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateForceCaptureIssuingTransaction {
    inner: CreateForceCaptureIssuingTransactionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateForceCaptureIssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateForceCaptureIssuingTransaction").finish_non_exhaustive()
    }
}
impl CreateForceCaptureIssuingTransaction {
    /// Construct a new `CreateForceCaptureIssuingTransaction`.
    pub fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self { inner: CreateForceCaptureIssuingTransactionBuilder::new(amount.into(), card.into()) }
    }
    /// The currency of the capture.
    /// If not provided, defaults to the currency of the card.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
    pub fn merchant_data(mut self, merchant_data: impl Into<MerchantDataSpecs>) -> Self {
        self.inner.merchant_data = Some(merchant_data.into());
        self
    }
    /// Additional purchase information that is optionally provided by the merchant.
    pub fn purchase_details(mut self, purchase_details: impl Into<PurchaseDetailsSpecs>) -> Self {
        self.inner.purchase_details = Some(purchase_details.into());
        self
    }
}
impl CreateForceCaptureIssuingTransaction {
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

impl StripeRequest for CreateForceCaptureIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(
            StripeMethod::Post,
            "/test_helpers/issuing/transactions/create_force_capture",
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateUnlinkedRefundIssuingTransactionBuilder {
    amount: i64,
    card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_data: Option<MerchantDataSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purchase_details: Option<PurchaseDetailsSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateUnlinkedRefundIssuingTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateUnlinkedRefundIssuingTransactionBuilder").finish_non_exhaustive()
    }
}
impl CreateUnlinkedRefundIssuingTransactionBuilder {
    fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self {
            amount: amount.into(),
            card: card.into(),
            currency: None,
            expand: None,
            merchant_data: None,
            purchase_details: None,
        }
    }
}
/// Allows the user to refund an arbitrary amount, also known as a unlinked refund.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateUnlinkedRefundIssuingTransaction {
    inner: CreateUnlinkedRefundIssuingTransactionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateUnlinkedRefundIssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateUnlinkedRefundIssuingTransaction").finish_non_exhaustive()
    }
}
impl CreateUnlinkedRefundIssuingTransaction {
    /// Construct a new `CreateUnlinkedRefundIssuingTransaction`.
    pub fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self {
            inner: CreateUnlinkedRefundIssuingTransactionBuilder::new(amount.into(), card.into()),
        }
    }
    /// The currency of the unlinked refund.
    /// If not provided, defaults to the currency of the card.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
    pub fn merchant_data(mut self, merchant_data: impl Into<MerchantDataSpecs>) -> Self {
        self.inner.merchant_data = Some(merchant_data.into());
        self
    }
    /// Additional purchase information that is optionally provided by the merchant.
    pub fn purchase_details(mut self, purchase_details: impl Into<PurchaseDetailsSpecs>) -> Self {
        self.inner.purchase_details = Some(purchase_details.into());
        self
    }
}
impl CreateUnlinkedRefundIssuingTransaction {
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

impl StripeRequest for CreateUnlinkedRefundIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(
            StripeMethod::Post,
            "/test_helpers/issuing/transactions/create_unlinked_refund",
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct MerchantDataSpecs {
    /// A categorization of the seller's type of business.
    /// See our [merchant categories guide](https://docs.stripe.com/issuing/merchant-categories) for a list of possible values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<stripe_shared::CreateIssuingAuthorizationMerchantDataCategory>,
    /// City where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Name of the seller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier assigned to the seller by the card network.
    /// Different card networks may assign different network_id fields to the same merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// Postal code where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// An ID assigned by the seller to the location of the sale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    /// URL provided by the merchant on a 3DS request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MerchantDataSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MerchantDataSpecs").finish_non_exhaustive()
    }
}
impl MerchantDataSpecs {
    pub fn new() -> Self {
        Self {
            category: None,
            city: None,
            country: None,
            name: None,
            network_id: None,
            postal_code: None,
            state: None,
            terminal_id: None,
            url: None,
        }
    }
}
impl Default for MerchantDataSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetCardholderPromptDataSpecs {
    /// Driver ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<String>,
    /// Odometer reading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odometer: Option<i64>,
    /// An alphanumeric ID.
    /// This field is used when a vehicle ID, driver ID, or generic ID is entered by the cardholder, but the merchant or card network did not specify the prompt type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unspecified_id: Option<String>,
    /// User ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Vehicle number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetCardholderPromptDataSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetCardholderPromptDataSpecs").finish_non_exhaustive()
    }
}
impl FleetCardholderPromptDataSpecs {
    pub fn new() -> Self {
        Self {
            driver_id: None,
            odometer: None,
            unspecified_id: None,
            user_id: None,
            vehicle_number: None,
        }
    }
}
impl Default for FleetCardholderPromptDataSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownFuelSpecs {
    /// Gross fuel amount that should equal Fuel Volume multiplied by Fuel Unit Cost, inclusive of taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownFuelSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownFuelSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownFuelSpecs {
    pub fn new() -> Self {
        Self { gross_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownFuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownNonFuelSpecs {
    /// Gross non-fuel amount that should equal the sum of the line items, inclusive of taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownNonFuelSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownNonFuelSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownNonFuelSpecs {
    pub fn new() -> Self {
        Self { gross_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownNonFuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownTaxSpecs {
    /// Amount of state or provincial Sales Tax included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amount_decimal: Option<String>,
    /// Amount of national Sales Tax or VAT included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_amount_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownTaxSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownTaxSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownTaxSpecs {
    pub fn new() -> Self {
        Self { local_amount_decimal: None, national_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownTaxSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FlightSegmentSpecs {
    /// The three-letter IATA airport code of the flight's destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_airport_code: Option<String>,
    /// The airline carrier code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// The three-letter IATA airport code that the flight departed from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,
    /// The flight number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<String>,
    /// The flight's service class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_class: Option<String>,
    /// Whether a stopover is allowed on this flight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopover_allowed: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FlightSegmentSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FlightSegmentSpecs").finish_non_exhaustive()
    }
}
impl FlightSegmentSpecs {
    pub fn new() -> Self {
        Self {
            arrival_airport_code: None,
            carrier: None,
            departure_airport_code: None,
            flight_number: None,
            service_class: None,
            stopover_allowed: None,
        }
    }
}
impl Default for FlightSegmentSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FuelSpecs {
    /// [Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying the primary fuel product purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_product_code: Option<String>,
    /// The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_decimal: Option<String>,
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<stripe_shared::IssuingAuthorizationFuelDataType>,
    /// The units for `quantity_decimal`.
    /// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<stripe_shared::IssuingAuthorizationFuelDataUnit>,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost_decimal: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FuelSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FuelSpecs").finish_non_exhaustive()
    }
}
impl FuelSpecs {
    pub fn new() -> Self {
        Self {
            industry_product_code: None,
            quantity_decimal: None,
            type_: None,
            unit: None,
            unit_cost_decimal: None,
        }
    }
}
impl Default for FuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct LodgingSpecs {
    /// The time of checking into the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_at: Option<stripe_types::Timestamp>,
    /// The number of nights stayed at the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nights: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LodgingSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LodgingSpecs").finish_non_exhaustive()
    }
}
impl LodgingSpecs {
    pub fn new() -> Self {
        Self { check_in_at: None, nights: None }
    }
}
impl Default for LodgingSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ReceiptSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ReceiptSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ReceiptSpecs").finish_non_exhaustive()
    }
}
impl ReceiptSpecs {
    pub fn new() -> Self {
        Self { description: None, quantity: None, total: None, unit_cost: None }
    }
}
impl Default for ReceiptSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetReportedBreakdownSpecs {
    /// Breakdown of fuel portion of the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<FleetReportedBreakdownFuelSpecs>,
    /// Breakdown of non-fuel portion of the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_fuel: Option<FleetReportedBreakdownNonFuelSpecs>,
    /// Information about tax included in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<FleetReportedBreakdownTaxSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetReportedBreakdownSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetReportedBreakdownSpecs").finish_non_exhaustive()
    }
}
impl FleetReportedBreakdownSpecs {
    pub fn new() -> Self {
        Self { fuel: None, non_fuel: None, tax: None }
    }
}
impl Default for FleetReportedBreakdownSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FlightSpecs {
    /// The time that the flight departed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<stripe_types::Timestamp>,
    /// The name of the passenger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<String>,
    /// Whether the ticket is refundable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable: Option<bool>,
    /// The legs of the trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<FlightSegmentSpecs>>,
    /// The travel agency that issued the ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_agency: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FlightSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FlightSpecs").finish_non_exhaustive()
    }
}
impl FlightSpecs {
    pub fn new() -> Self {
        Self {
            departure_at: None,
            passenger_name: None,
            refundable: None,
            segments: None,
            travel_agency: None,
        }
    }
}
impl Default for FlightSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct FleetSpecs {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<FleetCardholderPromptDataSpecs>,
    /// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<stripe_shared::PurchaseType>,
    /// More information about the total amount.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<FleetReportedBreakdownSpecs>,
    /// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<stripe_shared::ServiceType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FleetSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FleetSpecs").finish_non_exhaustive()
    }
}
impl FleetSpecs {
    pub fn new() -> Self {
        Self {
            cardholder_prompt_data: None,
            purchase_type: None,
            reported_breakdown: None,
            service_type: None,
        }
    }
}
impl Default for FleetSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PurchaseDetailsSpecs {
    /// Fleet-specific information for transactions using Fleet cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<FleetSpecs>,
    /// Information about the flight that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<FlightSpecs>,
    /// Information about fuel that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<FuelSpecs>,
    /// Information about lodging that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lodging: Option<LodgingSpecs>,
    /// The line items in the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<ReceiptSpecs>>,
    /// A merchant-specific order number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PurchaseDetailsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PurchaseDetailsSpecs").finish_non_exhaustive()
    }
}
impl PurchaseDetailsSpecs {
    pub fn new() -> Self {
        Self {
            fleet: None,
            flight: None,
            fuel: None,
            lodging: None,
            receipt: None,
            reference: None,
        }
    }
}
impl Default for PurchaseDetailsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
/// Only return transactions that have the given type. One of `capture` or `refund`.
pub type ListIssuingTransactionType = stripe_shared::IssuingTransactionType;
/// A categorization of the seller's type of business.
/// See our [merchant categories guide](https://docs.stripe.com/issuing/merchant-categories) for a list of possible values.
pub type CreateForceCaptureIssuingTransactionMerchantDataCategory =
    stripe_shared::CreateIssuingAuthorizationMerchantDataCategory;
/// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType =
    stripe_shared::PurchaseType;
/// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType =
    stripe_shared::ServiceType;
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType =
    stripe_shared::IssuingAuthorizationFuelDataType;
/// The units for `quantity_decimal`.
/// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit =
    stripe_shared::IssuingAuthorizationFuelDataUnit;
/// A categorization of the seller's type of business.
/// See our [merchant categories guide](https://docs.stripe.com/issuing/merchant-categories) for a list of possible values.
pub type CreateUnlinkedRefundIssuingTransactionMerchantDataCategory =
    stripe_shared::CreateIssuingAuthorizationMerchantDataCategory;
/// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType =
    stripe_shared::PurchaseType;
/// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType =
    stripe_shared::ServiceType;
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType =
    stripe_shared::IssuingAuthorizationFuelDataType;
/// The units for `quantity_decimal`.
/// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit =
    stripe_shared::IssuingAuthorizationFuelDataUnit;
/// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
pub type CreateForceCaptureIssuingTransactionMerchantData = MerchantDataSpecs;
/// Answers to prompts presented to the cardholder at the point of sale.
/// Prompted fields vary depending on the configuration of your physical fleet cards.
/// Typical points of sale support only numeric entry.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleetCardholderPromptData =
    FleetCardholderPromptDataSpecs;
/// Breakdown of fuel portion of the purchase.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleetReportedBreakdownFuel =
    FleetReportedBreakdownFuelSpecs;
/// Breakdown of non-fuel portion of the purchase.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleetReportedBreakdownNonFuel =
    FleetReportedBreakdownNonFuelSpecs;
/// Information about tax included in this transaction.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleetReportedBreakdownTax =
    FleetReportedBreakdownTaxSpecs;
/// The legs of the trip.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFlightSegments = FlightSegmentSpecs;
/// Information about fuel that was purchased with this transaction.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFuel = FuelSpecs;
/// Information about lodging that was purchased with this transaction.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsLodging = LodgingSpecs;
/// The line items in the purchase.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsReceipt = ReceiptSpecs;
/// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
pub type CreateUnlinkedRefundIssuingTransactionMerchantData = MerchantDataSpecs;
/// Answers to prompts presented to the cardholder at the point of sale.
/// Prompted fields vary depending on the configuration of your physical fleet cards.
/// Typical points of sale support only numeric entry.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetCardholderPromptData =
    FleetCardholderPromptDataSpecs;
/// Breakdown of fuel portion of the purchase.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetReportedBreakdownFuel =
    FleetReportedBreakdownFuelSpecs;
/// Breakdown of non-fuel portion of the purchase.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetReportedBreakdownNonFuel =
    FleetReportedBreakdownNonFuelSpecs;
/// Information about tax included in this transaction.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetReportedBreakdownTax =
    FleetReportedBreakdownTaxSpecs;
/// The legs of the trip.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFlightSegments = FlightSegmentSpecs;
/// Information about fuel that was purchased with this transaction.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuel = FuelSpecs;
/// Information about lodging that was purchased with this transaction.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsLodging = LodgingSpecs;
/// The line items in the purchase.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsReceipt = ReceiptSpecs;
/// More information about the total amount.
/// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleetReportedBreakdown =
    FleetReportedBreakdownSpecs;
/// Information about the flight that was purchased with this transaction.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFlight = FlightSpecs;
/// More information about the total amount.
/// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetReportedBreakdown =
    FleetReportedBreakdownSpecs;
/// Information about the flight that was purchased with this transaction.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFlight = FlightSpecs;
/// Fleet-specific information for transactions using Fleet cards.
pub type CreateForceCaptureIssuingTransactionPurchaseDetailsFleet = FleetSpecs;
/// Fleet-specific information for transactions using Fleet cards.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleet = FleetSpecs;
/// Additional purchase information that is optionally provided by the merchant.
pub type CreateForceCaptureIssuingTransactionPurchaseDetails = PurchaseDetailsSpecs;
/// Additional purchase information that is optionally provided by the merchant.
pub type CreateUnlinkedRefundIssuingTransactionPurchaseDetails = PurchaseDetailsSpecs;
