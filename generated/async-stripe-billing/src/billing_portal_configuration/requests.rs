use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListBillingPortalConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListBillingPortalConfigurationBuilder").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListBillingPortalConfiguration {
    inner: ListBillingPortalConfigurationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListBillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListBillingPortalConfiguration").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveBillingPortalConfigurationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveBillingPortalConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveBillingPortalConfigurationBuilder").finish_non_exhaustive()
    }
}
impl RetrieveBillingPortalConfigurationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a configuration that describes the functionality of the customer portal.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveBillingPortalConfiguration {
    inner: RetrieveBillingPortalConfigurationBuilder,
    configuration: stripe_billing::BillingPortalConfigurationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveBillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveBillingPortalConfiguration").finish_non_exhaustive()
    }
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfigurationBuilder").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfigurationBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfigurationBusinessProfile").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfigurationFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfigurationFeatures").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesCustomerUpdate {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<stripe_shared::AllowedUpdates>>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfigurationFeaturesCustomerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfigurationFeaturesCustomerUpdate")
            .finish_non_exhaustive()
    }
}
impl CreateBillingPortalConfigurationFeaturesCustomerUpdate {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { allowed_updates: None, enabled: enabled.into() }
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionCancel {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<stripe_shared::PortalSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// Passing `always_invoice` will result in an error.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<stripe_shared::ProrationBehavior>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfigurationFeaturesSubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfigurationFeaturesSubscriptionCancel")
            .finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The IDs of custom feedback options to use for this cancellation reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_options: Option<Vec<String>>,
    /// Which cancellation reasons will be given as options to the customer.
    pub options: Vec<stripe_shared::CancellationDetailsFeedback>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason",
        )
        .finish_non_exhaustive()
    }
}
impl CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    pub fn new(
        enabled: impl Into<bool>,
        options: impl Into<Vec<stripe_shared::CancellationDetailsFeedback>>,
    ) -> Self {
        Self { enabled: enabled.into(), feedback_options: None, options: options.into() }
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    /// Determines the value to use for the billing cycle anchor on subscription updates.
    /// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<stripe_shared::BillingCycleAnchor>,
    /// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allowed_updates: Option<Vec<stripe_shared::DefaultAllowedUpdates>>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of up to 10 products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<SubscriptionUpdateProductParam>>,
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<stripe_shared::ProrationBehavior>,
    /// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_at_period_end:
        Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd>,
    /// The behavior when updating a subscription that is trialing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_update_behavior: Option<stripe_shared::TrialUpdateBehavior>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfigurationFeaturesSubscriptionUpdate")
            .finish_non_exhaustive()
    }
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
/// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    /// List of conditions.
    /// When any condition is true, the update will be scheduled at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ScheduleUpdateAtPeriodEndConditionParam>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd",
        )
        .finish_non_exhaustive()
    }
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
/// The hosted login page for this configuration.
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingPortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://docs.stripe.com/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfigurationLoginPage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfigurationLoginPage").finish_non_exhaustive()
    }
}
impl CreateBillingPortalConfigurationLoginPage {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Creates a configuration that describes the functionality and behavior of a PortalSession
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateBillingPortalConfiguration {
    inner: CreateBillingPortalConfigurationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateBillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateBillingPortalConfiguration").finish_non_exhaustive()
    }
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
    /// This can be [overridden](https://docs.stripe.com/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
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
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfigurationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfigurationBuilder").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfigurationBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfigurationBusinessProfile").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfigurationFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfigurationFeatures").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesCustomerUpdate {
    /// The types of customer updates that are supported. When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<stripe_shared::AllowedUpdates>>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfigurationFeaturesCustomerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfigurationFeaturesCustomerUpdate")
            .finish_non_exhaustive()
    }
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
/// Information about canceling subscriptions in the portal.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
    pub mode: Option<stripe_shared::PortalSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// Passing `always_invoice` will result in an error.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<stripe_shared::ProrationBehavior>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfigurationFeaturesSubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfigurationFeaturesSubscriptionCancel")
            .finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The IDs of custom feedback options to use for this cancellation reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_options: Option<Vec<String>>,
    /// Which cancellation reasons will be given as options to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<stripe_shared::CancellationDetailsFeedback>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason",
        )
        .finish_non_exhaustive()
    }
}
impl UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReason {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), feedback_options: None, options: None }
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    /// Determines the value to use for the billing cycle anchor on subscription updates.
    /// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<stripe_shared::BillingCycleAnchor>,
    /// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allowed_updates: Option<Vec<stripe_shared::DefaultAllowedUpdates>>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The list of up to 10 products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<SubscriptionUpdateProductParam>>,
    /// Determines how to handle prorations resulting from subscription updates.
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<stripe_shared::ProrationBehavior>,
    /// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_at_period_end:
        Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd>,
    /// The behavior when updating a subscription that is trialing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_update_behavior: Option<stripe_shared::TrialUpdateBehavior>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate")
            .finish_non_exhaustive()
    }
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
/// Setting to control when an update should be scheduled at the end of the period instead of applying immediately.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd {
    /// List of conditions.
    /// When any condition is true, the update will be scheduled at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ScheduleUpdateAtPeriodEndConditionParam>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEnd",
        )
        .finish_non_exhaustive()
    }
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
/// The hosted login page for this configuration.
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBillingPortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://docs.stripe.com/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    ///
    /// Set to `false` to deactivate the `login_page.url`.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfigurationLoginPage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfigurationLoginPage").finish_non_exhaustive()
    }
}
impl UpdateBillingPortalConfigurationLoginPage {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
/// Updates a configuration that describes the functionality of the customer portal.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateBillingPortalConfiguration {
    inner: UpdateBillingPortalConfigurationBuilder,
    configuration: stripe_billing::BillingPortalConfigurationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateBillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateBillingPortalConfiguration").finish_non_exhaustive()
    }
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
    /// This can be [overridden](https://docs.stripe.com/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
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

