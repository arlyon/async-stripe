use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListBillingPortalConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListBillingPortalConfigurationBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            ending_before: None,
            expand: None,
            is_default: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of configurations that describe the functionality of the customer portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListBillingPortalConfiguration<'a> {
    inner: ListBillingPortalConfigurationBuilder<'a>,
}
impl<'a> ListBillingPortalConfiguration<'a> {
    /// Construct a new `ListBillingPortalConfiguration`.
    pub fn new() -> Self {
        Self { inner: ListBillingPortalConfigurationBuilder::new() }
    }
    /// Only return configurations that are active or inactive (e.g., pass `true` to only list active configurations).
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
    /// Only return the default or non-default configurations (e.g., pass `true` to only list the default configuration).
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.inner.is_default = Some(is_default);
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
impl<'a> Default for ListBillingPortalConfiguration<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingPortalConfiguration<'_> {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_billing::BillingPortalConfiguration>,
    > {
        stripe_client_core::ListPaginator::new_list("/billing_portal/configurations", self.inner)
    }
}

impl StripeRequest for ListBillingPortalConfiguration<'_> {
    type Output = stripe_types::List<stripe_billing::BillingPortalConfiguration>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing_portal/configurations").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveBillingPortalConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveBillingPortalConfigurationBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a configuration that describes the functionality of the customer portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveBillingPortalConfiguration<'a> {
    inner: RetrieveBillingPortalConfigurationBuilder<'a>,
    configuration: &'a stripe_billing::BillingPortalConfigurationId,
}
impl<'a> RetrieveBillingPortalConfiguration<'a> {
    /// Construct a new `RetrieveBillingPortalConfiguration`.
    pub fn new(configuration: &'a stripe_billing::BillingPortalConfigurationId) -> Self {
        Self { configuration, inner: RetrieveBillingPortalConfigurationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveBillingPortalConfiguration<'_> {
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

impl StripeRequest for RetrieveBillingPortalConfiguration<'_> {
    type Output = stripe_billing::BillingPortalConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = self.configuration;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/billing_portal/configurations/{configuration}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateBillingPortalConfigurationBuilder<'a> {
    business_profile: CreateBillingPortalConfigurationBusinessProfile<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    features: CreateBillingPortalConfigurationFeatures<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_page: Option<CreateBillingPortalConfigurationLoginPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateBillingPortalConfigurationBuilder<'a> {
    fn new(
        business_profile: CreateBillingPortalConfigurationBusinessProfile<'a>,
        features: CreateBillingPortalConfigurationFeatures<'a>,
    ) -> Self {
        Self {
            business_profile,
            default_return_url: None,
            expand: None,
            features,
            login_page: None,
            metadata: None,
        }
    }
}
/// The business information shown to customers in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationBusinessProfile<'a> {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<&'a str>,
    /// A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<&'a str>,
    /// A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<&'a str>,
}
impl<'a> CreateBillingPortalConfigurationBusinessProfile<'a> {
    pub fn new() -> Self {
        Self { headline: None, privacy_policy_url: None, terms_of_service_url: None }
    }
}
impl<'a> Default for CreateBillingPortalConfigurationBusinessProfile<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeatures<'a> {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreateBillingPortalConfigurationFeaturesCustomerUpdate<'a>>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<CreateBillingPortalConfigurationFeaturesInvoiceHistory>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<CreateBillingPortalConfigurationFeaturesPaymentMethodUpdate>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancel<'a>>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> CreateBillingPortalConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self {
            customer_update: None,
            invoice_history: None,
            payment_method_update: None,
            subscription_cancel: None,
            subscription_update: None,
        }
    }
}
impl<'a> Default for CreateBillingPortalConfigurationFeatures<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesCustomerUpdate<'a> {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates:
        Option<&'a [CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates]>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl<'a> CreateBillingPortalConfigurationFeaturesCustomerUpdate<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { allowed_updates: None, enabled }
    }
}
/// The types of customer updates that are supported. When empty, customers are not updateable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
}
impl CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match self {
            Address => "address",
            Email => "email",
            Name => "name",
            Phone => "phone",
            Shipping => "shipping",
            TaxId => "tax_id",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates"))
    }
}
/// Information about showing the billing history in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl CreateBillingPortalConfigurationFeaturesInvoiceHistory {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about updating payment methods in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl CreateBillingPortalConfigurationFeaturesPaymentMethodUpdate {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionCancel<'a> {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a>>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior>,
}
impl<'a> CreateBillingPortalConfigurationFeaturesSubscriptionCancel<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { cancellation_reason: None, enabled, mode: None, proration_behavior: None }
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    pub options:
        &'a [CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions],
}
impl<'a> CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    pub fn new(
        enabled: bool,
        options: &'a [CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions],
    ) -> Self {
        Self { enabled, options }
    }
}
/// Which cancellation reasons will be given as options to the customer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions::*;
        match self {
            CustomerService => "customer_service",
            LowQuality => "low_quality",
            MissingFeatures => "missing_features",
            Other => "other",
            SwitchedService => "switched_service",
            TooComplex => "too_complex",
            TooExpensive => "too_expensive",
            Unused => "unused",
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions::*;
        match s {
            "customer_service" => Ok(CustomerService),
            "low_quality" => Ok(LowQuality),
            "missing_features" => Ok(MissingFeatures),
            "other" => Ok(Other),
            "switched_service" => Ok(SwitchedService),
            "too_complex" => Ok(TooComplex),
            "too_expensive" => Ok(TooExpensive),
            "unused" => Ok(Unused),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
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
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions"))
    }
}
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match self {
            AtPeriodEnd => "at_period_end",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode",
            )
        })
    }
}
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
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
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior"))
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a> {
    /// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
    pub default_allowed_updates:
        &'a [CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates],
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of up to 10 products that support subscription updates.
    pub products: &'a [SubscriptionUpdateProductParam<'a>],
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior>,
}
impl<'a> CreateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a> {
    pub fn new(
        default_allowed_updates: &'a [CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates],
        enabled: bool,
        products: &'a [SubscriptionUpdateProductParam<'a>],
    ) -> Self {
        Self { default_allowed_updates, enabled, products, proration_behavior: None }
    }
}
/// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match self {
            Price => "price",
            PromotionCode => "promotion_code",
            Quantity => "quantity",
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
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
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates"))
    }
}
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
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
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior"))
    }
}
/// The hosted login page for this configuration.
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    pub enabled: bool,
}
impl CreateBillingPortalConfigurationLoginPage {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Creates a configuration that describes the functionality and behavior of a PortalSession
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfiguration<'a> {
    inner: CreateBillingPortalConfigurationBuilder<'a>,
}
impl<'a> CreateBillingPortalConfiguration<'a> {
    /// Construct a new `CreateBillingPortalConfiguration`.
    pub fn new(
        business_profile: CreateBillingPortalConfigurationBusinessProfile<'a>,
        features: CreateBillingPortalConfigurationFeatures<'a>,
    ) -> Self {
        Self { inner: CreateBillingPortalConfigurationBuilder::new(business_profile, features) }
    }
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub fn default_return_url(mut self, default_return_url: &'a str) -> Self {
        self.inner.default_return_url = Some(default_return_url);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The hosted login page for this configuration.
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    pub fn login_page(mut self, login_page: CreateBillingPortalConfigurationLoginPage) -> Self {
        self.inner.login_page = Some(login_page);
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
}
impl CreateBillingPortalConfiguration<'_> {
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

impl StripeRequest for CreateBillingPortalConfiguration<'_> {
    type Output = stripe_billing::BillingPortalConfiguration;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing_portal/configurations").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateBillingPortalConfigurationBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile: Option<UpdateBillingPortalConfigurationBusinessProfile<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<UpdateBillingPortalConfigurationFeatures<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_page: Option<UpdateBillingPortalConfigurationLoginPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateBillingPortalConfigurationBuilder<'a> {
    fn new() -> Self {
        Self {
            active: None,
            business_profile: None,
            default_return_url: None,
            expand: None,
            features: None,
            login_page: None,
            metadata: None,
        }
    }
}
/// The business information shown to customers in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationBusinessProfile<'a> {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<&'a str>,
    /// A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<&'a str>,
    /// A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<&'a str>,
}
impl<'a> UpdateBillingPortalConfigurationBusinessProfile<'a> {
    pub fn new() -> Self {
        Self { headline: None, privacy_policy_url: None, terms_of_service_url: None }
    }
}
impl<'a> Default for UpdateBillingPortalConfigurationBusinessProfile<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeatures<'a> {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<UpdateBillingPortalConfigurationFeaturesCustomerUpdate<'a>>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<UpdateBillingPortalConfigurationFeaturesInvoiceHistory>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<UpdateBillingPortalConfigurationFeaturesPaymentMethodUpdate>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancel<'a>>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> UpdateBillingPortalConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self {
            customer_update: None,
            invoice_history: None,
            payment_method_update: None,
            subscription_cancel: None,
            subscription_update: None,
        }
    }
}
impl<'a> Default for UpdateBillingPortalConfigurationFeatures<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesCustomerUpdate<'a> {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates:
        Option<&'a [UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates]>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl<'a> UpdateBillingPortalConfigurationFeaturesCustomerUpdate<'a> {
    pub fn new() -> Self {
        Self { allowed_updates: None, enabled: None }
    }
}
impl<'a> Default for UpdateBillingPortalConfigurationFeaturesCustomerUpdate<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The types of customer updates that are supported. When empty, customers are not updateable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
}
impl UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match self {
            Address => "address",
            Email => "email",
            Name => "name",
            Phone => "phone",
            Shipping => "shipping",
            TaxId => "tax_id",
        }
    }
}

