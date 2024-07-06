use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListChargeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<&'a str>,
}
impl<'a> ListChargeBuilder<'a> {
    fn new() -> Self {
        Self {
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
            transfer_group: None,
        }
    }
}
/// Returns a list of charges you’ve previously created.
/// The charges are returned in sorted order, with the most recent charges appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListCharge<'a> {
    inner: ListChargeBuilder<'a>,
}
impl<'a> ListCharge<'a> {
    /// Construct a new `ListCharge`.
    pub fn new() -> Self {
        Self { inner: ListChargeBuilder::new() }
    }
    /// Only return charges that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// Only return charges for the customer specified by this customer ID.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Only return charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    pub fn payment_intent(mut self, payment_intent: &'a str) -> Self {
        self.inner.payment_intent = Some(payment_intent);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Only return charges for this transfer group.
    pub fn transfer_group(mut self, transfer_group: &'a str) -> Self {
        self.inner.transfer_group = Some(transfer_group);
        self
    }
}
impl<'a> Default for ListCharge<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCharge<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Charge>> {
        stripe_client_core::ListPaginator::new_list("/charges", self.inner)
    }
}

impl StripeRequest for ListCharge<'_> {
    type Output = stripe_types::List<stripe_shared::Charge>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/charges").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveChargeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveChargeBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a charge that has previously been created.
/// Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information.
/// The same information is returned when creating or refunding the charge.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCharge<'a> {
    inner: RetrieveChargeBuilder<'a>,
    charge: &'a stripe_shared::ChargeId,
}
impl<'a> RetrieveCharge<'a> {
    /// Construct a new `RetrieveCharge`.
    pub fn new(charge: &'a stripe_shared::ChargeId) -> Self {
        Self { charge, inner: RetrieveChargeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveCharge<'_> {
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

impl StripeRequest for RetrieveCharge<'_> {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        let charge = self.charge;
        RequestBuilder::new(StripeMethod::Get, format!("/charges/{charge}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct SearchChargeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<&'a str>,
    query: &'a str,
}
impl<'a> SearchChargeBuilder<'a> {
    fn new(query: &'a str) -> Self {
        Self { expand: None, limit: None, page: None, query }
    }
}
/// Search for charges you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SearchCharge<'a> {
    inner: SearchChargeBuilder<'a>,
}
impl<'a> SearchCharge<'a> {
    /// Construct a new `SearchCharge`.
    pub fn new(query: &'a str) -> Self {
        Self { inner: SearchChargeBuilder::new(query) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    pub fn page(mut self, page: &'a str) -> Self {
        self.inner.page = Some(page);
        self
    }
}
impl SearchCharge<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::SearchList<stripe_shared::Charge>> {
        stripe_client_core::ListPaginator::new_search_list("/charges/search", self.inner)
    }
}

impl StripeRequest for SearchCharge<'_> {
    type Output = stripe_types::SearchList<stripe_shared::Charge>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/charges/search").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateChargeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<CreateChargeDestination<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radar_options: Option<CreateChargeRadarOptions<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<OptionalFieldsShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor_suffix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<CreateChargeTransferData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<&'a str>,
}
impl<'a> CreateChargeBuilder<'a> {
    fn new() -> Self {
        Self {
            amount: None,
            application_fee: None,
            application_fee_amount: None,
            capture: None,
            currency: None,
            customer: None,
            description: None,
            destination: None,
            expand: None,
            metadata: None,
            on_behalf_of: None,
            radar_options: None,
            receipt_email: None,
            shipping: None,
            source: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_data: None,
            transfer_group: None,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateChargeDestination<'a> {
    /// ID of an existing, connected Stripe account.
    pub account: &'a str,
    /// The amount to transfer to the destination account without creating an `Application Fee` object.
    /// Cannot be combined with the `application_fee` parameter.
    /// Must be less than or equal to the charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl<'a> CreateChargeDestination<'a> {
    pub fn new(account: &'a str) -> Self {
        Self { account, amount: None }
    }
}
/// Options to configure Radar.
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateChargeRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> CreateChargeRadarOptions<'a> {
    pub fn new() -> Self {
        Self { session: None }
    }
}
impl<'a> Default for CreateChargeRadarOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// An optional dictionary including the account to automatically transfer to as part of a destination charge.
/// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateChargeTransferData<'a> {
    /// The amount transferred to the destination account, if specified.
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateChargeTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: None, destination }
    }
}
/// This method is no longer recommended—use the [Payment Intents API](https://stripe.com/docs/api/payment_intents).
/// to initiate a new payment instead. Confirmation of the PaymentIntent creates the `Charge`
/// object used to request payment.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCharge<'a> {
    inner: CreateChargeBuilder<'a>,
}
impl<'a> CreateCharge<'a> {
    /// Construct a new `CreateCharge`.
    pub fn new() -> Self {
        Self { inner: CreateChargeBuilder::new() }
    }
    /// Amount intended to be collected by this payment.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    pub fn application_fee(mut self, application_fee: i64) -> Self {
        self.inner.application_fee = Some(application_fee);
        self
    }
    /// A fee in cents (or local equivalent) that will be applied to the charge and transferred to the application owner's Stripe account.
    /// The request must be made with an OAuth key or the `Stripe-Account` header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/direct-charges#collect-fees).
    pub fn application_fee_amount(mut self, application_fee_amount: i64) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount);
        self
    }
    /// Whether to immediately capture the charge.
    /// Defaults to `true`.
    /// When `false`, the charge issues an authorization (or pre-authorization), and will need to be [captured](https://stripe.com/docs/api#capture_charge) later.
    /// Uncaptured charges expire after a set number of days (7 by default).
    /// For more information, see the [authorizing charges and settling later](https://stripe.com/docs/charges/placing-a-hold) documentation.
    pub fn capture(mut self, capture: bool) -> Self {
        self.inner.capture = Some(capture);
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: stripe_types::Currency) -> Self {
        self.inner.currency = Some(currency);
        self
    }
    /// The ID of an existing customer that will be charged in this request.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// An arbitrary string which you can attach to a `Charge` object.
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    pub fn destination(mut self, destination: CreateChargeDestination<'a>) -> Self {
        self.inner.destination = Some(destination);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The Stripe account ID for which these funds are intended.
    /// Automatically set if you use the `destination` parameter.
    /// For details, see [Creating Separate Charges and Transfers](https://stripe.com/docs/connect/separate-charges-and-transfers#settlement-merchant).
    pub fn on_behalf_of(mut self, on_behalf_of: &'a str) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of);
        self
    }
    /// Options to configure Radar.
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    pub fn radar_options(mut self, radar_options: CreateChargeRadarOptions<'a>) -> Self {
        self.inner.radar_options = Some(radar_options);
        self
    }
    /// The email address to which this charge's [receipt](https://stripe.com/docs/dashboard/receipts) will be sent.
    /// The receipt will not be sent until the charge is paid, and no receipts will be sent for test mode charges.
    /// If this charge is for a [Customer](https://stripe.com/docs/api/customers/object), the email address specified here will override the customer's email address.
    /// If `receipt_email` is specified for a charge in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    pub fn receipt_email(mut self, receipt_email: &'a str) -> Self {
        self.inner.receipt_email = Some(receipt_email);
        self
    }
    /// Shipping information for the charge. Helps prevent fraud on charges for physical goods.
    pub fn shipping(mut self, shipping: OptionalFieldsShipping<'a>) -> Self {
        self.inner.shipping = Some(shipping);
        self
    }
    /// A payment source to be charged.
    /// This can be the ID of a [card](https://stripe.com/docs/api#cards) (i.e., credit or debit card), a [bank account](https://stripe.com/docs/api#bank_accounts), a [source](https://stripe.com/docs/api#sources), a [token](https://stripe.com/docs/api#tokens), or a [connected account](https://stripe.com/docs/connect/account-debits#charging-a-connected-account).
    /// For certain sources---namely, [cards](https://stripe.com/docs/api#cards), [bank accounts](https://stripe.com/docs/api#bank_accounts), and attached [sources](https://stripe.com/docs/api#sources)---you must also pass the ID of the associated customer.
    pub fn source(mut self, source: &'a str) -> Self {
        self.inner.source = Some(source);
        self
    }
    /// For card charges, use `statement_descriptor_suffix` instead.
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    pub fn statement_descriptor(mut self, statement_descriptor: &'a str) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor);
        self
    }
    /// Provides information about the charge that customers see on their statements.
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub fn statement_descriptor_suffix(mut self, statement_descriptor_suffix: &'a str) -> Self {
        self.inner.statement_descriptor_suffix = Some(statement_descriptor_suffix);
        self
    }
    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub fn transfer_data(mut self, transfer_data: CreateChargeTransferData<'a>) -> Self {
        self.inner.transfer_data = Some(transfer_data);
        self
    }
    /// A string that identifies this transaction as part of a group.
    /// For details, see [Grouping transactions](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options).
    pub fn transfer_group(mut self, transfer_group: &'a str) -> Self {
        self.inner.transfer_group = Some(transfer_group);
        self
    }
}
impl<'a> Default for CreateCharge<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateCharge<'_> {
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

impl StripeRequest for CreateCharge<'_> {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/charges").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateChargeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fraud_details: Option<UpdateChargeFraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<OptionalFieldsShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<&'a str>,
}
impl<'a> UpdateChargeBuilder<'a> {
    fn new() -> Self {
        Self {
            customer: None,
            description: None,
            expand: None,
            fraud_details: None,
            metadata: None,
            receipt_email: None,
            shipping: None,
            transfer_group: None,
        }
    }
}
/// A set of key-value pairs you can attach to a charge giving information about its riskiness.
/// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
/// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
/// Stripe will use the information you send to improve our fraud detection algorithms.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateChargeFraudDetails {
    /// Either `safe` or `fraudulent`.
    pub user_report: UpdateChargeFraudDetailsUserReport,
}
impl UpdateChargeFraudDetails {
    pub fn new(user_report: UpdateChargeFraudDetailsUserReport) -> Self {
        Self { user_report }
    }
}
/// Either `safe` or `fraudulent`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateChargeFraudDetailsUserReport {
    Fraudulent,
    Safe,
}
impl UpdateChargeFraudDetailsUserReport {
    pub fn as_str(self) -> &'static str {
        use UpdateChargeFraudDetailsUserReport::*;
        match self {
            Fraudulent => "fraudulent",
            Safe => "safe",
        }
    }
}

