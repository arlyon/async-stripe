use stripe::{Client, Response};

impl stripe_core::billing_portal::configuration::Configuration {
    /// Returns a list of configurations that describe the functionality of the customer portal.
    pub fn list(
        client: &Client,
        params: ListConfiguration,
    ) -> Response<stripe_types::List<stripe_core::billing_portal::configuration::Configuration>>
    {
        client.get_query("/billing_portal/configurations", params)
    }
    /// Creates a configuration that describes the functionality and behavior of a PortalSession.
    pub fn create(
        client: &Client,
        params: CreateConfiguration,
    ) -> Response<stripe_core::billing_portal::configuration::Configuration> {
        client.send_form("/billing_portal/configurations", params, http_types::Method::Post)
    }
    /// Updates a configuration that describes the functionality of the customer portal.
    pub fn update(
        client: &Client,
        configuration: &str,
        params: UpdateConfiguration,
    ) -> Response<stripe_core::billing_portal::configuration::Configuration> {
        client.send_form(
            &format!(
                "/billing_portal/configurations/{configuration}",
                configuration = configuration
            ),
            params,
            http_types::Method::Post,
        )
    }
    /// Retrieves a configuration that describes the functionality of the customer portal.
    pub fn retrieve(
        client: &Client,
        configuration: &str,
        params: RetrieveConfiguration,
    ) -> Response<stripe_core::billing_portal::configuration::Configuration> {
        client.get_query(
            &format!(
                "/billing_portal/configurations/{configuration}",
                configuration = configuration
            ),
            params,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListConfiguration<'a> {
    /// Only return configurations that are active or inactive (e.g., pass `true` to only list active configurations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Only return the default or non-default configurations (e.g., pass `true` to only list the default configuration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
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
    pub starting_after: Option<String>,
}
impl<'a> ListConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfiguration<'a> {
    /// The business information shown to customers in the portal.
    pub business_profile: CreateConfigurationBusinessProfile<'a>,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Information about the features available in the portal.
    pub features: CreateConfigurationFeatures<'a>,
    /// The hosted login page for this configuration.
    ///
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<CreateConfigurationLoginPage>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateConfiguration<'a> {
    pub fn new(
        business_profile: CreateConfigurationBusinessProfile<'a>,
        features: CreateConfigurationFeatures<'a>,
    ) -> Self {
        Self {
            business_profile,
            default_return_url: Default::default(),
            expand: Default::default(),
            features,
            login_page: Default::default(),
            metadata: Default::default(),
        }
    }
}
/// The business information shown to customers in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationBusinessProfile<'a> {
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
impl<'a> CreateConfigurationBusinessProfile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationFeatures<'a> {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreateConfigurationFeaturesCustomerUpdate<'a>>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<CreateConfigurationFeaturesInvoiceHistory>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<CreateConfigurationFeaturesPaymentMethodUpdate>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<CreateConfigurationFeaturesSubscriptionCancel<'a>>,
    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause: Option<CreateConfigurationFeaturesSubscriptionPause>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreateConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> CreateConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationFeaturesCustomerUpdate<'a> {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [CreateConfigurationFeaturesCustomerUpdateAllowedUpdates]>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl<'a> CreateConfigurationFeaturesCustomerUpdate<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { allowed_updates: Default::default(), enabled }
    }
}
/// The types of customer updates that are supported.
///
/// When empty, customers are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Phone,
    Shipping,
    TaxId,
}

impl CreateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Shipping => "shipping",
            Self::TaxId => "tax_id",
        }
    }
}