impl std::str::FromStr for UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates"))
    }
}
/// Information about showing the billing history in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl UpdateBillingPortalConfigurationFeaturesInvoiceHistory {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about updating payment methods in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl UpdateBillingPortalConfigurationFeaturesPaymentMethodUpdate {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionCancel<'a> {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a>>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior>,
}
impl<'a> UpdateBillingPortalConfigurationFeaturesSubscriptionCancel<'a> {
    pub fn new() -> Self {
        Self { cancellation_reason: None, enabled: None, mode: None, proration_behavior: None }
    }
}
impl<'a> Default for UpdateBillingPortalConfigurationFeaturesSubscriptionCancel<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<
        &'a [UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions],
    >,
}
impl<'a> UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, options: None }
    }
}
/// Which cancellation reasons will be given as options to the customer.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions::*;
        match self {
            CustomerService => "customer_service",
            LowQuality => "low_quality",
            MissingFeatures => "missing_features",
            Other => "other",
            SwitchedService => "switched_service",
            TooComplex => "too_complex",
            TooExpensive => "too_expensive",
            Unused => "unused",
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions::*;
        match s {
            "customer_service" => Ok(CustomerService),
            "low_quality" => Ok(LowQuality),
            "missing_features" => Ok(MissingFeatures),
            "other" => Ok(Other),
            "switched_service" => Ok(SwitchedService),
            "too_complex" => Ok(TooComplex),
            "too_expensive" => Ok(TooExpensive),
            "unused" => Ok(Unused),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
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
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions"))
    }
}
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match self {
            AtPeriodEnd => "at_period_end",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode",
            )
        })
    }
}
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
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
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior"))
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a> {
    /// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allowed_updates: Option<
        &'a [UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates],
    >,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The list of up to 10 products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<&'a [SubscriptionUpdateProductParam<'a>]>,
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior>,
}
impl<'a> UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a> {
    pub fn new() -> Self {
        Self {
            default_allowed_updates: None,
            enabled: None,
            products: None,
            proration_behavior: None,
        }
    }
}
impl<'a> Default for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a> {
    fn default() -> Self {
        Self::new()
    }
}
/// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match self {
            Price => "price",
            PromotionCode => "promotion_code",
            Quantity => "quantity",
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
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
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates"))
    }
}
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
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
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior"))
    }
}
/// The hosted login page for this configuration.
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    ///
    /// Set to `false` to deactivate the `login_page.url`.
    pub enabled: bool,
}
impl UpdateBillingPortalConfigurationLoginPage {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Updates a configuration that describes the functionality of the customer portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfiguration<'a> {
    inner: UpdateBillingPortalConfigurationBuilder<'a>,
    configuration: &'a stripe_billing::BillingPortalConfigurationId,
}
impl<'a> UpdateBillingPortalConfiguration<'a> {
    /// Construct a new `UpdateBillingPortalConfiguration`.
    pub fn new(configuration: &'a stripe_billing::BillingPortalConfigurationId) -> Self {
        Self { configuration, inner: UpdateBillingPortalConfigurationBuilder::new() }
    }
    /// Whether the configuration is active and can be used to create portal sessions.
    pub fn active(mut self, active: bool) -> Self {
        self.inner.active = Some(active);
        self
    }
    /// The business information shown to customers in the portal.
    pub fn business_profile(
        mut self,
        business_profile: UpdateBillingPortalConfigurationBusinessProfile<'a>,
    ) -> Self {
        self.inner.business_profile = Some(business_profile);
        self
    }
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub fn default_return_url(mut self, default_return_url: &'a str) -> Self {
        self.inner.default_return_url = Some(default_return_url);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Information about the features available in the portal.
    pub fn features(mut self, features: UpdateBillingPortalConfigurationFeatures<'a>) -> Self {
        self.inner.features = Some(features);
        self
    }
    /// The hosted login page for this configuration.
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    pub fn login_page(mut self, login_page: UpdateBillingPortalConfigurationLoginPage) -> Self {
        self.inner.login_page = Some(login_page);
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
}
impl UpdateBillingPortalConfiguration<'_> {
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

impl StripeRequest for UpdateBillingPortalConfiguration<'_> {
    type Output = stripe_billing::BillingPortalConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = self.configuration;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/billing_portal/configurations/{configuration}"),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SubscriptionUpdateProductParam<'a> {
    /// The list of price IDs for the product that a subscription can be updated to.
    pub prices: &'a [&'a str],
    /// The product id.
    pub product: &'a str,
}
impl<'a> SubscriptionUpdateProductParam<'a> {
    pub fn new(prices: &'a [&'a str], product: &'a str) -> Self {
        Self { prices, product }
    }
}