#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct InvoiceListParam {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceListParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceListParam").finish_non_exhaustive()
    }
}
impl InvoiceListParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentMethodUpdateParam {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The [Payment Method Configuration](/api/payment_method_configurations) to use for this portal session.
    /// When specified, customers will be able to update their payment method to one of the options specified by the payment method configuration.
    /// If not set or set to an empty string, the default payment method configuration is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodUpdateParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodUpdateParam").finish_non_exhaustive()
    }
}
impl PaymentMethodUpdateParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), payment_method_configuration: None }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionUpdateProductAdjustableQuantityParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionUpdateProductAdjustableQuantityParam").finish_non_exhaustive()
    }
}
impl SubscriptionUpdateProductAdjustableQuantityParam {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), maximum: None, minimum: None }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ScheduleUpdateAtPeriodEndConditionParam {
    /// The type of condition.
    #[serde(rename = "type")]
    pub type_: stripe_shared::PortalResourceScheduleUpdateAtPeriodEndConditionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ScheduleUpdateAtPeriodEndConditionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ScheduleUpdateAtPeriodEndConditionParam").finish_non_exhaustive()
    }
}
impl ScheduleUpdateAtPeriodEndConditionParam {
    pub fn new(
        type_: impl Into<stripe_shared::PortalResourceScheduleUpdateAtPeriodEndConditionType>,
    ) -> Self {
        Self { type_: type_.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SubscriptionUpdateProductParam {
    /// Control whether the quantity of the product can be adjusted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<SubscriptionUpdateProductAdjustableQuantityParam>,
    /// The list of price IDs for the product that a subscription can be updated to.
    pub prices: Vec<String>,
    /// The product id.
    pub product: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionUpdateProductParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionUpdateProductParam").finish_non_exhaustive()
    }
}
impl SubscriptionUpdateProductParam {
    pub fn new(prices: impl Into<Vec<String>>, product: impl Into<String>) -> Self {
        Self { adjustable_quantity: None, prices: prices.into(), product: product.into() }
    }
}
/// The types of customer updates that are supported. When empty, customers are not updateable.
pub type CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates =
    stripe_shared::AllowedUpdates;
/// Which cancellation reasons will be given as options to the customer.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions =
    stripe_shared::CancellationDetailsFeedback;
/// Whether to cancel subscriptions immediately or at the end of the billing period.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode =
    stripe_shared::PortalSubscriptionCancelMode;
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// Passing `always_invoice` will result in an error.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior =
    stripe_shared::ProrationBehavior;
/// Determines the value to use for the billing cycle anchor on subscription updates.
/// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor =
    stripe_shared::BillingCycleAnchor;
/// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates =
    stripe_shared::DefaultAllowedUpdates;
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior =
    stripe_shared::ProrationBehavior;
/// The type of condition.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType =
    stripe_shared::PortalResourceScheduleUpdateAtPeriodEndConditionType;
/// The behavior when updating a subscription that is trialing.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior =
    stripe_shared::TrialUpdateBehavior;
/// The types of customer updates that are supported. When empty, customers are not updateable.
pub type UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates =
    stripe_shared::AllowedUpdates;
/// Which cancellation reasons will be given as options to the customer.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionCancelCancellationReasonOptions =
    stripe_shared::CancellationDetailsFeedback;
/// Whether to cancel subscriptions immediately or at the end of the billing period.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode =
    stripe_shared::PortalSubscriptionCancelMode;
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// Passing `always_invoice` will result in an error.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior =
    stripe_shared::ProrationBehavior;
/// Determines the value to use for the billing cycle anchor on subscription updates.
/// Valid values are `now` or `unchanged`, and the default value is `unchanged`.
/// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
/// For more information, see the billing cycle [documentation](https://docs.stripe.com/billing/subscriptions/billing-cycle).
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateBillingCycleAnchor =
    stripe_shared::BillingCycleAnchor;
/// The types of subscription updates that are supported. When empty, subscriptions are not updateable.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates =
    stripe_shared::DefaultAllowedUpdates;
/// Determines how to handle prorations resulting from subscription updates.
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior =
    stripe_shared::ProrationBehavior;
/// The type of condition.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsType =
    stripe_shared::PortalResourceScheduleUpdateAtPeriodEndConditionType;
/// The behavior when updating a subscription that is trialing.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateTrialUpdateBehavior =
    stripe_shared::TrialUpdateBehavior;
/// Information about showing the billing history in the portal.
pub type CreateBillingPortalConfigurationFeaturesInvoiceHistory = InvoiceListParam;
/// Information about updating payment methods in the portal.
pub type CreateBillingPortalConfigurationFeaturesPaymentMethodUpdate = PaymentMethodUpdateParam;
/// Control whether the quantity of the product can be adjusted.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProductsAdjustableQuantity =
    SubscriptionUpdateProductAdjustableQuantityParam;
/// List of conditions.
/// When any condition is true, the update will be scheduled at the end of the current period.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions =
    ScheduleUpdateAtPeriodEndConditionParam;
/// Information about showing the billing history in the portal.
pub type UpdateBillingPortalConfigurationFeaturesInvoiceHistory = InvoiceListParam;
/// Information about updating payment methods in the portal.
pub type UpdateBillingPortalConfigurationFeaturesPaymentMethodUpdate = PaymentMethodUpdateParam;
/// Control whether the quantity of the product can be adjusted.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProductsAdjustableQuantity =
    SubscriptionUpdateProductAdjustableQuantityParam;
/// List of conditions.
/// When any condition is true, the update will be scheduled at the end of the current period.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateScheduleAtPeriodEndConditions =
    ScheduleUpdateAtPeriodEndConditionParam;
/// The list of up to 10 products that support subscription updates.
pub type CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProducts =
    SubscriptionUpdateProductParam;
/// The list of up to 10 products that support subscription updates.
pub type UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProducts =
    SubscriptionUpdateProductParam;
