use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListPaymentLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListPaymentLinkBuilder<'a> {
    fn new() -> Self {
        Self { active: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of your payment links.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPaymentLink<'a> {
    inner: ListPaymentLinkBuilder<'a>,
}
impl<'a> ListPaymentLink<'a> {
    /// Construct a new `ListPaymentLink`.
    pub fn new() -> Self {
        Self { inner: ListPaymentLinkBuilder::new() }
    }
    /// Only return payment links that are active or inactive (e.g., pass `false` to list all inactive payment links).
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListPaymentLink<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPaymentLink<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::PaymentLink>> {
        stripe_client_core::ListPaginator::new_list("/payment_links", self.inner)
    }
}

impl StripeRequest for ListPaymentLink<'_> {
    type Output = stripe_types::List<stripe_shared::PaymentLink>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_links").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePaymentLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentLinkBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieve a payment link.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentLink<'a> {
    inner: RetrievePaymentLinkBuilder<'a>,
    payment_link: &'a stripe_shared::PaymentLinkId,
}
impl<'a> RetrievePaymentLink<'a> {
    /// Construct a new `RetrievePaymentLink`.
    pub fn new(payment_link: &'a stripe_shared::PaymentLinkId) -> Self {
        Self { payment_link, inner: RetrievePaymentLinkBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePaymentLink<'_> {
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

impl StripeRequest for RetrievePaymentLink<'_> {
    type Output = stripe_shared::PaymentLink;

    fn build(&self) -> RequestBuilder {
        let payment_link = self.payment_link;
        RequestBuilder::new(StripeMethod::Get, format!("/payment_links/{payment_link}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListLineItemsPaymentLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListLineItemsPaymentLinkBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// When retrieving a payment link, there is an includable **line_items** property containing the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListLineItemsPaymentLink<'a> {
    inner: ListLineItemsPaymentLinkBuilder<'a>,
    payment_link: &'a stripe_shared::PaymentLinkId,
}
impl<'a> ListLineItemsPaymentLink<'a> {
    /// Construct a new `ListLineItemsPaymentLink`.
    pub fn new(payment_link: &'a stripe_shared::PaymentLinkId) -> Self {
        Self { payment_link, inner: ListLineItemsPaymentLinkBuilder::new() }
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl ListLineItemsPaymentLink<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::CheckoutSessionItem>>
    {
        let payment_link = self.payment_link;

        stripe_client_core::ListPaginator::new_list(
            format!("/payment_links/{payment_link}/line_items"),
            self.inner,
        )
    }
}

impl StripeRequest for ListLineItemsPaymentLink<'_> {
    type Output = stripe_types::List<stripe_shared::CheckoutSessionItem>;

    fn build(&self) -> RequestBuilder {
        let payment_link = self.payment_link;
        RequestBuilder::new(StripeMethod::Get, format!("/payment_links/{payment_link}/line_items"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreatePaymentLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    after_completion: Option<CreatePaymentLinkAfterCompletion<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<CreatePaymentLinkAutomaticTax<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_address_collection: Option<stripe_shared::PaymentLinkBillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consent_collection: Option<CreatePaymentLinkConsentCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<&'a [CreatePaymentLinkCustomFields<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_text: Option<CustomTextParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_creation: Option<CreatePaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_creation: Option<CreatePaymentLinkInvoiceCreation<'a>>,
    line_items: &'a [CreatePaymentLinkLineItems<'a>],
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent_data: Option<CreatePaymentLinkPaymentIntentData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_collection: Option<CreatePaymentLinkPaymentMethodCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<&'a [stripe_shared::PaymentLinkPaymentMethodTypes]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number_collection: Option<CreatePaymentLinkPhoneNumberCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<RestrictionsParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address_collection: Option<CreatePaymentLinkShippingAddressCollection<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<&'a [CreatePaymentLinkShippingOptions<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_type: Option<stripe_shared::PaymentLinkSubmitType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<CreatePaymentLinkSubscriptionData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id_collection: Option<CreatePaymentLinkTaxIdCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<CreatePaymentLinkTransferData<'a>>,
}
impl<'a> CreatePaymentLinkBuilder<'a> {
    fn new(line_items: &'a [CreatePaymentLinkLineItems<'a>]) -> Self {
        Self {
            after_completion: None,
            allow_promotion_codes: None,
            application_fee_amount: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_address_collection: None,
            consent_collection: None,
            currency: None,
            custom_fields: None,
            custom_text: None,
            customer_creation: None,
            expand: None,
            inactive_message: None,
            invoice_creation: None,
            line_items,
            metadata: None,
            on_behalf_of: None,
            payment_intent_data: None,
            payment_method_collection: None,
            payment_method_types: None,
            phone_number_collection: None,
            restrictions: None,
            shipping_address_collection: None,
            shipping_options: None,
            submit_type: None,
            subscription_data: None,
            tax_id_collection: None,
            transfer_data: None,
        }
    }
}
/// Behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAfterCompletion<'a> {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<AfterCompletionConfirmationPageParams<'a>>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<AfterCompletionRedirectParams<'a>>,
    /// The specified behavior after the purchase is complete. Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkAfterCompletionType,
}
impl<'a> CreatePaymentLinkAfterCompletion<'a> {
    pub fn new(type_: CreatePaymentLinkAfterCompletionType) -> Self {
        Self { hosted_confirmation: None, redirect: None, type_ }
    }
}
/// The specified behavior after the purchase is complete. Either `redirect` or `hosted_confirmation`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}
impl CreatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkAfterCompletionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentLinkAfterCompletionType")
        })
    }
}
/// Configuration for automatic tax collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAutomaticTax<'a> {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreatePaymentLinkAutomaticTaxLiability<'a>>,
}
impl<'a> CreatePaymentLinkAutomaticTax<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkAutomaticTaxLiability<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkAutomaticTaxLiabilityType,
}
impl<'a> CreatePaymentLinkAutomaticTaxLiability<'a> {
    pub fn new(type_: CreatePaymentLinkAutomaticTaxLiabilityType) -> Self {
        Self { account: None, type_ }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreatePaymentLinkAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentLinkAutomaticTaxLiabilityType")
        })
    }
}
/// Configure fields to gather active consent from customers.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkConsentCollection {
    /// Determines the display of payment method reuse agreement text in the UI.
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement:
        Option<CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreement>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    /// The Checkout.
    /// Session will determine whether to display an option to opt into promotional communication
    /// from the merchant depending on the customer's locale. Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreatePaymentLinkConsentCollectionPromotions>,
    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreatePaymentLinkConsentCollectionTermsOfService>,
}
impl CreatePaymentLinkConsentCollection {
    pub fn new() -> Self {
        Self { payment_method_reuse_agreement: None, promotions: None, terms_of_service: None }
    }
}
impl Default for CreatePaymentLinkConsentCollection {
    fn default() -> Self {
        Self::new()
    }
}
/// Determines the display of payment method reuse agreement text in the UI.
/// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    /// When set to `auto`, Stripe's.
    /// defaults will be used.
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition,
}
impl CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreement {
    pub fn new(
        position: CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition,
    ) -> Self {
        Self { position }
    }
}
/// Determines the position and visibility of the payment method reuse agreement in the UI.
/// When set to `auto`, Stripe's.
/// defaults will be used.
/// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}
impl CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition::*;
        match self {
            Auto => "auto",
            Hidden => "hidden",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition::*;
        match s {
            "auto" => Ok(Auto),
            "hidden" => Ok(Hidden),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition"))
    }
}
/// If set to `auto`, enables the collection of customer consent for promotional communications.
/// The Checkout.
/// Session will determine whether to display an option to opt into promotional communication
/// from the merchant depending on the customer's locale. Only available to US merchants.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkConsentCollectionPromotions {
    Auto,
    None,
}
impl CreatePaymentLinkConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkConsentCollectionPromotions::*;
        match self {
            Auto => "auto",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkConsentCollectionPromotions {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkConsentCollectionPromotions::*;
        match s {
            "auto" => Ok(Auto),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkConsentCollectionPromotions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkConsentCollectionPromotions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentLinkConsentCollectionPromotions",
            )
        })
    }
}
/// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
/// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkConsentCollectionTermsOfService {
    None,
    Required,
}
impl CreatePaymentLinkConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkConsentCollectionTermsOfService::*;
        match self {
            None => "none",
            Required => "required",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkConsentCollectionTermsOfService {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkConsentCollectionTermsOfService::*;
        match s {
            "none" => Ok(None),
            "required" => Ok(Required),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkConsentCollectionTermsOfService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkConsentCollectionTermsOfService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentLinkConsentCollectionTermsOfService",
            )
        })
    }
}
/// Collect additional information from your customer using custom fields.
/// Up to 3 fields are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFields<'a> {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CustomFieldDropdownParam<'a>>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: &'a str,
    /// The label for the field, displayed to the customer.
    pub label: CreatePaymentLinkCustomFieldsLabel<'a>,
    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CreatePaymentLinkCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CreatePaymentLinkCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkCustomFieldsType,
}
impl<'a> CreatePaymentLinkCustomFields<'a> {
    pub fn new(
        key: &'a str,
        label: CreatePaymentLinkCustomFieldsLabel<'a>,
        type_: CreatePaymentLinkCustomFieldsType,
    ) -> Self {
        Self { dropdown: None, key, label, numeric: None, optional: None, text: None, type_ }
    }
}
/// The label for the field, displayed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsLabel<'a> {
    /// Custom text for the label, displayed to the customer. Up to 50 characters.
    pub custom: &'a str,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkCustomFieldsLabelType,
}
impl<'a> CreatePaymentLinkCustomFieldsLabel<'a> {
    pub fn new(custom: &'a str, type_: CreatePaymentLinkCustomFieldsLabelType) -> Self {
        Self { custom, type_ }
    }
}
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkCustomFieldsLabelType {
    Custom,
}
impl CreatePaymentLinkCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkCustomFieldsLabelType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkCustomFieldsLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentLinkCustomFieldsLabelType")
        })
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl CreatePaymentLinkCustomFieldsNumeric {
    pub fn new() -> Self {
        Self { maximum_length: None, minimum_length: None }
    }
}
impl Default for CreatePaymentLinkCustomFieldsNumeric {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for `type=text` fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl CreatePaymentLinkCustomFieldsText {
    pub fn new() -> Self {
        Self { maximum_length: None, minimum_length: None }
    }
}
impl Default for CreatePaymentLinkCustomFieldsText {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}
impl CreatePaymentLinkCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkCustomFieldsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentLinkCustomFieldsType")
        })
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkCustomerCreation {
    Always,
    IfRequired,
}
impl CreatePaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkCustomerCreation {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentLinkCustomerCreation")
        })
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreation<'a> {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreatePaymentLinkInvoiceCreationInvoiceData<'a>>,
}
impl<'a> CreatePaymentLinkInvoiceCreation<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, invoice_data: None }
    }
}
/// Invoice PDF configuration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceData<'a> {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<&'a [&'a str]>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CustomFieldParams<'a>]>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreatePaymentLinkInvoiceCreationInvoiceDataIssuer<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions>,
}
impl<'a> CreatePaymentLinkInvoiceCreationInvoiceData<'a> {
    pub fn new() -> Self {
        Self {
            account_tax_ids: None,
            custom_fields: None,
            description: None,
            footer: None,
            issuer: None,
            metadata: None,
            rendering_options: None,
        }
    }
}
impl<'a> Default for CreatePaymentLinkInvoiceCreationInvoiceData<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataIssuer<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType,
}
impl<'a> CreatePaymentLinkInvoiceCreationInvoiceDataIssuer<'a> {
    pub fn new(type_: CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType) -> Self {
        Self { account: None, type_ }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    Account,
    Self_,
}
impl CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType",
            )
        })
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}
impl CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    pub fn new() -> Self {
        Self { amount_tax_display: None }
    }
}
impl Default for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay"))
    }
}
/// The line items representing what is being sold.
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<AdjustableQuantityParams>,
    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    pub price: &'a str,
    /// The quantity of the line item being purchased.
    pub quantity: u64,
}
impl<'a> CreatePaymentLinkLineItems<'a> {
    pub fn new(price: &'a str, quantity: u64) -> Self {
        Self { adjustable_quantity: None, price, quantity }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkPaymentIntentData<'a> {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentLinkPaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached to a Customer.
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
    ///
    /// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentLinkPaymentIntentDataSetupFutureUsage>,
    /// Extra information about the payment.
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about the charge that customers see on their statements.
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> CreatePaymentLinkPaymentIntentData<'a> {
    pub fn new() -> Self {
        Self {
            capture_method: None,
            description: None,
            metadata: None,
            setup_future_usage: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_group: None,
        }
    }
}
impl<'a> Default for CreatePaymentLinkPaymentIntentData<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkPaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}
impl CreatePaymentLinkPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkPaymentIntentDataCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkPaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentLinkPaymentIntentDataCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
///
/// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
///
/// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
///
/// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
///
/// If Checkout does not create a Customer, the payment method is not attached to a Customer.
/// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.
///
/// When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}
impl CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkPaymentIntentDataSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkPaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentLinkPaymentIntentDataSetupFutureUsage",
            )
        })
    }
}
/// Specify whether Checkout should collect a payment method.
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
///
/// Can only be set in `subscription` mode. Defaults to `always`.
///
/// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}
impl CreatePaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkPaymentMethodCollection {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreatePaymentLinkPaymentMethodCollection")
        })
    }
}
/// Controls phone number collection settings during checkout.
///
/// We recommend that you review your privacy policy and check with your legal contacts.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    pub enabled: bool,
}
impl CreatePaymentLinkPhoneNumberCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Configuration for collecting the customer's shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
    /// shipping locations.
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [CreatePaymentLinkShippingAddressCollectionAllowedCountries],
}
impl<'a> CreatePaymentLinkShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [CreatePaymentLinkShippingAddressCollectionAllowedCountries],
    ) -> Self {
        Self { allowed_countries }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
/// shipping locations.
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    Ac,
    Ad,
    Ae,
    Af,
    Ag,
    Ai,
    Al,
    Am,
    Ao,
    Aq,
    Ar,
    At,
    Au,
    Aw,
    Ax,
    Az,
    Ba,
    Bb,
    Bd,
    Be,
    Bf,
    Bg,
    Bh,
    Bi,
    Bj,
    Bl,
    Bm,
    Bn,
    Bo,
    Bq,
    Br,
    Bs,
    Bt,
    Bv,
    Bw,
    By,
    Bz,
    Ca,
    Cd,
    Cf,
    Cg,
    Ch,
    Ci,
    Ck,
    Cl,
    Cm,
    Cn,
    Co,
    Cr,
    Cv,
    Cw,
    Cy,
    Cz,
    De,
    Dj,
    Dk,
    Dm,
    Do,
    Dz,
    Ec,
    Ee,
    Eg,
    Eh,
    Er,
    Es,
    Et,
    Fi,
    Fj,
    Fk,
    Fo,
    Fr,
    Ga,
    Gb,
    Gd,
    Ge,
    Gf,
    Gg,
    Gh,
    Gi,
    Gl,
    Gm,
    Gn,
    Gp,
    Gq,
    Gr,
    Gs,
    Gt,
    Gu,
    Gw,
    Gy,
    Hk,
    Hn,
    Hr,
    Ht,
    Hu,
    Id,
    Ie,
    Il,
    Im,
    In,
    Io,
    Iq,
    Is,
    It,
    Je,
    Jm,
    Jo,
    Jp,
    Ke,
    Kg,
    Kh,
    Ki,
    Km,
    Kn,
    Kr,
    Kw,
    Ky,
    Kz,
    La,
    Lb,
    Lc,
    Li,
    Lk,
    Lr,
    Ls,
    Lt,
    Lu,
    Lv,
    Ly,
    Ma,
    Mc,
    Md,
    Me,
    Mf,
    Mg,
    Mk,
    Ml,
    Mm,
    Mn,
    Mo,
    Mq,
    Mr,
    Ms,
    Mt,
    Mu,
    Mv,
    Mw,
    Mx,
    My,
    Mz,
    Na,
    Nc,
    Ne,
    Ng,
    Ni,
    Nl,
    No,
    Np,
    Nr,
    Nu,
    Nz,
    Om,
    Pa,
    Pe,
    Pf,
    Pg,
    Ph,
    Pk,
    Pl,
    Pm,
    Pn,
    Pr,
    Ps,
    Pt,
    Py,
    Qa,
    Re,
    Ro,
    Rs,
    Ru,
    Rw,
    Sa,
    Sb,
    Sc,
    Se,
    Sg,
    Sh,
    Si,
    Sj,
    Sk,
    Sl,
    Sm,
    Sn,
    So,
    Sr,
    Ss,
    St,
    Sv,
    Sx,
    Sz,
    Ta,
    Tc,
    Td,
    Tf,
    Tg,
    Th,
    Tj,
    Tk,
    Tl,
    Tm,
    Tn,
    To,
    Tr,
    Tt,
    Tv,
    Tw,
    Tz,
    Ua,
    Ug,
    Us,
    Uy,
    Uz,
    Va,
    Vc,
    Ve,
    Vg,
    Vn,
    Vu,
    Wf,
    Ws,
    Xk,
    Ye,
    Yt,
    Za,
    Zm,
    Zw,
    Zz,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match self {
            Ac => "AC",
            Ad => "AD",
            Ae => "AE",
            Af => "AF",
            Ag => "AG",
            Ai => "AI",
            Al => "AL",
            Am => "AM",
            Ao => "AO",
            Aq => "AQ",
            Ar => "AR",
            At => "AT",
            Au => "AU",
            Aw => "AW",
            Ax => "AX",
            Az => "AZ",
            Ba => "BA",
            Bb => "BB",
            Bd => "BD",
            Be => "BE",
            Bf => "BF",
            Bg => "BG",
            Bh => "BH",
            Bi => "BI",
            Bj => "BJ",
            Bl => "BL",
            Bm => "BM",
            Bn => "BN",
            Bo => "BO",
            Bq => "BQ",
            Br => "BR",
            Bs => "BS",
            Bt => "BT",
            Bv => "BV",
            Bw => "BW",
            By => "BY",
            Bz => "BZ",
            Ca => "CA",
            Cd => "CD",
            Cf => "CF",
            Cg => "CG",
            Ch => "CH",
            Ci => "CI",
            Ck => "CK",
            Cl => "CL",
            Cm => "CM",
            Cn => "CN",
            Co => "CO",
            Cr => "CR",
            Cv => "CV",
            Cw => "CW",
            Cy => "CY",
            Cz => "CZ",
            De => "DE",
            Dj => "DJ",
            Dk => "DK",
            Dm => "DM",
            Do => "DO",
            Dz => "DZ",
            Ec => "EC",
            Ee => "EE",
            Eg => "EG",
            Eh => "EH",
            Er => "ER",
            Es => "ES",
            Et => "ET",
            Fi => "FI",
            Fj => "FJ",
            Fk => "FK",
            Fo => "FO",
            Fr => "FR",
            Ga => "GA",
            Gb => "GB",
            Gd => "GD",
            Ge => "GE",
            Gf => "GF",
            Gg => "GG",
            Gh => "GH",
            Gi => "GI",
            Gl => "GL",
            Gm => "GM",
            Gn => "GN",
            Gp => "GP",
            Gq => "GQ",
            Gr => "GR",
            Gs => "GS",
            Gt => "GT",
            Gu => "GU",
            Gw => "GW",
            Gy => "GY",
            Hk => "HK",
            Hn => "HN",
            Hr => "HR",
            Ht => "HT",
            Hu => "HU",
            Id => "ID",
            Ie => "IE",
            Il => "IL",
            Im => "IM",
            In => "IN",
            Io => "IO",
            Iq => "IQ",
            Is => "IS",
            It => "IT",
            Je => "JE",
            Jm => "JM",
            Jo => "JO",
            Jp => "JP",
            Ke => "KE",
            Kg => "KG",
            Kh => "KH",
            Ki => "KI",
            Km => "KM",
            Kn => "KN",
            Kr => "KR",
            Kw => "KW",
            Ky => "KY",
            Kz => "KZ",
            La => "LA",
            Lb => "LB",
            Lc => "LC",
            Li => "LI",
            Lk => "LK",
            Lr => "LR",
            Ls => "LS",
            Lt => "LT",
            Lu => "LU",
            Lv => "LV",
            Ly => "LY",
            Ma => "MA",
            Mc => "MC",
            Md => "MD",
            Me => "ME",
            Mf => "MF",
            Mg => "MG",
            Mk => "MK",
            Ml => "ML",
            Mm => "MM",
            Mn => "MN",
            Mo => "MO",
            Mq => "MQ",
            Mr => "MR",
            Ms => "MS",
            Mt => "MT",
            Mu => "MU",
            Mv => "MV",
            Mw => "MW",
            Mx => "MX",
            My => "MY",
            Mz => "MZ",
            Na => "NA",
            Nc => "NC",
            Ne => "NE",
            Ng => "NG",
            Ni => "NI",
            Nl => "NL",
            No => "NO",
            Np => "NP",
            Nr => "NR",
            Nu => "NU",
            Nz => "NZ",
            Om => "OM",
            Pa => "PA",
            Pe => "PE",
            Pf => "PF",
            Pg => "PG",
            Ph => "PH",
            Pk => "PK",
            Pl => "PL",
            Pm => "PM",
            Pn => "PN",
            Pr => "PR",
            Ps => "PS",
            Pt => "PT",
            Py => "PY",
            Qa => "QA",
            Re => "RE",
            Ro => "RO",
            Rs => "RS",
            Ru => "RU",
            Rw => "RW",
            Sa => "SA",
            Sb => "SB",
            Sc => "SC",
            Se => "SE",
            Sg => "SG",
            Sh => "SH",
            Si => "SI",
            Sj => "SJ",
            Sk => "SK",
            Sl => "SL",
            Sm => "SM",
            Sn => "SN",
            So => "SO",
            Sr => "SR",
            Ss => "SS",
            St => "ST",
            Sv => "SV",
            Sx => "SX",
            Sz => "SZ",
            Ta => "TA",
            Tc => "TC",
            Td => "TD",
            Tf => "TF",
            Tg => "TG",
            Th => "TH",
            Tj => "TJ",
            Tk => "TK",
            Tl => "TL",
            Tm => "TM",
            Tn => "TN",
            To => "TO",
            Tr => "TR",
            Tt => "TT",
            Tv => "TV",
            Tw => "TW",
            Tz => "TZ",
            Ua => "UA",
            Ug => "UG",
            Us => "US",
            Uy => "UY",
            Uz => "UZ",
            Va => "VA",
            Vc => "VC",
            Ve => "VE",
            Vg => "VG",
            Vn => "VN",
            Vu => "VU",
            Wf => "WF",
            Ws => "WS",
            Xk => "XK",
            Ye => "YE",
            Yt => "YT",
            Za => "ZA",
            Zm => "ZM",
            Zw => "ZW",
            Zz => "ZZ",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match s {
            "AC" => Ok(Ac),
            "AD" => Ok(Ad),
            "AE" => Ok(Ae),
            "AF" => Ok(Af),
            "AG" => Ok(Ag),
            "AI" => Ok(Ai),
            "AL" => Ok(Al),
            "AM" => Ok(Am),
            "AO" => Ok(Ao),
            "AQ" => Ok(Aq),
            "AR" => Ok(Ar),
            "AT" => Ok(At),
            "AU" => Ok(Au),
            "AW" => Ok(Aw),
            "AX" => Ok(Ax),
            "AZ" => Ok(Az),
            "BA" => Ok(Ba),
            "BB" => Ok(Bb),
            "BD" => Ok(Bd),
            "BE" => Ok(Be),
            "BF" => Ok(Bf),
            "BG" => Ok(Bg),
            "BH" => Ok(Bh),
            "BI" => Ok(Bi),
            "BJ" => Ok(Bj),
            "BL" => Ok(Bl),
            "BM" => Ok(Bm),
            "BN" => Ok(Bn),
            "BO" => Ok(Bo),
            "BQ" => Ok(Bq),
            "BR" => Ok(Br),
            "BS" => Ok(Bs),
            "BT" => Ok(Bt),
            "BV" => Ok(Bv),
            "BW" => Ok(Bw),
            "BY" => Ok(By),
            "BZ" => Ok(Bz),
            "CA" => Ok(Ca),
            "CD" => Ok(Cd),
            "CF" => Ok(Cf),
            "CG" => Ok(Cg),
            "CH" => Ok(Ch),
            "CI" => Ok(Ci),
            "CK" => Ok(Ck),
            "CL" => Ok(Cl),
            "CM" => Ok(Cm),
            "CN" => Ok(Cn),
            "CO" => Ok(Co),
            "CR" => Ok(Cr),
            "CV" => Ok(Cv),
            "CW" => Ok(Cw),
            "CY" => Ok(Cy),
            "CZ" => Ok(Cz),
            "DE" => Ok(De),
            "DJ" => Ok(Dj),
            "DK" => Ok(Dk),
            "DM" => Ok(Dm),
            "DO" => Ok(Do),
            "DZ" => Ok(Dz),
            "EC" => Ok(Ec),
            "EE" => Ok(Ee),
            "EG" => Ok(Eg),
            "EH" => Ok(Eh),
            "ER" => Ok(Er),
            "ES" => Ok(Es),
            "ET" => Ok(Et),
            "FI" => Ok(Fi),
            "FJ" => Ok(Fj),
            "FK" => Ok(Fk),
            "FO" => Ok(Fo),
            "FR" => Ok(Fr),
            "GA" => Ok(Ga),
            "GB" => Ok(Gb),
            "GD" => Ok(Gd),
            "GE" => Ok(Ge),
            "GF" => Ok(Gf),
            "GG" => Ok(Gg),
            "GH" => Ok(Gh),
            "GI" => Ok(Gi),
            "GL" => Ok(Gl),
            "GM" => Ok(Gm),
            "GN" => Ok(Gn),
            "GP" => Ok(Gp),
            "GQ" => Ok(Gq),
            "GR" => Ok(Gr),
            "GS" => Ok(Gs),
            "GT" => Ok(Gt),
            "GU" => Ok(Gu),
            "GW" => Ok(Gw),
            "GY" => Ok(Gy),
            "HK" => Ok(Hk),
            "HN" => Ok(Hn),
            "HR" => Ok(Hr),
            "HT" => Ok(Ht),
            "HU" => Ok(Hu),
            "ID" => Ok(Id),
            "IE" => Ok(Ie),
            "IL" => Ok(Il),
            "IM" => Ok(Im),
            "IN" => Ok(In),
            "IO" => Ok(Io),
            "IQ" => Ok(Iq),
            "IS" => Ok(Is),
            "IT" => Ok(It),
            "JE" => Ok(Je),
            "JM" => Ok(Jm),
            "JO" => Ok(Jo),
            "JP" => Ok(Jp),
            "KE" => Ok(Ke),
            "KG" => Ok(Kg),
            "KH" => Ok(Kh),
            "KI" => Ok(Ki),
            "KM" => Ok(Km),
            "KN" => Ok(Kn),
            "KR" => Ok(Kr),
            "KW" => Ok(Kw),
            "KY" => Ok(Ky),
            "KZ" => Ok(Kz),
            "LA" => Ok(La),
            "LB" => Ok(Lb),
            "LC" => Ok(Lc),
            "LI" => Ok(Li),
            "LK" => Ok(Lk),
            "LR" => Ok(Lr),
            "LS" => Ok(Ls),
            "LT" => Ok(Lt),
            "LU" => Ok(Lu),
            "LV" => Ok(Lv),
            "LY" => Ok(Ly),
            "MA" => Ok(Ma),
            "MC" => Ok(Mc),
            "MD" => Ok(Md),
            "ME" => Ok(Me),
            "MF" => Ok(Mf),
            "MG" => Ok(Mg),
            "MK" => Ok(Mk),
            "ML" => Ok(Ml),
            "MM" => Ok(Mm),
            "MN" => Ok(Mn),
            "MO" => Ok(Mo),
            "MQ" => Ok(Mq),
            "MR" => Ok(Mr),
            "MS" => Ok(Ms),
            "MT" => Ok(Mt),
            "MU" => Ok(Mu),
            "MV" => Ok(Mv),
            "MW" => Ok(Mw),
            "MX" => Ok(Mx),
            "MY" => Ok(My),
            "MZ" => Ok(Mz),
            "NA" => Ok(Na),
            "NC" => Ok(Nc),
            "NE" => Ok(Ne),
            "NG" => Ok(Ng),
            "NI" => Ok(Ni),
            "NL" => Ok(Nl),
            "NO" => Ok(No),
            "NP" => Ok(Np),
            "NR" => Ok(Nr),
            "NU" => Ok(Nu),
            "NZ" => Ok(Nz),
            "OM" => Ok(Om),
            "PA" => Ok(Pa),
            "PE" => Ok(Pe),
            "PF" => Ok(Pf),
            "PG" => Ok(Pg),
            "PH" => Ok(Ph),
            "PK" => Ok(Pk),
            "PL" => Ok(Pl),
            "PM" => Ok(Pm),
            "PN" => Ok(Pn),
            "PR" => Ok(Pr),
            "PS" => Ok(Ps),
            "PT" => Ok(Pt),
            "PY" => Ok(Py),
            "QA" => Ok(Qa),
            "RE" => Ok(Re),
            "RO" => Ok(Ro),
            "RS" => Ok(Rs),
            "RU" => Ok(Ru),
            "RW" => Ok(Rw),
            "SA" => Ok(Sa),
            "SB" => Ok(Sb),
            "SC" => Ok(Sc),
            "SE" => Ok(Se),
            "SG" => Ok(Sg),
            "SH" => Ok(Sh),
            "SI" => Ok(Si),
            "SJ" => Ok(Sj),
            "SK" => Ok(Sk),
            "SL" => Ok(Sl),
            "SM" => Ok(Sm),
            "SN" => Ok(Sn),
            "SO" => Ok(So),
            "SR" => Ok(Sr),
            "SS" => Ok(Ss),
            "ST" => Ok(St),
            "SV" => Ok(Sv),
            "SX" => Ok(Sx),
            "SZ" => Ok(Sz),
            "TA" => Ok(Ta),
            "TC" => Ok(Tc),
            "TD" => Ok(Td),
            "TF" => Ok(Tf),
            "TG" => Ok(Tg),
            "TH" => Ok(Th),
            "TJ" => Ok(Tj),
            "TK" => Ok(Tk),
            "TL" => Ok(Tl),
            "TM" => Ok(Tm),
            "TN" => Ok(Tn),
            "TO" => Ok(To),
            "TR" => Ok(Tr),
            "TT" => Ok(Tt),
            "TV" => Ok(Tv),
            "TW" => Ok(Tw),
            "TZ" => Ok(Tz),
            "UA" => Ok(Ua),
            "UG" => Ok(Ug),
            "US" => Ok(Us),
            "UY" => Ok(Uy),
            "UZ" => Ok(Uz),
            "VA" => Ok(Va),
            "VC" => Ok(Vc),
            "VE" => Ok(Ve),
            "VG" => Ok(Vg),
            "VN" => Ok(Vn),
            "VU" => Ok(Vu),
            "WF" => Ok(Wf),
            "WS" => Ok(Ws),
            "XK" => Ok(Xk),
            "YE" => Ok(Ye),
            "YT" => Ok(Yt),
            "ZA" => Ok(Za),
            "ZM" => Ok(Zm),
            "ZW" => Ok(Zw),
            "ZZ" => Ok(Zz),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// The shipping rate options to apply to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkShippingOptions<'a> {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
}
impl<'a> CreatePaymentLinkShippingOptions<'a> {
    pub fn new() -> Self {
        Self { shipping_rate: None }
    }
}
impl<'a> Default for CreatePaymentLinkShippingOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// When creating a subscription, the specified configuration data will be used.
/// There must be at least one line item with a recurring price to use `subscription_data`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionData<'a> {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreatePaymentLinkSubscriptionDataInvoiceSettings<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreatePaymentLinkSubscriptionDataTrialSettings>,
}
impl<'a> CreatePaymentLinkSubscriptionData<'a> {
    pub fn new() -> Self {
        Self {
            description: None,
            invoice_settings: None,
            metadata: None,
            trial_period_days: None,
            trial_settings: None,
        }
    }
}
impl<'a> Default for CreatePaymentLinkSubscriptionData<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionDataInvoiceSettings<'a> {
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer<'a>>,
}
impl<'a> CreatePaymentLinkSubscriptionDataInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self { issuer: None }
    }
}
impl<'a> Default for CreatePaymentLinkSubscriptionDataInvoiceSettings<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType,
}
impl<'a> CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer<'a> {
    pub fn new(type_: CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType) -> Self {
        Self { account: None, type_ }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType",
            )
        })
    }
}
/// Settings related to subscription trials.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionDataTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior,
}
impl CreatePaymentLinkSubscriptionDataTrialSettings {
    pub fn new(end_behavior: CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior) -> Self {
        Self { end_behavior }
    }
}
/// Defines how the subscription should behave when the user's free trial ends.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method:
        CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}