impl std::str::FromStr for UpdateChargeFraudDetailsUserReport {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateChargeFraudDetailsUserReport::*;
        match s {
            "fraudulent" => Ok(Fraudulent),
            "safe" => Ok(Safe),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateChargeFraudDetailsUserReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateChargeFraudDetailsUserReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateChargeFraudDetailsUserReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateChargeFraudDetailsUserReport {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateChargeFraudDetailsUserReport")
        })
    }
}
/// Updates the specified charge by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCharge<'a> {
    inner: UpdateChargeBuilder<'a>,
    charge: &'a stripe_shared::ChargeId,
}
impl<'a> UpdateCharge<'a> {
    /// Construct a new `UpdateCharge`.
    pub fn new(charge: &'a stripe_shared::ChargeId) -> Self {
        Self { charge, inner: UpdateChargeBuilder::new() }
    }
    /// The ID of an existing customer that will be associated with this request.
    /// This field may only be updated if there is no existing associated customer with this charge.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// An arbitrary string which you can attach to a charge object.
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A set of key-value pairs you can attach to a charge giving information about its riskiness.
    /// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
    /// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
    /// Stripe will use the information you send to improve our fraud detection algorithms.
    pub fn fraud_details(mut self, fraud_details: UpdateChargeFraudDetails) -> Self {
        self.inner.fraud_details = Some(fraud_details);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// This is the email address that the receipt for this charge will be sent to.
    /// If this field is updated, then a new email receipt will be sent to the updated address.
    pub fn receipt_email(mut self, receipt_email: &'a str) -> Self {
        self.inner.receipt_email = Some(receipt_email);
        self
    }
    /// Shipping information for the charge. Helps prevent fraud on charges for physical goods.
    pub fn shipping(mut self, shipping: OptionalFieldsShipping<'a>) -> Self {
        self.inner.shipping = Some(shipping);
        self
    }
    /// A string that identifies this transaction as part of a group.
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub fn transfer_group(mut self, transfer_group: &'a str) -> Self {
        self.inner.transfer_group = Some(transfer_group);
        self
    }
}
impl UpdateCharge<'_> {
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

impl StripeRequest for UpdateCharge<'_> {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        let charge = self.charge;
        RequestBuilder::new(StripeMethod::Post, format!("/charges/{charge}")).form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CaptureChargeBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor_suffix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<CaptureChargeTransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<&'a str>,
}
impl<'a> CaptureChargeBuilder<'a> {
    fn new() -> Self {
        Self {
            amount: None,
            application_fee: None,
            application_fee_amount: None,
            expand: None,
            receipt_email: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_data: None,
            transfer_group: None,
        }
    }
}
/// An optional dictionary including the account to automatically transfer to as part of a destination charge.
/// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CaptureChargeTransferData {
    /// The amount transferred to the destination account, if specified.
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl CaptureChargeTransferData {
    pub fn new() -> Self {
        Self { amount: None }
    }
}
impl Default for CaptureChargeTransferData {
    fn default() -> Self {
        Self::new()
    }
}
/// Capture the payment of an existing, uncaptured charge that was created with the `capture` option set to false.
///
/// Uncaptured payments expire a set number of days after they are created ([7 by default](https://stripe.com/docs/charges/placing-a-hold)), after which they are marked as refunded and capture attempts will fail.
///
/// Don’t use this method to capture a PaymentIntent-initiated charge.
/// Use [Capture a PaymentIntent](https://stripe.com/docs/api/payment_intents/capture).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CaptureCharge<'a> {
    inner: CaptureChargeBuilder<'a>,
    charge: &'a stripe_shared::ChargeId,
}
impl<'a> CaptureCharge<'a> {
    /// Construct a new `CaptureCharge`.
    pub fn new(charge: &'a stripe_shared::ChargeId) -> Self {
        Self { charge, inner: CaptureChargeBuilder::new() }
    }
    /// The amount to capture, which must be less than or equal to the original amount.
    /// Any additional amount will be automatically refunded.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// An application fee to add on to this charge.
    pub fn application_fee(mut self, application_fee: i64) -> Self {
        self.inner.application_fee = Some(application_fee);
        self
    }
    /// An application fee amount to add on to this charge, which must be less than or equal to the original amount.
    pub fn application_fee_amount(mut self, application_fee_amount: i64) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The email address to send this charge's receipt to.
    /// This will override the previously-specified email address for this charge, if one was set.
    /// Receipts will not be sent in test mode.
    pub fn receipt_email(mut self, receipt_email: &'a str) -> Self {
        self.inner.receipt_email = Some(receipt_email);
        self
    }
    /// For card charges, use `statement_descriptor_suffix` instead.
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    pub fn statement_descriptor(mut self, statement_descriptor: &'a str) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor);
        self
    }
    /// Provides information about the charge that customers see on their statements.
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub fn statement_descriptor_suffix(mut self, statement_descriptor_suffix: &'a str) -> Self {
        self.inner.statement_descriptor_suffix = Some(statement_descriptor_suffix);
        self
    }
    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub fn transfer_data(mut self, transfer_data: CaptureChargeTransferData) -> Self {
        self.inner.transfer_data = Some(transfer_data);
        self
    }
    /// A string that identifies this transaction as part of a group.
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub fn transfer_group(mut self, transfer_group: &'a str) -> Self {
        self.inner.transfer_group = Some(transfer_group);
        self
    }
}
impl CaptureCharge<'_> {
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

impl StripeRequest for CaptureCharge<'_> {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        let charge = self.charge;
        RequestBuilder::new(StripeMethod::Post, format!("/charges/{charge}/capture"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OptionalFieldsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> OptionalFieldsAddress<'a> {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl<'a> Default for OptionalFieldsAddress<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct OptionalFieldsShipping<'a> {
    /// Shipping address.
    pub address: OptionalFieldsAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> OptionalFieldsShipping<'a> {
    pub fn new(address: OptionalFieldsAddress<'a>, name: &'a str) -> Self {
        Self { address, carrier: None, name, phone: None, tracking_number: None }
    }
}
