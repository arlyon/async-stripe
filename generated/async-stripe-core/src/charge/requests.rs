use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListChargeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<String>,
}
impl ListChargeBuilder {
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
pub struct ListCharge {
    inner: ListChargeBuilder,
}
impl ListCharge {
    /// Construct a new `ListCharge`.
    pub fn new() -> Self {
        Self { inner: ListChargeBuilder::new() }
    }
    /// Only return charges that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return charges for the customer specified by this customer ID.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
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
    /// Only return charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    pub fn payment_intent(mut self, payment_intent: impl Into<String>) -> Self {
        self.inner.payment_intent = Some(payment_intent.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return charges for this transfer group, limited to 100.
    pub fn transfer_group(mut self, transfer_group: impl Into<String>) -> Self {
        self.inner.transfer_group = Some(transfer_group.into());
        self
    }
}
impl Default for ListCharge {
    fn default() -> Self {
        Self::new()
    }
}
impl ListCharge {
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
        stripe_client_core::ListPaginator::new_list("/charges", &self.inner)
    }
}

impl StripeRequest for ListCharge {
    type Output = stripe_types::List<stripe_shared::Charge>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/charges").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveChargeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveChargeBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of a charge that has previously been created.
/// Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information.
/// The same information is returned when creating or refunding the charge.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveCharge {
    inner: RetrieveChargeBuilder,
    charge: stripe_shared::ChargeId,
}
impl RetrieveCharge {
    /// Construct a new `RetrieveCharge`.
    pub fn new(charge: impl Into<stripe_shared::ChargeId>) -> Self {
        Self { charge: charge.into(), inner: RetrieveChargeBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveCharge {
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

impl StripeRequest for RetrieveCharge {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        let charge = &self.charge;
        RequestBuilder::new(StripeMethod::Get, format!("/charges/{charge}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct SearchChargeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    query: String,
}
impl SearchChargeBuilder {
    fn new(query: impl Into<String>) -> Self {
        Self { expand: None, limit: None, page: None, query: query.into() }
    }
}
/// Search for charges you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
/// Don’t use search in read-after-write flows where strict consistency is necessary.
/// Under normal operating.
/// conditions, data is searchable in less than a minute.
/// Occasionally, propagation of new or updated data can be up.
/// to an hour behind during outages. Search functionality is not available to merchants in India.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SearchCharge {
    inner: SearchChargeBuilder,
}
impl SearchCharge {
    /// Construct a new `SearchCharge`.
    pub fn new(query: impl Into<String>) -> Self {
        Self { inner: SearchChargeBuilder::new(query.into()) }
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
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    pub fn page(mut self, page: impl Into<String>) -> Self {
        self.inner.page = Some(page.into());
        self
    }
}
impl SearchCharge {
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
        stripe_client_core::ListPaginator::new_search_list("/charges/search", &self.inner)
    }
}

impl StripeRequest for SearchCharge {
    type Output = stripe_types::SearchList<stripe_shared::Charge>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/charges/search").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateChargeBuilder {
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
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<CreateChargeDestination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radar_options: Option<CreateChargeRadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<OptionalFieldsShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<CreateChargeTransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<String>,
}
impl CreateChargeBuilder {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateChargeDestination {
    /// ID of an existing, connected Stripe account.
    pub account: String,
    /// The amount to transfer to the destination account without creating an `Application Fee` object.
    /// Cannot be combined with the `application_fee` parameter.
    /// Must be less than or equal to the charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl CreateChargeDestination {
    pub fn new(account: impl Into<String>) -> Self {
        Self { account: account.into(), amount: None }
    }
}
/// Options to configure Radar.
/// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateChargeRadarOptions {
    /// A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}
impl CreateChargeRadarOptions {
    pub fn new() -> Self {
        Self { session: None }
    }
}
impl Default for CreateChargeRadarOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// An optional dictionary including the account to automatically transfer to as part of a destination charge.
/// [See the Connect documentation](https://docs.stripe.com/connect/destination-charges) for details.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateChargeTransferData {
    /// The amount transferred to the destination account, if specified.
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
impl CreateChargeTransferData {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, destination: destination.into() }
    }
}
/// This method is no longer recommended—use the [Payment Intents API](https://stripe.com/docs/api/payment_intents).
/// to initiate a new payment instead. Confirmation of the PaymentIntent creates the `Charge`
/// object used to request payment.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateCharge {
    inner: CreateChargeBuilder,
}
impl CreateCharge {
    /// Construct a new `CreateCharge`.
    pub fn new() -> Self {
        Self { inner: CreateChargeBuilder::new() }
    }
    /// Amount intended to be collected by this payment.
    /// A positive integer representing how much to charge in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://docs.stripe.com/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    pub fn application_fee(mut self, application_fee: impl Into<i64>) -> Self {
        self.inner.application_fee = Some(application_fee.into());
        self
    }
    /// A fee in cents (or local equivalent) that will be applied to the charge and transferred to the application owner's Stripe account.
    /// The request must be made with an OAuth key or the `Stripe-Account` header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://docs.stripe.com/connect/direct-charges#collect-fees).
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// Whether to immediately capture the charge.
    /// Defaults to `true`.
    /// When `false`, the charge issues an authorization (or pre-authorization), and will need to be [captured](https://api.stripe.com#capture_charge) later.
    /// Uncaptured charges expire after a set number of days (7 by default).
    /// For more information, see the [authorizing charges and settling later](https://docs.stripe.com/charges/placing-a-hold) documentation.
    pub fn capture(mut self, capture: impl Into<bool>) -> Self {
        self.inner.capture = Some(capture.into());
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// The ID of an existing customer that will be charged in this request.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// An arbitrary string which you can attach to a `Charge` object.
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    pub fn destination(mut self, destination: impl Into<CreateChargeDestination>) -> Self {
        self.inner.destination = Some(destination.into());
        self
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
    /// The Stripe account ID for which these funds are intended.
    /// You can specify the business of record as the connected account using the `on_behalf_of` attribute on the charge.
    /// For details, see [Creating Separate Charges and Transfers](https://docs.stripe.com/connect/separate-charges-and-transfers#settlement-merchant).
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// Options to configure Radar.
    /// See [Radar Session](https://docs.stripe.com/radar/radar-session) for more information.
    pub fn radar_options(mut self, radar_options: impl Into<CreateChargeRadarOptions>) -> Self {
        self.inner.radar_options = Some(radar_options.into());
        self
    }
    /// The email address to which this charge's [receipt](https://docs.stripe.com/dashboard/receipts) will be sent.
    /// The receipt will not be sent until the charge is paid, and no receipts will be sent for test mode charges.
    /// If this charge is for a [Customer](https://docs.stripe.com/api/customers/object), the email address specified here will override the customer's email address.
    /// If `receipt_email` is specified for a charge in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    pub fn receipt_email(mut self, receipt_email: impl Into<String>) -> Self {
        self.inner.receipt_email = Some(receipt_email.into());
        self
    }
    /// Shipping information for the charge. Helps prevent fraud on charges for physical goods.
    pub fn shipping(mut self, shipping: impl Into<OptionalFieldsShipping>) -> Self {
        self.inner.shipping = Some(shipping.into());
        self
    }
    /// A payment source to be charged.
    /// This can be the ID of a [card](https://docs.stripe.com/api#cards) (i.e., credit or debit card), a [bank account](https://docs.stripe.com/api#bank_accounts), a [source](https://docs.stripe.com/api#sources), a [token](https://docs.stripe.com/api#tokens), or a [connected account](https://docs.stripe.com/connect/account-debits#charging-a-connected-account).
    /// For certain sources---namely, [cards](https://docs.stripe.com/api#cards), [bank accounts](https://docs.stripe.com/api#bank_accounts), and attached [sources](https://docs.stripe.com/api#sources)---you must also pass the ID of the associated customer.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner.source = Some(source.into());
        self
    }
    /// For a non-card charge, text that appears on the customer's statement as the statement descriptor.
    /// This value overrides the account's default statement descriptor.
    /// For information about requirements, including the 22-character limit, see [the Statement Descriptor docs](https://docs.stripe.com/get-started/account/statement-descriptors).
    ///
    /// For a card charge, this value is ignored unless you don't specify a `statement_descriptor_suffix`, in which case this value is used as the suffix.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// Provides information about a card charge.
    /// Concatenated to the account's [statement descriptor prefix](https://docs.stripe.com/get-started/account/statement-descriptors#static) to form the complete statement descriptor that appears on the customer's statement.
    /// If the account has no prefix value, the suffix is concatenated to the account's statement descriptor.
    pub fn statement_descriptor_suffix(
        mut self,
        statement_descriptor_suffix: impl Into<String>,
    ) -> Self {
        self.inner.statement_descriptor_suffix = Some(statement_descriptor_suffix.into());
        self
    }
    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    /// [See the Connect documentation](https://docs.stripe.com/connect/destination-charges) for details.
    pub fn transfer_data(mut self, transfer_data: impl Into<CreateChargeTransferData>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
    /// A string that identifies this transaction as part of a group.
    /// For details, see [Grouping transactions](https://docs.stripe.com/connect/separate-charges-and-transfers#transfer-options).
    pub fn transfer_group(mut self, transfer_group: impl Into<String>) -> Self {
        self.inner.transfer_group = Some(transfer_group.into());
        self
    }
}
impl Default for CreateCharge {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateCharge {
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

impl StripeRequest for CreateCharge {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/charges").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateChargeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fraud_details: Option<UpdateChargeFraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<OptionalFieldsShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<String>,
}
impl UpdateChargeBuilder {
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateChargeFraudDetails {
    /// Either `safe` or `fraudulent`.
    pub user_report: UpdateChargeFraudDetailsUserReport,
}
impl UpdateChargeFraudDetails {
    pub fn new(user_report: impl Into<UpdateChargeFraudDetailsUserReport>) -> Self {
        Self { user_report: user_report.into() }
    }
}
/// Either `safe` or `fraudulent`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateChargeFraudDetailsUserReport {
    Fraudulent,
    Safe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateChargeFraudDetailsUserReport {
    pub fn as_str(&self) -> &str {
        use UpdateChargeFraudDetailsUserReport::*;
        match self {
            Fraudulent => "fraudulent",
            Safe => "safe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateChargeFraudDetailsUserReport {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateChargeFraudDetailsUserReport::*;
        match s {
            "fraudulent" => Ok(Fraudulent),
            "safe" => Ok(Safe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateChargeFraudDetailsUserReport"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Updates the specified charge by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateCharge {
    inner: UpdateChargeBuilder,
    charge: stripe_shared::ChargeId,
}
impl UpdateCharge {
    /// Construct a new `UpdateCharge`.
    pub fn new(charge: impl Into<stripe_shared::ChargeId>) -> Self {
        Self { charge: charge.into(), inner: UpdateChargeBuilder::new() }
    }
    /// The ID of an existing customer that will be associated with this request.
    /// This field may only be updated if there is no existing associated customer with this charge.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// An arbitrary string which you can attach to a charge object.
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A set of key-value pairs you can attach to a charge giving information about its riskiness.
    /// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
    /// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
    /// Stripe will use the information you send to improve our fraud detection algorithms.
    pub fn fraud_details(mut self, fraud_details: impl Into<UpdateChargeFraudDetails>) -> Self {
        self.inner.fraud_details = Some(fraud_details.into());
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
    /// This is the email address that the receipt for this charge will be sent to.
    /// If this field is updated, then a new email receipt will be sent to the updated address.
    pub fn receipt_email(mut self, receipt_email: impl Into<String>) -> Self {
        self.inner.receipt_email = Some(receipt_email.into());
        self
    }
    /// Shipping information for the charge. Helps prevent fraud on charges for physical goods.
    pub fn shipping(mut self, shipping: impl Into<OptionalFieldsShipping>) -> Self {
        self.inner.shipping = Some(shipping.into());
        self
    }
    /// A string that identifies this transaction as part of a group.
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://docs.stripe.com/connect/separate-charges-and-transfers#transfer-options) for details.
    pub fn transfer_group(mut self, transfer_group: impl Into<String>) -> Self {
        self.inner.transfer_group = Some(transfer_group.into());
        self
    }
}
impl UpdateCharge {
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

impl StripeRequest for UpdateCharge {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        let charge = &self.charge;
        RequestBuilder::new(StripeMethod::Post, format!("/charges/{charge}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CaptureChargeBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_descriptor_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<CaptureChargeTransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_group: Option<String>,
}
impl CaptureChargeBuilder {
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
/// [See the Connect documentation](https://docs.stripe.com/connect/destination-charges) for details.
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
pub struct CaptureCharge {
    inner: CaptureChargeBuilder,
    charge: stripe_shared::ChargeId,
}
impl CaptureCharge {
    /// Construct a new `CaptureCharge`.
    pub fn new(charge: impl Into<stripe_shared::ChargeId>) -> Self {
        Self { charge: charge.into(), inner: CaptureChargeBuilder::new() }
    }
    /// The amount to capture, which must be less than or equal to the original amount.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// An application fee to add on to this charge.
    pub fn application_fee(mut self, application_fee: impl Into<i64>) -> Self {
        self.inner.application_fee = Some(application_fee.into());
        self
    }
    /// An application fee amount to add on to this charge, which must be less than or equal to the original amount.
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The email address to send this charge's receipt to.
    /// This will override the previously-specified email address for this charge, if one was set.
    /// Receipts will not be sent in test mode.
    pub fn receipt_email(mut self, receipt_email: impl Into<String>) -> Self {
        self.inner.receipt_email = Some(receipt_email.into());
        self
    }
    /// For a non-card charge, text that appears on the customer's statement as the statement descriptor.
    /// This value overrides the account's default statement descriptor.
    /// For information about requirements, including the 22-character limit, see [the Statement Descriptor docs](https://docs.stripe.com/get-started/account/statement-descriptors).
    ///
    /// For a card charge, this value is ignored unless you don't specify a `statement_descriptor_suffix`, in which case this value is used as the suffix.
    pub fn statement_descriptor(mut self, statement_descriptor: impl Into<String>) -> Self {
        self.inner.statement_descriptor = Some(statement_descriptor.into());
        self
    }
    /// Provides information about a card charge.
    /// Concatenated to the account's [statement descriptor prefix](https://docs.stripe.com/get-started/account/statement-descriptors#static) to form the complete statement descriptor that appears on the customer's statement.
    /// If the account has no prefix value, the suffix is concatenated to the account's statement descriptor.
    pub fn statement_descriptor_suffix(
        mut self,
        statement_descriptor_suffix: impl Into<String>,
    ) -> Self {
        self.inner.statement_descriptor_suffix = Some(statement_descriptor_suffix.into());
        self
    }
    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    /// [See the Connect documentation](https://docs.stripe.com/connect/destination-charges) for details.
    pub fn transfer_data(mut self, transfer_data: impl Into<CaptureChargeTransferData>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
    /// A string that identifies this transaction as part of a group.
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://docs.stripe.com/connect/separate-charges-and-transfers#transfer-options) for details.
    pub fn transfer_group(mut self, transfer_group: impl Into<String>) -> Self {
        self.inner.transfer_group = Some(transfer_group.into());
        self
    }
}
impl CaptureCharge {
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

impl StripeRequest for CaptureCharge {
    type Output = stripe_shared::Charge;

    fn build(&self) -> RequestBuilder {
        let charge = &self.charge;
        RequestBuilder::new(StripeMethod::Post, format!("/charges/{charge}/capture"))
            .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct OptionalFieldsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl OptionalFieldsAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for OptionalFieldsAddress {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct OptionalFieldsShipping {
    /// Shipping address.
    pub address: OptionalFieldsAddress,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// Recipient name.
    pub name: String,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The tracking number for a physical product, obtained from the delivery service.
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}
impl OptionalFieldsShipping {
    pub fn new(address: impl Into<OptionalFieldsAddress>, name: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            carrier: None,
            name: name.into(),
            phone: None,
            tracking_number: None,
        }
    }
}