impl CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior {
    pub fn new(
        missing_payment_method: CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
    ) -> Self {
        Self { missing_payment_method }
    }
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}
impl CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr
    for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod"))
    }
}
/// Controls tax ID collection during checkout.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkTaxIdCollection {
    /// Set to `true` to enable tax ID collection.
    pub enabled: bool,
}
impl CreatePaymentLinkTaxIdCollection {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLinkTransferData<'a> {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account. The ID of the resulting transfer will be
    /// returned on the successful charge's `transfer` field.
    pub destination: &'a str,
}
impl<'a> CreatePaymentLinkTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: None, destination }
    }
}
/// Creates a payment link.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePaymentLink<'a> {
    inner: CreatePaymentLinkBuilder<'a>,
}
impl<'a> CreatePaymentLink<'a> {
    /// Construct a new `CreatePaymentLink`.
    pub fn new(line_items: &'a [CreatePaymentLinkLineItems<'a>]) -> Self {
        Self { inner: CreatePaymentLinkBuilder::new(line_items) }
    }
    /// Behavior after the purchase is complete.
    pub fn after_completion(
        mut self,
        after_completion: CreatePaymentLinkAfterCompletion<'a>,
    ) -> Self {
        self.inner.after_completion = Some(after_completion);
        self
    }
    /// Enables user redeemable promotion codes.
    pub fn allow_promotion_codes(mut self, allow_promotion_codes: bool) -> Self {
        self.inner.allow_promotion_codes = Some(allow_promotion_codes);
        self
    }
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// Can only be applied when there are no line items with recurring prices.
    pub fn application_fee_amount(mut self, application_fee_amount: i64) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount);
        self
    }
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    pub fn application_fee_percent(mut self, application_fee_percent: f64) -> Self {
        self.inner.application_fee_percent = Some(application_fee_percent);
        self
    }
    /// Configuration for automatic tax collection.
    pub fn automatic_tax(mut self, automatic_tax: CreatePaymentLinkAutomaticTax<'a>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax);
        self
    }
    /// Configuration for collecting the customer's billing address. Defaults to `auto`.
    pub fn billing_address_collection(
        mut self,
        billing_address_collection: stripe_shared::PaymentLinkBillingAddressCollection,
    ) -> Self {
        self.inner.billing_address_collection = Some(billing_address_collection);
        self
    }
    /// Configure fields to gather active consent from customers.
    pub fn consent_collection(
        mut self,
        consent_collection: CreatePaymentLinkConsentCollection,
    ) -> Self {
        self.inner.consent_collection = Some(consent_collection);
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price.
    pub fn currency(mut self, currency: stripe_types::Currency) -> Self {
        self.inner.currency = Some(currency);
        self
    }
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    pub fn custom_fields(mut self, custom_fields: &'a [CreatePaymentLinkCustomFields<'a>]) -> Self {
        self.inner.custom_fields = Some(custom_fields);
        self
    }
    /// Display additional text for your customers using custom text.
    pub fn custom_text(mut self, custom_text: CustomTextParam<'a>) -> Self {
        self.inner.custom_text = Some(custom_text);
        self
    }
    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    pub fn customer_creation(
        mut self,
        customer_creation: CreatePaymentLinkCustomerCreation,
    ) -> Self {
        self.inner.customer_creation = Some(customer_creation);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The custom message to be displayed to a customer when a payment link is no longer active.
    pub fn inactive_message(mut self, inactive_message: &'a str) -> Self {
        self.inner.inactive_message = Some(inactive_message);
        self
    }
    /// Generate a post-purchase Invoice for one-time payments.
    pub fn invoice_creation(
        mut self,
        invoice_creation: CreatePaymentLinkInvoiceCreation<'a>,
    ) -> Self {
        self.inner.invoice_creation = Some(invoice_creation);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The account on behalf of which to charge.
    pub fn on_behalf_of(mut self, on_behalf_of: &'a str) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of);
        self
    }
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    pub fn payment_intent_data(
        mut self,
        payment_intent_data: CreatePaymentLinkPaymentIntentData<'a>,
    ) -> Self {
        self.inner.payment_intent_data = Some(payment_intent_data);
        self
    }
    /// Specify whether Checkout should collect a payment method.
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
    ///
    /// Can only be set in `subscription` mode. Defaults to `always`.
    ///
    /// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    pub fn payment_method_collection(
        mut self,
        payment_method_collection: CreatePaymentLinkPaymentMethodCollection,
    ) -> Self {
        self.inner.payment_method_collection = Some(payment_method_collection);
        self
    }
    /// The list of payment method types that customers can use.
    /// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support)).
    pub fn payment_method_types(
        mut self,
        payment_method_types: &'a [stripe_shared::PaymentLinkPaymentMethodTypes],
    ) -> Self {
        self.inner.payment_method_types = Some(payment_method_types);
        self
    }
    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    pub fn phone_number_collection(
        mut self,
        phone_number_collection: CreatePaymentLinkPhoneNumberCollection,
    ) -> Self {
        self.inner.phone_number_collection = Some(phone_number_collection);
        self
    }
    /// Settings that restrict the usage of a payment link.
    pub fn restrictions(mut self, restrictions: RestrictionsParams) -> Self {
        self.inner.restrictions = Some(restrictions);
        self
    }
    /// Configuration for collecting the customer's shipping address.
    pub fn shipping_address_collection(
        mut self,
        shipping_address_collection: CreatePaymentLinkShippingAddressCollection<'a>,
    ) -> Self {
        self.inner.shipping_address_collection = Some(shipping_address_collection);
        self
    }
    /// The shipping rate options to apply to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    pub fn shipping_options(
        mut self,
        shipping_options: &'a [CreatePaymentLinkShippingOptions<'a>],
    ) -> Self {
        self.inner.shipping_options = Some(shipping_options);
        self
    }
    /// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
    /// Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
    pub fn submit_type(mut self, submit_type: stripe_shared::PaymentLinkSubmitType) -> Self {
        self.inner.submit_type = Some(submit_type);
        self
    }
    /// When creating a subscription, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub fn subscription_data(
        mut self,
        subscription_data: CreatePaymentLinkSubscriptionData<'a>,
    ) -> Self {
        self.inner.subscription_data = Some(subscription_data);
        self
    }
    /// Controls tax ID collection during checkout.
    pub fn tax_id_collection(
        mut self,
        tax_id_collection: CreatePaymentLinkTaxIdCollection,
    ) -> Self {
        self.inner.tax_id_collection = Some(tax_id_collection);
        self
    }
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub fn transfer_data(mut self, transfer_data: CreatePaymentLinkTransferData<'a>) -> Self {
        self.inner.transfer_data = Some(transfer_data);
        self
    }
}
impl CreatePaymentLink<'_> {
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