impl AsRef<str> for CreateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Information about showing the billing history in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl CreateConfigurationFeaturesInvoiceHistory {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about updating payment methods in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl CreateConfigurationFeaturesPaymentMethodUpdate {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationFeaturesSubscriptionCancel<'a> {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<CreateConfigurationFeaturesSubscriptionCancelCancellationReason<'a>>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CreateConfigurationFeaturesSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateConfigurationFeaturesSubscriptionCancelProrationBehavior>,
}
impl<'a> CreateConfigurationFeaturesSubscriptionCancel<'a> {
    pub fn new(enabled: bool) -> Self {
        Self {
            cancellation_reason: Default::default(),
            enabled,
            mode: Default::default(),
            proration_behavior: Default::default(),
        }
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    pub options: &'a [CreateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions],
}
impl<'a> CreateConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    pub fn new(
        enabled: bool,
        options: &'a [CreateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions],
    ) -> Self {
        Self { enabled, options }
    }
}
/// Which cancellation reasons will be given as options to the customer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl CreateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomerService => "customer_service",
            Self::LowQuality => "low_quality",
            Self::MissingFeatures => "missing_features",
            Self::Other => "other",
            Self::SwitchedService => "switched_service",
            Self::TooComplex => "too_complex",
            Self::TooExpensive => "too_expensive",
            Self::Unused => "unused",
        }
    }
}

impl AsRef<str> for CreateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateConfigurationFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

impl CreateConfigurationFeaturesSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtPeriodEnd => "at_period_end",
            Self::Immediately => "immediately",
        }
    }
}

impl AsRef<str> for CreateConfigurationFeaturesSubscriptionCancelMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateConfigurationFeaturesSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Whether to create prorations when canceling subscriptions.
///
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl CreateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Information about pausing subscriptions in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateConfigurationFeaturesSubscriptionPause {
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl CreateConfigurationFeaturesSubscriptionPause {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationFeaturesSubscriptionUpdate<'a> {
    /// The types of subscription updates that are supported.
    ///
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates:
        &'a [CreateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates],
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of products that support subscription updates.
    pub products: &'a [CreateConfigurationFeaturesSubscriptionUpdateProducts<'a>],
    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateConfigurationFeaturesSubscriptionUpdateProrationBehavior>,
}
impl<'a> CreateConfigurationFeaturesSubscriptionUpdate<'a> {
    pub fn new(
        default_allowed_updates: &'a [CreateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates],
        enabled: bool,
        products: &'a [CreateConfigurationFeaturesSubscriptionUpdateProducts<'a>],
    ) -> Self {
        Self { default_allowed_updates, enabled, products, proration_behavior: Default::default() }
    }
}
/// The types of subscription updates that are supported.
///
/// When empty, subscriptions are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl CreateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Price => "price",
            Self::PromotionCode => "promotion_code",
            Self::Quantity => "quantity",
        }
    }
}

impl AsRef<str> for CreateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of products that support subscription updates.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationFeaturesSubscriptionUpdateProducts<'a> {
    /// The list of price IDs for the product that a subscription can be updated to.
    pub prices: &'a [&'a str],
    /// The product id.
    pub product: &'a str,
}
impl<'a> CreateConfigurationFeaturesSubscriptionUpdateProducts<'a> {
    pub fn new(prices: &'a [&'a str], product: &'a str) -> Self {
        Self { prices, product }
    }
}
/// Determines how to handle prorations resulting from subscription updates.
///
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl CreateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CreateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The hosted login page for this configuration.
///
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    pub enabled: bool,
}
impl CreateConfigurationLoginPage {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfiguration<'a> {
    /// Whether the configuration is active and can be used to create portal sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The business information shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<UpdateConfigurationBusinessProfile<'a>>,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Information about the features available in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<UpdateConfigurationFeatures<'a>>,
    /// The hosted login page for this configuration.
    ///
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<UpdateConfigurationLoginPage>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The business information shown to customers in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationBusinessProfile<'a> {
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
impl<'a> UpdateConfigurationBusinessProfile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationFeatures<'a> {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<UpdateConfigurationFeaturesCustomerUpdate<'a>>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<UpdateConfigurationFeaturesInvoiceHistory>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<UpdateConfigurationFeaturesPaymentMethodUpdate>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<UpdateConfigurationFeaturesSubscriptionCancel<'a>>,
    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause: Option<UpdateConfigurationFeaturesSubscriptionPause>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<UpdateConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> UpdateConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationFeaturesCustomerUpdate<'a> {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [UpdateConfigurationFeaturesCustomerUpdateAllowedUpdates]>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl<'a> UpdateConfigurationFeaturesCustomerUpdate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The types of customer updates that are supported.
///
/// When empty, customers are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Phone,
    Shipping,
    TaxId,
}

impl UpdateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Shipping => "shipping",
            Self::TaxId => "tax_id",
        }
    }
}

