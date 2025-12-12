use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListBillingPortalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListBillingPortalConfigurationBuilder {
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
pub struct ListBillingPortalConfiguration {
    inner: ListBillingPortalConfigurationBuilder,
}
impl ListBillingPortalConfiguration {
    /// Construct a new `ListBillingPortalConfiguration`.
    pub fn new() -> Self {
        Self { inner: ListBillingPortalConfigurationBuilder::new() }
    }
    /// Only return configurations that are active or inactive (e.g., pass `true` to only list active configurations).
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
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
    /// Only return the default or non-default configurations (e.g., pass `true` to only list the default configuration).
    pub fn is_default(mut self, is_default: impl Into<bool>) -> Self {
        self.inner.is_default = Some(is_default.into());
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
impl Default for ListBillingPortalConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingPortalConfiguration {
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
        stripe_client_core::ListPaginator::new_list("/billing_portal/configurations", &self.inner)
    }
}

impl StripeRequest for ListBillingPortalConfiguration {
    type Output = stripe_types::List<stripe_billing::BillingPortalConfiguration>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing_portal/configurations").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveBillingPortalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveBillingPortalConfigurationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a configuration that describes the functionality of the customer portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveBillingPortalConfiguration {
    inner: RetrieveBillingPortalConfigurationBuilder,
    configuration: stripe_billing::BillingPortalConfigurationId,
}
impl RetrieveBillingPortalConfiguration {
    /// Construct a new `RetrieveBillingPortalConfiguration`.
    pub fn new(configuration: impl Into<stripe_billing::BillingPortalConfigurationId>) -> Self {
        Self {
            configuration: configuration.into(),
            inner: RetrieveBillingPortalConfigurationBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveBillingPortalConfiguration {
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

impl StripeRequest for RetrieveBillingPortalConfiguration {
    type Output = stripe_billing::BillingPortalConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = &self.configuration;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/billing_portal/configurations/{configuration}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateBillingPortalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile: Option<CreateBillingPortalConfigurationBusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    features: CreateBillingPortalConfigurationFeatures,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_page: Option<CreateBillingPortalConfigurationLoginPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
impl CreateBillingPortalConfigurationBuilder {
    fn new(features: impl Into<CreateBillingPortalConfigurationFeatures>) -> Self {
        Self {
            business_profile: None,
            default_return_url: None,
            expand: None,
            features: features.into(),
            login_page: None,
            metadata: None,
            name: None,
        }
    }
}
/// The business information shown to customers in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationBusinessProfile {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    /// A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
}
impl CreateBillingPortalConfigurationBusinessProfile {
    pub fn new() -> Self {
        Self { headline: None, privacy_policy_url: None, terms_of_service_url: None }
    }
}
impl Default for CreateBillingPortalConfigurationBusinessProfile {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the features available in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeatures {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreateBillingPortalConfigurationFeaturesCustomerUpdate>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<InvoiceListParam>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<PaymentMethodUpdateParam>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancel>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdate>,
}
impl CreateBillingPortalConfigurationFeatures {
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
impl Default for CreateBillingPortalConfigurationFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesCustomerUpdate {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates:
        Option<Vec<CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates>>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl CreateBillingPortalConfigurationFeaturesCustomerUpdate {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { allowed_updates: None, enabled: enabled.into() }
    }
}
/// The types of customer updates that are supported. When empty, customers are not updateable.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match self {
            Address => "address",
            Email => "email",
            Name => "name",
            Phone => "phone",
            Shipping => "shipping",
            TaxId => "tax_id",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionCancel {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// Passing `always_invoice` will result in an error.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior>,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancel {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            cancellation_reason: None,
            enabled: enabled.into(),
            mode: None,
            proration_behavior: None,
        }
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    pub options:
        Vec<CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions>,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    pub fn new(
        enabled: impl Into<bool>,
        options: impl Into<
            Vec<
                CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions,
            >,
        >,
    ) -> Self {
        Self { enabled: enabled.into(), options: options.into() }
    }
}
/// Which cancellation reasons will be given as options to the customer.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match self {
            AtPeriodEnd => "at_period_end",
            Immediately => "immediately",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// Passing `always_invoice` will result in an error.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    /// Determines the value to use for the billing cycle anchor on subscription updates.
    /// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor>,
    /// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allowed_updates: Option<
        Vec<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates>,
    >,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of up to 10 products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<SubscriptionUpdateProductParam>>,
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior>,
    /// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_at_period_end:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd>,
    /// The behavior when updating a subscription that is trialing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_update_behavior:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior>,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            billing_cycle_anchor: None,
            default_allowed_updates: None,
            enabled: enabled.into(),
            products: None,
            proration_behavior: None,
            schedule_at_period_end: None,
            trial_update_behavior: None,
        }
    }
}
/// Determines the value to use for the billing cycle anchor on subscription updates.
/// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor {
    Now,
    Unchanged,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor::*;
        match self {
            Now => "now",
            Unchanged => "unchanged",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor::*;
        match s {
            "now" => Ok(Now),
            "unchanged" => Ok(Unchanged),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
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
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match self {
            Price => "price",
            PromotionCode => "promotion_code",
            Quantity => "quantity",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    /// List of conditions.
    /// When any condition is true, the update will be scheduled at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        Vec<
            CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions,
        >,
    >,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    pub fn new() -> Self {
        Self { conditions: None }
    }
}
impl Default for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    fn default() -> Self {
        Self::new()
    }
}
/// List of conditions.
/// When any condition is true, the update will be scheduled at the end of the current period.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions {
    /// The type of condition.
    #[serde(rename = "type")]
    pub type_:
        CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType,
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions {
    pub fn new(
        type_: impl Into<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType>,
    ) -> Self {
        Self { type_: type_.into() }
    }
}
/// The type of condition.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    DecreasingItemAmount,
    ShorteningInterval,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType::*;
        match self {
            DecreasingItemAmount => "decreasing_item_amount",
            ShorteningInterval => "shortening_interval",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType::*;
        match s {
            "decreasing_item_amount" => Ok(DecreasingItemAmount),
            "shortening_interval" => Ok(ShorteningInterval),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
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
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The behavior when updating a subscription that is trialing.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior {
    ContinueTrial,
    EndTrial,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior {
    pub fn as_str(&self) -> &str {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior::*;
        match self {
            ContinueTrial => "continue_trial",
            EndTrial => "end_trial",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior::*;
        match s {
            "continue_trial" => Ok(ContinueTrial),
            "end_trial" => Ok(EndTrial),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
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
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The hosted login page for this configuration.
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://docs.stripe.com/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    pub enabled: bool,
}
impl CreateBillingPortalConfigurationLoginPage {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Creates a configuration that describes the functionality and behavior of a PortalSession
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfiguration {
    inner: CreateBillingPortalConfigurationBuilder,
}
impl CreateBillingPortalConfiguration {
    /// Construct a new `CreateBillingPortalConfiguration`.
    pub fn new(features: impl Into<CreateBillingPortalConfigurationFeatures>) -> Self {
        Self { inner: CreateBillingPortalConfigurationBuilder::new(features.into()) }
    }
    /// The business information shown to customers in the portal.
    pub fn business_profile(
        mut self,
        business_profile: impl Into<CreateBillingPortalConfigurationBusinessProfile>,
    ) -> Self {
        self.inner.business_profile = Some(business_profile.into());
        self
    }
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://docs.stripe.com/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub fn default_return_url(mut self, default_return_url: impl Into<String>) -> Self {
        self.inner.default_return_url = Some(default_return_url.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The hosted login page for this configuration.
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    pub fn login_page(
        mut self,
        login_page: impl Into<CreateBillingPortalConfigurationLoginPage>,
    ) -> Self {
        self.inner.login_page = Some(login_page.into());
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
    /// The name of the configuration.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
}
impl CreateBillingPortalConfiguration {
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

impl StripeRequest for CreateBillingPortalConfiguration {
    type Output = stripe_billing::BillingPortalConfiguration;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing_portal/configurations").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateBillingPortalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile: Option<UpdateBillingPortalConfigurationBusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<UpdateBillingPortalConfigurationFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_page: Option<UpdateBillingPortalConfigurationLoginPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
impl UpdateBillingPortalConfigurationBuilder {
    fn new() -> Self {
        Self {
            active: None,
            business_profile: None,
            default_return_url: None,
            expand: None,
            features: None,
            login_page: None,
            metadata: None,
            name: None,
        }
    }
}
/// The business information shown to customers in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationBusinessProfile {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    /// A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
}
impl UpdateBillingPortalConfigurationBusinessProfile {
    pub fn new() -> Self {
        Self { headline: None, privacy_policy_url: None, terms_of_service_url: None }
    }
}
impl Default for UpdateBillingPortalConfigurationBusinessProfile {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the features available in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeatures {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<UpdateBillingPortalConfigurationFeaturesCustomerUpdate>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<InvoiceListParam>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<PaymentMethodUpdateParam>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancel>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate>,
}
impl UpdateBillingPortalConfigurationFeatures {
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
impl Default for UpdateBillingPortalConfigurationFeatures {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesCustomerUpdate {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates:
        Option<Vec<UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates>>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl UpdateBillingPortalConfigurationFeaturesCustomerUpdate {
    pub fn new() -> Self {
        Self { allowed_updates: None, enabled: None }
    }
}
impl Default for UpdateBillingPortalConfigurationFeaturesCustomerUpdate {
    fn default() -> Self {
        Self::new()
    }
}
/// The types of customer updates that are supported. When empty, customers are not updateable.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match self {
            Address => "address",
            Email => "email",
            Name => "name",
            Phone => "phone",
            Shipping => "shipping",
            TaxId => "tax_id",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionCancel {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// Passing `always_invoice` will result in an error.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior>,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancel {
    pub fn new() -> Self {
        Self { cancellation_reason: None, enabled: None, mode: None, proration_behavior: None }
    }
}
impl Default for UpdateBillingPortalConfigurationFeaturesSubscriptionCancel {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<
        Vec<UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions>,
    >,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), options: None }
    }
}
/// Which cancellation reasons will be given as options to the customer.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions
{
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match self {
            AtPeriodEnd => "at_period_end",
            Immediately => "immediately",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// Passing `always_invoice` will result in an error.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    /// Determines the value to use for the billing cycle anchor on subscription updates.
    /// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor>,
    /// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allowed_updates: Option<
        Vec<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates>,
    >,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The list of up to 10 products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<SubscriptionUpdateProductParam>>,
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior>,
    /// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_at_period_end:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd>,
    /// The behavior when updating a subscription that is trialing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_update_behavior:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior>,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    pub fn new() -> Self {
        Self {
            billing_cycle_anchor: None,
            default_allowed_updates: None,
            enabled: None,
            products: None,
            proration_behavior: None,
            schedule_at_period_end: None,
            trial_update_behavior: None,
        }
    }
}
impl Default for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    fn default() -> Self {
        Self::new()
    }
}
/// Determines the value to use for the billing cycle anchor on subscription updates.
/// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor {
    Now,
    Unchanged,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor::*;
        match self {
            Now => "now",
            Unchanged => "unchanged",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor::*;
        match s {
            "now" => Ok(Now),
            "unchanged" => Ok(Unchanged),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
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
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match self {
            Price => "price",
            PromotionCode => "promotion_code",
            Quantity => "quantity",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    /// List of conditions.
    /// When any condition is true, the update will be scheduled at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        Vec<
            UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions,
        >,
    >,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    pub fn new() -> Self {
        Self { conditions: None }
    }
}
impl Default for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    fn default() -> Self {
        Self::new()
    }
}
/// List of conditions.
/// When any condition is true, the update will be scheduled at the end of the current period.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions {
    /// The type of condition.
    #[serde(rename = "type")]
    pub type_:
        UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType,
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions {
    pub fn new(
        type_: impl Into<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType>,
    ) -> Self {
        Self { type_: type_.into() }
    }
}
/// The type of condition.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    DecreasingItemAmount,
    ShorteningInterval,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType::*;
        match self {
            DecreasingItemAmount => "decreasing_item_amount",
            ShorteningInterval => "shortening_interval",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType::*;
        match s {
            "decreasing_item_amount" => Ok(DecreasingItemAmount),
            "shortening_interval" => Ok(ShorteningInterval),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
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
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The behavior when updating a subscription that is trialing.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior {
    ContinueTrial,
    EndTrial,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior {
    pub fn as_str(&self) -> &str {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior::*;
        match self {
            ContinueTrial => "continue_trial",
            EndTrial => "end_trial",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior::*;
        match s {
            "continue_trial" => Ok(ContinueTrial),
            "end_trial" => Ok(EndTrial),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
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
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The hosted login page for this configuration.
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://docs.stripe.com/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    ///
    /// Set to `false` to deactivate the `login_page.url`.
    pub enabled: bool,
}
impl UpdateBillingPortalConfigurationLoginPage {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Updates a configuration that describes the functionality of the customer portal.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingPortalConfiguration {
    inner: UpdateBillingPortalConfigurationBuilder,
    configuration: stripe_billing::BillingPortalConfigurationId,
}
impl UpdateBillingPortalConfiguration {
    /// Construct a new `UpdateBillingPortalConfiguration`.
    pub fn new(configuration: impl Into<stripe_billing::BillingPortalConfigurationId>) -> Self {
        Self {
            configuration: configuration.into(),
            inner: UpdateBillingPortalConfigurationBuilder::new(),
        }
    }
    /// Whether the configuration is active and can be used to create portal sessions.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// The business information shown to customers in the portal.
    pub fn business_profile(
        mut self,
        business_profile: impl Into<UpdateBillingPortalConfigurationBusinessProfile>,
    ) -> Self {
        self.inner.business_profile = Some(business_profile.into());
        self
    }
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://docs.stripe.com/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub fn default_return_url(mut self, default_return_url: impl Into<String>) -> Self {
        self.inner.default_return_url = Some(default_return_url.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Information about the features available in the portal.
    pub fn features(
        mut self,
        features: impl Into<UpdateBillingPortalConfigurationFeatures>,
    ) -> Self {
        self.inner.features = Some(features.into());
        self
    }
    /// The hosted login page for this configuration.
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    pub fn login_page(
        mut self,
        login_page: impl Into<UpdateBillingPortalConfigurationLoginPage>,
    ) -> Self {
        self.inner.login_page = Some(login_page.into());
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
    /// The name of the configuration.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
}
impl UpdateBillingPortalConfiguration {
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

impl StripeRequest for UpdateBillingPortalConfiguration {
    type Output = stripe_billing::BillingPortalConfiguration;

    fn build(&self) -> RequestBuilder {
        let configuration = &self.configuration;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/billing_portal/configurations/{configuration}"),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct InvoiceListParam {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl InvoiceListParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct PaymentMethodUpdateParam {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The [Payment Method Configuration](/api/payment_method_configurations) to use for this portal session.
    /// When specified, customers will be able to update their payment method to one of the options specified by the payment method configuration.
    /// If not set or set to an empty string, the default payment method configuration is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<String>,
}
impl PaymentMethodUpdateParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), payment_method_configuration: None }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SubscriptionUpdateProductAdjustableQuantityParam {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    pub enabled: bool,
    /// The maximum quantity that can be set for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The minimum quantity that can be set for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}
impl SubscriptionUpdateProductAdjustableQuantityParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct SubscriptionUpdateProductParam {
    /// Control whether the quantity of the product can be adjusted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<SubscriptionUpdateProductAdjustableQuantityParam>,
    /// The list of price IDs for the product that a subscription can be updated to.
    pub prices: Vec<String>,
    /// The product id.
    pub product: String,
}
impl SubscriptionUpdateProductParam {
    pub fn new(prices: impl Into<Vec<String>>, product: impl Into<String>) -> Self {
        Self { adjustable_quantity: None, prices: prices.into(), product: product.into() }
    }
}