impl StripeRequest for CreatePaymentLink<'_> {
    type Output = stripe_shared::PaymentLink;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_links").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdatePaymentLinkBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_completion: Option<UpdatePaymentLinkAfterCompletion<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<UpdatePaymentLinkAutomaticTax<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_address_collection: Option<stripe_shared::PaymentLinkBillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<&'a [UpdatePaymentLinkCustomFields<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_text: Option<CustomTextParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_creation: Option<UpdatePaymentLinkCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_creation: Option<UpdatePaymentLinkInvoiceCreation<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<&'a [UpdatePaymentLinkLineItems<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent_data: Option<UpdatePaymentLinkPaymentIntentData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_collection: Option<UpdatePaymentLinkPaymentMethodCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_types: Option<&'a [stripe_shared::PaymentLinkPaymentMethodTypes]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions: Option<RestrictionsParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_address_collection: Option<UpdatePaymentLinkShippingAddressCollection<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<UpdatePaymentLinkSubscriptionData<'a>>,
}
impl<'a> UpdatePaymentLinkBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            after_completion: None,
            allow_promotion_codes: None,
            automatic_tax: None,
            billing_address_collection: None,
            custom_fields: None,
            custom_text: None,
            customer_creation: None,
            expand: None,
            inactive_message: None,
            invoice_creation: None,
            line_items: None,
            metadata: None,
            payment_intent_data: None,
            payment_method_collection: None,
            payment_method_types: None,
            restrictions: None,
            shipping_address_collection: None,
            subscription_data: None,
        }
    }
}
/// Behavior after the purchase is complete.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAfterCompletion<'a> {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<AfterCompletionConfirmationPageParams<'a>>,
    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<AfterCompletionRedirectParams<'a>>,
    /// The specified behavior after the purchase is complete. Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkAfterCompletionType,
}
impl<'a> UpdatePaymentLinkAfterCompletion<'a> {
    pub fn new(type_: UpdatePaymentLinkAfterCompletionType) -> Self {
        Self { hosted_confirmation: None, redirect: None, type_ }
    }
}
/// The specified behavior after the purchase is complete. Either `redirect` or `hosted_confirmation`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}
impl UpdatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkAfterCompletionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "redirect" => Ok(Redirect),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkAfterCompletionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentLinkAfterCompletionType")
        })
    }
}
/// Configuration for automatic tax collection.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAutomaticTax<'a> {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdatePaymentLinkAutomaticTaxLiability<'a>>,
}
impl<'a> UpdatePaymentLinkAutomaticTax<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkAutomaticTaxLiability<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkAutomaticTaxLiabilityType,
}
impl<'a> UpdatePaymentLinkAutomaticTaxLiability<'a> {
    pub fn new(type_: UpdatePaymentLinkAutomaticTaxLiabilityType) -> Self {
        Self { account: None, type_ }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpdatePaymentLinkAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentLinkAutomaticTaxLiabilityType")
        })
    }
}
/// Collect additional information from your customer using custom fields.
/// Up to 3 fields are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFields<'a> {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CustomFieldDropdownParam<'a>>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: &'a str,
    /// The label for the field, displayed to the customer.
    pub label: UpdatePaymentLinkCustomFieldsLabel<'a>,
    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<UpdatePaymentLinkCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<UpdatePaymentLinkCustomFieldsText>,
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkCustomFieldsType,
}
impl<'a> UpdatePaymentLinkCustomFields<'a> {
    pub fn new(
        key: &'a str,
        label: UpdatePaymentLinkCustomFieldsLabel<'a>,
        type_: UpdatePaymentLinkCustomFieldsType,
    ) -> Self {
        Self { dropdown: None, key, label, numeric: None, optional: None, text: None, type_ }
    }
}
/// The label for the field, displayed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsLabel<'a> {
    /// Custom text for the label, displayed to the customer. Up to 50 characters.
    pub custom: &'a str,
    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkCustomFieldsLabelType,
}
impl<'a> UpdatePaymentLinkCustomFieldsLabel<'a> {
    pub fn new(custom: &'a str, type_: UpdatePaymentLinkCustomFieldsLabelType) -> Self {
        Self { custom, type_ }
    }
}
/// The type of the label.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkCustomFieldsLabelType {
    Custom,
}
impl UpdatePaymentLinkCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkCustomFieldsLabelType::*;
        match self {
            Custom => "custom",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkCustomFieldsLabelType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkCustomFieldsLabelType::*;
        match s {
            "custom" => Ok(Custom),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkCustomFieldsLabelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkCustomFieldsLabelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentLinkCustomFieldsLabelType")
        })
    }
}
/// Configuration for `type=numeric` fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl UpdatePaymentLinkCustomFieldsNumeric {
    pub fn new() -> Self {
        Self { maximum_length: None, minimum_length: None }
    }
}
impl Default for UpdatePaymentLinkCustomFieldsNumeric {
    fn default() -> Self {
        Self::new()
    }
}
/// Configuration for `type=text` fields.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}
impl UpdatePaymentLinkCustomFieldsText {
    pub fn new() -> Self {
        Self { maximum_length: None, minimum_length: None }
    }
}
impl Default for UpdatePaymentLinkCustomFieldsText {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}
impl UpdatePaymentLinkCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkCustomFieldsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentLinkCustomFieldsType")
        })
    }
}
/// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkCustomerCreation {
    Always,
    IfRequired,
}
impl UpdatePaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkCustomerCreation {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentLinkCustomerCreation")
        })
    }
}
/// Generate a post-purchase Invoice for one-time payments.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreation<'a> {
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<UpdatePaymentLinkInvoiceCreationInvoiceData<'a>>,
}
impl<'a> UpdatePaymentLinkInvoiceCreation<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, invoice_data: None }
    }
}
/// Invoice PDF configuration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceData<'a> {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<&'a [&'a str]>,
    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CustomFieldParams<'a>]>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions>,
}
impl<'a> UpdatePaymentLinkInvoiceCreationInvoiceData<'a> {
    pub fn new() -> Self {
        Self {
            account_tax_ids: None,
            custom_fields: None,
            description: None,
            footer: None,
            issuer: None,
            metadata: None,
            rendering_options: None,
        }
    }
}
impl<'a> Default for UpdatePaymentLinkInvoiceCreationInvoiceData<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType,
}
impl<'a> UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer<'a> {
    pub fn new(type_: UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType) -> Self {
        Self { account: None, type_ }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    Account,
    Self_,
}
impl UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType",
            )
        })
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}
impl UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    pub fn new() -> Self {
        Self { amount_tax_display: None }
    }
}
impl Default for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay"))
    }
}
/// The line items representing what is being sold.
/// Each line item represents an item being sold.
/// Up to 20 line items are supported.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkLineItems<'a> {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<AdjustableQuantityParams>,
    /// The ID of an existing line item on the payment link.
    pub id: &'a str,
    /// The quantity of the line item being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