impl AsRef<str> for UpdateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateConfigurationFeaturesCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Information about showing the billing history in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateConfigurationFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl UpdateConfigurationFeaturesInvoiceHistory {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about updating payment methods in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateConfigurationFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl UpdateConfigurationFeaturesPaymentMethodUpdate {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationFeaturesSubscriptionCancel<'a> {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<UpdateConfigurationFeaturesSubscriptionCancelCancellationReason<'a>>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<UpdateConfigurationFeaturesSubscriptionCancelMode>,
    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateConfigurationFeaturesSubscriptionCancelProrationBehavior>,
}
impl<'a> UpdateConfigurationFeaturesSubscriptionCancel<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options:
        Option<&'a [UpdateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions]>,
}
impl<'a> UpdateConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, options: Default::default() }
    }
}
/// Which cancellation reasons will be given as options to the customer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl UpdateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomerService => "customer_service",
            Self::LowQuality => "low_quality",
            Self::MissingFeatures => "missing_features",
            Self::Other => "other",
            Self::SwitchedService => "switched_service",
            Self::TooComplex => "too_complex",
            Self::TooExpensive => "too_expensive",
            Self::Unused => "unused",
        }
    }
}

impl AsRef<str> for UpdateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateConfigurationFeaturesSubscriptionCancelCancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateConfigurationFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

impl UpdateConfigurationFeaturesSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtPeriodEnd => "at_period_end",
            Self::Immediately => "immediately",
        }
    }
}

impl AsRef<str> for UpdateConfigurationFeaturesSubscriptionCancelMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateConfigurationFeaturesSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Whether to create prorations when canceling subscriptions.
///
/// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
/// No prorations are generated when canceling a subscription at the end of its natural billing period.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl UpdateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateConfigurationFeaturesSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Information about pausing subscriptions in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationFeaturesSubscriptionPause {
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl UpdateConfigurationFeaturesSubscriptionPause {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateConfigurationFeaturesSubscriptionUpdate<'a> {
    /// The types of subscription updates that are supported.
    ///
    /// When empty, subscriptions are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allowed_updates:
        Option<&'a [UpdateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates]>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The list of products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<&'a [UpdateConfigurationFeaturesSubscriptionUpdateProducts<'a>]>,
    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateConfigurationFeaturesSubscriptionUpdateProrationBehavior>,
}
impl<'a> UpdateConfigurationFeaturesSubscriptionUpdate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The types of subscription updates that are supported.
///
/// When empty, subscriptions are not updateable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl UpdateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Price => "price",
            Self::PromotionCode => "promotion_code",
            Self::Quantity => "quantity",
        }
    }
}

impl AsRef<str> for UpdateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The list of products that support subscription updates.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateConfigurationFeaturesSubscriptionUpdateProducts<'a> {
    /// The list of price IDs for the product that a subscription can be updated to.
    pub prices: &'a [&'a str],
    /// The product id.
    pub product: &'a str,
}
impl<'a> UpdateConfigurationFeaturesSubscriptionUpdateProducts<'a> {
    pub fn new(prices: &'a [&'a str], product: &'a str) -> Self {
        Self { prices, product }
    }
}
/// Determines how to handle prorations resulting from subscription updates.
///
/// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl UpdateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for UpdateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateConfigurationFeaturesSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The hosted login page for this configuration.
///
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    ///
    /// Set to `false` to deactivate the `login_page.url`.
    pub enabled: bool,
}
impl UpdateConfigurationLoginPage {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