impl<'a> UpdatePaymentLinkLineItems<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { adjustable_quantity: None, id, quantity: None }
    }
}
/// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkPaymentIntentData<'a> {
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Extra information about the payment.
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about the charge that customers see on their statements.
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> UpdatePaymentLinkPaymentIntentData<'a> {
    pub fn new() -> Self {
        Self {
            description: None,
            metadata: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_group: None,
        }
    }
}
impl<'a> Default for UpdatePaymentLinkPaymentIntentData<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Specify whether Checkout should collect a payment method.
/// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
///
/// Can only be set in `subscription` mode. Defaults to `always`.
///
/// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}
impl UpdatePaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkPaymentMethodCollection {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdatePaymentLinkPaymentMethodCollection")
        })
    }
}
/// Configuration for collecting the customer's shipping address.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkShippingAddressCollection<'a> {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
    /// shipping locations.
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: &'a [UpdatePaymentLinkShippingAddressCollectionAllowedCountries],
}
impl<'a> UpdatePaymentLinkShippingAddressCollection<'a> {
    pub fn new(
        allowed_countries: &'a [UpdatePaymentLinkShippingAddressCollectionAllowedCountries],
    ) -> Self {
        Self { allowed_countries }
    }
}
/// An array of two-letter ISO country codes representing which countries Checkout should provide as options for.
/// shipping locations.
/// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    Ac,
    Ad,
    Ae,
    Af,
    Ag,
    Ai,
    Al,
    Am,
    Ao,
    Aq,
    Ar,
    At,
    Au,
    Aw,
    Ax,
    Az,
    Ba,
    Bb,
    Bd,
    Be,
    Bf,
    Bg,
    Bh,
    Bi,
    Bj,
    Bl,
    Bm,
    Bn,
    Bo,
    Bq,
    Br,
    Bs,
    Bt,
    Bv,
    Bw,
    By,
    Bz,
    Ca,
    Cd,
    Cf,
    Cg,
    Ch,
    Ci,
    Ck,
    Cl,
    Cm,
    Cn,
    Co,
    Cr,
    Cv,
    Cw,
    Cy,
    Cz,
    De,
    Dj,
    Dk,
    Dm,
    Do,
    Dz,
    Ec,
    Ee,
    Eg,
    Eh,
    Er,
    Es,
    Et,
    Fi,
    Fj,
    Fk,
    Fo,
    Fr,
    Ga,
    Gb,
    Gd,
    Ge,
    Gf,
    Gg,
    Gh,
    Gi,
    Gl,
    Gm,
    Gn,
    Gp,
    Gq,
    Gr,
    Gs,
    Gt,
    Gu,
    Gw,
    Gy,
    Hk,
    Hn,
    Hr,
    Ht,
    Hu,
    Id,
    Ie,
    Il,
    Im,
    In,
    Io,
    Iq,
    Is,
    It,
    Je,
    Jm,
    Jo,
    Jp,
    Ke,
    Kg,
    Kh,
    Ki,
    Km,
    Kn,
    Kr,
    Kw,
    Ky,
    Kz,
    La,
    Lb,
    Lc,
    Li,
    Lk,
    Lr,
    Ls,
    Lt,
    Lu,
    Lv,
    Ly,
    Ma,
    Mc,
    Md,
    Me,
    Mf,
    Mg,
    Mk,
    Ml,
    Mm,
    Mn,
    Mo,
    Mq,
    Mr,
    Ms,
    Mt,
    Mu,
    Mv,
    Mw,
    Mx,
    My,
    Mz,
    Na,
    Nc,
    Ne,
    Ng,
    Ni,
    Nl,
    No,
    Np,
    Nr,
    Nu,
    Nz,
    Om,
    Pa,
    Pe,
    Pf,
    Pg,
    Ph,
    Pk,
    Pl,
    Pm,
    Pn,
    Pr,
    Ps,
    Pt,
    Py,
    Qa,
    Re,
    Ro,
    Rs,
    Ru,
    Rw,
    Sa,
    Sb,
    Sc,
    Se,
    Sg,
    Sh,
    Si,
    Sj,
    Sk,
    Sl,
    Sm,
    Sn,
    So,
    Sr,
    Ss,
    St,
    Sv,
    Sx,
    Sz,
    Ta,
    Tc,
    Td,
    Tf,
    Tg,
    Th,
    Tj,
    Tk,
    Tl,
    Tm,
    Tn,
    To,
    Tr,
    Tt,
    Tv,
    Tw,
    Tz,
    Ua,
    Ug,
    Us,
    Uy,
    Uz,
    Va,
    Vc,
    Ve,
    Vg,
    Vn,
    Vu,
    Wf,
    Ws,
    Xk,
    Ye,
    Yt,
    Za,
    Zm,
    Zw,
    Zz,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match self {
            Ac => "AC",
            Ad => "AD",
            Ae => "AE",
            Af => "AF",
            Ag => "AG",
            Ai => "AI",
            Al => "AL",
            Am => "AM",
            Ao => "AO",
            Aq => "AQ",
            Ar => "AR",
            At => "AT",
            Au => "AU",
            Aw => "AW",
            Ax => "AX",
            Az => "AZ",
            Ba => "BA",
            Bb => "BB",
            Bd => "BD",
            Be => "BE",
            Bf => "BF",
            Bg => "BG",
            Bh => "BH",
            Bi => "BI",
            Bj => "BJ",
            Bl => "BL",
            Bm => "BM",
            Bn => "BN",
            Bo => "BO",
            Bq => "BQ",
            Br => "BR",
            Bs => "BS",
            Bt => "BT",
            Bv => "BV",
            Bw => "BW",
            By => "BY",
            Bz => "BZ",
            Ca => "CA",
            Cd => "CD",
            Cf => "CF",
            Cg => "CG",
            Ch => "CH",
            Ci => "CI",
            Ck => "CK",
            Cl => "CL",
            Cm => "CM",
            Cn => "CN",
            Co => "CO",
            Cr => "CR",
            Cv => "CV",
            Cw => "CW",
            Cy => "CY",
            Cz => "CZ",
            De => "DE",
            Dj => "DJ",
            Dk => "DK",
            Dm => "DM",
            Do => "DO",
            Dz => "DZ",
            Ec => "EC",
            Ee => "EE",
            Eg => "EG",
            Eh => "EH",
            Er => "ER",
            Es => "ES",
            Et => "ET",
            Fi => "FI",
            Fj => "FJ",
            Fk => "FK",
            Fo => "FO",
            Fr => "FR",
            Ga => "GA",
            Gb => "GB",
            Gd => "GD",
            Ge => "GE",
            Gf => "GF",
            Gg => "GG",
            Gh => "GH",
            Gi => "GI",
            Gl => "GL",
            Gm => "GM",
            Gn => "GN",
            Gp => "GP",
            Gq => "GQ",
            Gr => "GR",
            Gs => "GS",
            Gt => "GT",
            Gu => "GU",
            Gw => "GW",
            Gy => "GY",
            Hk => "HK",
            Hn => "HN",
            Hr => "HR",
            Ht => "HT",
            Hu => "HU",
            Id => "ID",
            Ie => "IE",
            Il => "IL",
            Im => "IM",
            In => "IN",
            Io => "IO",
            Iq => "IQ",
            Is => "IS",
            It => "IT",
            Je => "JE",
            Jm => "JM",
            Jo => "JO",
            Jp => "JP",
            Ke => "KE",
            Kg => "KG",
            Kh => "KH",
            Ki => "KI",
            Km => "KM",
            Kn => "KN",
            Kr => "KR",
            Kw => "KW",
            Ky => "KY",
            Kz => "KZ",
            La => "LA",
            Lb => "LB",
            Lc => "LC",
            Li => "LI",
            Lk => "LK",
            Lr => "LR",
            Ls => "LS",
            Lt => "LT",
            Lu => "LU",
            Lv => "LV",
            Ly => "LY",
            Ma => "MA",
            Mc => "MC",
            Md => "MD",
            Me => "ME",
            Mf => "MF",
            Mg => "MG",
            Mk => "MK",
            Ml => "ML",
            Mm => "MM",
            Mn => "MN",
            Mo => "MO",
            Mq => "MQ",
            Mr => "MR",
            Ms => "MS",
            Mt => "MT",
            Mu => "MU",
            Mv => "MV",
            Mw => "MW",
            Mx => "MX",
            My => "MY",
            Mz => "MZ",
            Na => "NA",
            Nc => "NC",
            Ne => "NE",
            Ng => "NG",
            Ni => "NI",
            Nl => "NL",
            No => "NO",
            Np => "NP",
            Nr => "NR",
            Nu => "NU",
            Nz => "NZ",
            Om => "OM",
            Pa => "PA",
            Pe => "PE",
            Pf => "PF",
            Pg => "PG",
            Ph => "PH",
            Pk => "PK",
            Pl => "PL",
            Pm => "PM",
            Pn => "PN",
            Pr => "PR",
            Ps => "PS",
            Pt => "PT",
            Py => "PY",
            Qa => "QA",
            Re => "RE",
            Ro => "RO",
            Rs => "RS",
            Ru => "RU",
            Rw => "RW",
            Sa => "SA",
            Sb => "SB",
            Sc => "SC",
            Se => "SE",
            Sg => "SG",
            Sh => "SH",
            Si => "SI",
            Sj => "SJ",
            Sk => "SK",
            Sl => "SL",
            Sm => "SM",
            Sn => "SN",
            So => "SO",
            Sr => "SR",
            Ss => "SS",
            St => "ST",
            Sv => "SV",
            Sx => "SX",
            Sz => "SZ",
            Ta => "TA",
            Tc => "TC",
            Td => "TD",
            Tf => "TF",
            Tg => "TG",
            Th => "TH",
            Tj => "TJ",
            Tk => "TK",
            Tl => "TL",
            Tm => "TM",
            Tn => "TN",
            To => "TO",
            Tr => "TR",
            Tt => "TT",
            Tv => "TV",
            Tw => "TW",
            Tz => "TZ",
            Ua => "UA",
            Ug => "UG",
            Us => "US",
            Uy => "UY",
            Uz => "UZ",
            Va => "VA",
            Vc => "VC",
            Ve => "VE",
            Vg => "VG",
            Vn => "VN",
            Vu => "VU",
            Wf => "WF",
            Ws => "WS",
            Xk => "XK",
            Ye => "YE",
            Yt => "YT",
            Za => "ZA",
            Zm => "ZM",
            Zw => "ZW",
            Zz => "ZZ",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkShippingAddressCollectionAllowedCountries::*;
        match s {
            "AC" => Ok(Ac),
            "AD" => Ok(Ad),
            "AE" => Ok(Ae),
            "AF" => Ok(Af),
            "AG" => Ok(Ag),
            "AI" => Ok(Ai),
            "AL" => Ok(Al),
            "AM" => Ok(Am),
            "AO" => Ok(Ao),
            "AQ" => Ok(Aq),
            "AR" => Ok(Ar),
            "AT" => Ok(At),
            "AU" => Ok(Au),
            "AW" => Ok(Aw),
            "AX" => Ok(Ax),
            "AZ" => Ok(Az),
            "BA" => Ok(Ba),
            "BB" => Ok(Bb),
            "BD" => Ok(Bd),
            "BE" => Ok(Be),
            "BF" => Ok(Bf),
            "BG" => Ok(Bg),
            "BH" => Ok(Bh),
            "BI" => Ok(Bi),
            "BJ" => Ok(Bj),
            "BL" => Ok(Bl),
            "BM" => Ok(Bm),
            "BN" => Ok(Bn),
            "BO" => Ok(Bo),
            "BQ" => Ok(Bq),
            "BR" => Ok(Br),
            "BS" => Ok(Bs),
            "BT" => Ok(Bt),
            "BV" => Ok(Bv),
            "BW" => Ok(Bw),
            "BY" => Ok(By),
            "BZ" => Ok(Bz),
            "CA" => Ok(Ca),
            "CD" => Ok(Cd),
            "CF" => Ok(Cf),
            "CG" => Ok(Cg),
            "CH" => Ok(Ch),
            "CI" => Ok(Ci),
            "CK" => Ok(Ck),
            "CL" => Ok(Cl),
            "CM" => Ok(Cm),
            "CN" => Ok(Cn),
            "CO" => Ok(Co),
            "CR" => Ok(Cr),
            "CV" => Ok(Cv),
            "CW" => Ok(Cw),
            "CY" => Ok(Cy),
            "CZ" => Ok(Cz),
            "DE" => Ok(De),
            "DJ" => Ok(Dj),
            "DK" => Ok(Dk),
            "DM" => Ok(Dm),
            "DO" => Ok(Do),
            "DZ" => Ok(Dz),
            "EC" => Ok(Ec),
            "EE" => Ok(Ee),
            "EG" => Ok(Eg),
            "EH" => Ok(Eh),
            "ER" => Ok(Er),
            "ES" => Ok(Es),
            "ET" => Ok(Et),
            "FI" => Ok(Fi),
            "FJ" => Ok(Fj),
            "FK" => Ok(Fk),
            "FO" => Ok(Fo),
            "FR" => Ok(Fr),
            "GA" => Ok(Ga),
            "GB" => Ok(Gb),
            "GD" => Ok(Gd),
            "GE" => Ok(Ge),
            "GF" => Ok(Gf),
            "GG" => Ok(Gg),
            "GH" => Ok(Gh),
            "GI" => Ok(Gi),
            "GL" => Ok(Gl),
            "GM" => Ok(Gm),
            "GN" => Ok(Gn),
            "GP" => Ok(Gp),
            "GQ" => Ok(Gq),
            "GR" => Ok(Gr),
            "GS" => Ok(Gs),
            "GT" => Ok(Gt),
            "GU" => Ok(Gu),
            "GW" => Ok(Gw),
            "GY" => Ok(Gy),
            "HK" => Ok(Hk),
            "HN" => Ok(Hn),
            "HR" => Ok(Hr),
            "HT" => Ok(Ht),
            "HU" => Ok(Hu),
            "ID" => Ok(Id),
            "IE" => Ok(Ie),
            "IL" => Ok(Il),
            "IM" => Ok(Im),
            "IN" => Ok(In),
            "IO" => Ok(Io),
            "IQ" => Ok(Iq),
            "IS" => Ok(Is),
            "IT" => Ok(It),
            "JE" => Ok(Je),
            "JM" => Ok(Jm),
            "JO" => Ok(Jo),
            "JP" => Ok(Jp),
            "KE" => Ok(Ke),
            "KG" => Ok(Kg),
            "KH" => Ok(Kh),
            "KI" => Ok(Ki),
            "KM" => Ok(Km),
            "KN" => Ok(Kn),
            "KR" => Ok(Kr),
            "KW" => Ok(Kw),
            "KY" => Ok(Ky),
            "KZ" => Ok(Kz),
            "LA" => Ok(La),
            "LB" => Ok(Lb),
            "LC" => Ok(Lc),
            "LI" => Ok(Li),
            "LK" => Ok(Lk),
            "LR" => Ok(Lr),
            "LS" => Ok(Ls),
            "LT" => Ok(Lt),
            "LU" => Ok(Lu),
            "LV" => Ok(Lv),
            "LY" => Ok(Ly),
            "MA" => Ok(Ma),
            "MC" => Ok(Mc),
            "MD" => Ok(Md),
            "ME" => Ok(Me),
            "MF" => Ok(Mf),
            "MG" => Ok(Mg),
            "MK" => Ok(Mk),
            "ML" => Ok(Ml),
            "MM" => Ok(Mm),
            "MN" => Ok(Mn),
            "MO" => Ok(Mo),
            "MQ" => Ok(Mq),
            "MR" => Ok(Mr),
            "MS" => Ok(Ms),
            "MT" => Ok(Mt),
            "MU" => Ok(Mu),
            "MV" => Ok(Mv),
            "MW" => Ok(Mw),
            "MX" => Ok(Mx),
            "MY" => Ok(My),
            "MZ" => Ok(Mz),
            "NA" => Ok(Na),
            "NC" => Ok(Nc),
            "NE" => Ok(Ne),
            "NG" => Ok(Ng),
            "NI" => Ok(Ni),
            "NL" => Ok(Nl),
            "NO" => Ok(No),
            "NP" => Ok(Np),
            "NR" => Ok(Nr),
            "NU" => Ok(Nu),
            "NZ" => Ok(Nz),
            "OM" => Ok(Om),
            "PA" => Ok(Pa),
            "PE" => Ok(Pe),
            "PF" => Ok(Pf),
            "PG" => Ok(Pg),
            "PH" => Ok(Ph),
            "PK" => Ok(Pk),
            "PL" => Ok(Pl),
            "PM" => Ok(Pm),
            "PN" => Ok(Pn),
            "PR" => Ok(Pr),
            "PS" => Ok(Ps),
            "PT" => Ok(Pt),
            "PY" => Ok(Py),
            "QA" => Ok(Qa),
            "RE" => Ok(Re),
            "RO" => Ok(Ro),
            "RS" => Ok(Rs),
            "RU" => Ok(Ru),
            "RW" => Ok(Rw),
            "SA" => Ok(Sa),
            "SB" => Ok(Sb),
            "SC" => Ok(Sc),
            "SE" => Ok(Se),
            "SG" => Ok(Sg),
            "SH" => Ok(Sh),
            "SI" => Ok(Si),
            "SJ" => Ok(Sj),
            "SK" => Ok(Sk),
            "SL" => Ok(Sl),
            "SM" => Ok(Sm),
            "SN" => Ok(Sn),
            "SO" => Ok(So),
            "SR" => Ok(Sr),
            "SS" => Ok(Ss),
            "ST" => Ok(St),
            "SV" => Ok(Sv),
            "SX" => Ok(Sx),
            "SZ" => Ok(Sz),
            "TA" => Ok(Ta),
            "TC" => Ok(Tc),
            "TD" => Ok(Td),
            "TF" => Ok(Tf),
            "TG" => Ok(Tg),
            "TH" => Ok(Th),
            "TJ" => Ok(Tj),
            "TK" => Ok(Tk),
            "TL" => Ok(Tl),
            "TM" => Ok(Tm),
            "TN" => Ok(Tn),
            "TO" => Ok(To),
            "TR" => Ok(Tr),
            "TT" => Ok(Tt),
            "TV" => Ok(Tv),
            "TW" => Ok(Tw),
            "TZ" => Ok(Tz),
            "UA" => Ok(Ua),
            "UG" => Ok(Ug),
            "US" => Ok(Us),
            "UY" => Ok(Uy),
            "UZ" => Ok(Uz),
            "VA" => Ok(Va),
            "VC" => Ok(Vc),
            "VE" => Ok(Ve),
            "VG" => Ok(Vg),
            "VN" => Ok(Vn),
            "VU" => Ok(Vu),
            "WF" => Ok(Wf),
            "WS" => Ok(Ws),
            "XK" => Ok(Xk),
            "YE" => Ok(Ye),
            "YT" => Ok(Yt),
            "ZA" => Ok(Za),
            "ZM" => Ok(Zm),
            "ZW" => Ok(Zw),
            "ZZ" => Ok(Zz),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// When creating a subscription, the specified configuration data will be used.
/// There must be at least one line item with a recurring price to use `subscription_data`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionData<'a> {
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdatePaymentLinkSubscriptionDataInvoiceSettings<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<UpdatePaymentLinkSubscriptionDataTrialSettings>,
}
impl<'a> UpdatePaymentLinkSubscriptionData<'a> {
    pub fn new() -> Self {
        Self { invoice_settings: None, metadata: None, trial_settings: None }
    }
}
impl<'a> Default for UpdatePaymentLinkSubscriptionData<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataInvoiceSettings<'a> {
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer<'a>>,
}
impl<'a> UpdatePaymentLinkSubscriptionDataInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self { issuer: None }
    }
}
impl<'a> Default for UpdatePaymentLinkSubscriptionDataInvoiceSettings<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer<'a> {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType,
}
impl<'a> UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer<'a> {
    pub fn new(type_: UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType) -> Self {
        Self { account: None, type_ }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType",
            )
        })
    }
}
/// Settings related to subscription trials.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior,
}
impl UpdatePaymentLinkSubscriptionDataTrialSettings {
    pub fn new(end_behavior: UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior) -> Self {
        Self { end_behavior }
    }
}
/// Defines how the subscription should behave when the user's free trial ends.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method:
        UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}
impl UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior {
    pub fn new(
        missing_payment_method: UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
    ) -> Self {
        Self { missing_payment_method }
    }
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}
impl UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
        }
    }
}

impl std::str::FromStr
    for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod"))
    }
}
/// Updates a payment link.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentLink<'a> {
    inner: UpdatePaymentLinkBuilder<'a>,
    payment_link: &'a stripe_shared::PaymentLinkId,
}
impl<'a> UpdatePaymentLink<'a> {
    /// Construct a new `UpdatePaymentLink`.
    pub fn new(payment_link: &'a stripe_shared::PaymentLinkId) -> Self {
        Self { payment_link, inner: UpdatePaymentLinkBuilder::new() }
    }
    /// Whether the payment link's `url` is active.
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// Behavior after the purchase is complete.
    pub fn after_completion(
        mut self,
        after_completion: UpdatePaymentLinkAfterCompletion<'a>,
    ) -> Self {
        self.inner.after_completion = Some(after_completion);
        self
    }
    /// Enables user redeemable promotion codes.
    pub fn allow_promotion_codes(mut self, allow_promotion_codes: bool) -> Self {
        self.inner.allow_promotion_codes = Some(allow_promotion_codes);
        self
    }
    /// Configuration for automatic tax collection.
    pub fn automatic_tax(mut self, automatic_tax: UpdatePaymentLinkAutomaticTax<'a>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax);
        self
    }
    /// Configuration for collecting the customer's billing address. Defaults to `auto`.
    pub fn billing_address_collection(
        mut self,
        billing_address_collection: stripe_shared::PaymentLinkBillingAddressCollection,
    ) -> Self {
        self.inner.billing_address_collection = Some(billing_address_collection);
        self
    }
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    pub fn custom_fields(mut self, custom_fields: &'a [UpdatePaymentLinkCustomFields<'a>]) -> Self {
        self.inner.custom_fields = Some(custom_fields);
        self
    }
    /// Display additional text for your customers using custom text.
    pub fn custom_text(mut self, custom_text: CustomTextParam<'a>) -> Self {
        self.inner.custom_text = Some(custom_text);
        self
    }
    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    pub fn customer_creation(
        mut self,
        customer_creation: UpdatePaymentLinkCustomerCreation,
    ) -> Self {
        self.inner.customer_creation = Some(customer_creation);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The custom message to be displayed to a customer when a payment link is no longer active.
    pub fn inactive_message(mut self, inactive_message: &'a str) -> Self {
        self.inner.inactive_message = Some(inactive_message);
        self
    }
    /// Generate a post-purchase Invoice for one-time payments.
    pub fn invoice_creation(
        mut self,
        invoice_creation: UpdatePaymentLinkInvoiceCreation<'a>,
    ) -> Self {
        self.inner.invoice_creation = Some(invoice_creation);
        self
    }
    /// The line items representing what is being sold.
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    pub fn line_items(mut self, line_items: &'a [UpdatePaymentLinkLineItems<'a>]) -> Self {
        self.inner.line_items = Some(line_items);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    pub fn payment_intent_data(
        mut self,
        payment_intent_data: UpdatePaymentLinkPaymentIntentData<'a>,
    ) -> Self {
        self.inner.payment_intent_data = Some(payment_intent_data);
        self
    }
    /// Specify whether Checkout should collect a payment method.
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.
    ///
    /// Can only be set in `subscription` mode. Defaults to `always`.
    ///
    /// If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    pub fn payment_method_collection(
        mut self,
        payment_method_collection: UpdatePaymentLinkPaymentMethodCollection,
    ) -> Self {
        self.inner.payment_method_collection = Some(payment_method_collection);
        self
    }
    /// The list of payment method types that customers can use.
    /// Pass an empty string to enable dynamic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub fn payment_method_types(
        mut self,
        payment_method_types: &'a [stripe_shared::PaymentLinkPaymentMethodTypes],
    ) -> Self {
        self.inner.payment_method_types = Some(payment_method_types);
        self
    }
    /// Settings that restrict the usage of a payment link.
    pub fn restrictions(mut self, restrictions: RestrictionsParams) -> Self {
        self.inner.restrictions = Some(restrictions);
        self
    }
    /// Configuration for collecting the customer's shipping address.
    pub fn shipping_address_collection(
        mut self,
        shipping_address_collection: UpdatePaymentLinkShippingAddressCollection<'a>,
    ) -> Self {
        self.inner.shipping_address_collection = Some(shipping_address_collection);
        self
    }
    /// When creating a subscription, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub fn subscription_data(
        mut self,
        subscription_data: UpdatePaymentLinkSubscriptionData<'a>,
    ) -> Self {
        self.inner.subscription_data = Some(subscription_data);
        self
    }
}
impl UpdatePaymentLink<'_> {
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

impl StripeRequest for UpdatePaymentLink<'_> {
    type Output = stripe_shared::PaymentLink;

    fn build(&self) -> RequestBuilder {
        let payment_link = self.payment_link;
        RequestBuilder::new(StripeMethod::Post, format!("/payment_links/{payment_link}"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AfterCompletionConfirmationPageParams<'a> {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> AfterCompletionConfirmationPageParams<'a> {
    pub fn new() -> Self {
        Self { custom_message: None }
    }
}
impl<'a> Default for AfterCompletionConfirmationPageParams<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AfterCompletionRedirectParams<'a> {
    /// The URL the customer will be redirected to after the purchase is complete.
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: &'a str,
}
impl<'a> AfterCompletionRedirectParams<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomFieldOptionParam<'a> {
    /// The label for the option, displayed to the customer. Up to 100 characters.
    pub label: &'a str,
    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: &'a str,
}
impl<'a> CustomFieldOptionParam<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self { label, value }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomTextPositionParam<'a> {
    /// Text may be up to 1200 characters in length.
    pub message: &'a str,
}
impl<'a> CustomTextPositionParam<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomFieldParams<'a> {
    /// The name of the custom field. This may be up to 40 characters.
    pub name: &'a str,
    /// The value of the custom field. This may be up to 140 characters.
    pub value: &'a str,
}
impl<'a> CustomFieldParams<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AdjustableQuantityParams {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,
    /// The maximum quantity the customer can purchase.
    /// By default this value is 99.
    /// You can specify a value up to 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity the customer can purchase.
    /// By default this value is 0.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
impl AdjustableQuantityParams {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, maximum: None, minimum: None }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CompletedSessionsParams {
    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}
impl CompletedSessionsParams {
    pub fn new(limit: i64) -> Self {
        Self { limit }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomFieldDropdownParam<'a> {
    /// The options available for the customer to select. Up to 200 options allowed.
    pub options: &'a [CustomFieldOptionParam<'a>],
}
impl<'a> CustomFieldDropdownParam<'a> {
    pub fn new(options: &'a [CustomFieldOptionParam<'a>]) -> Self {
        Self { options }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomTextParam<'a> {
    /// Custom text that should be displayed after the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<CustomTextPositionParam<'a>>,
    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CustomTextPositionParam<'a>>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CustomTextPositionParam<'a>>,
    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<CustomTextPositionParam<'a>>,
}
impl<'a> CustomTextParam<'a> {
    pub fn new() -> Self {
        Self {
            after_submit: None,
            shipping_address: None,
            submit: None,
            terms_of_service_acceptance: None,
        }
    }
}
impl<'a> Default for CustomTextParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RestrictionsParams {
    /// Configuration for the `completed_sessions` restriction type.
    pub completed_sessions: CompletedSessionsParams,
}
impl RestrictionsParams {
    pub fn new(completed_sessions: CompletedSessionsParams) -> Self {
        Self { completed_sessions }
    }
}
