#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPortalConfiguration<'a> {
    /// Only return configurations that are active or inactive (e.g., pass `true` to only list active configurations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
}
impl<'a> ListPortalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> stripe::PaginationParams for ListPortalConfiguration<'a> {}
impl<'a> ListPortalConfiguration<'a> {
    /// Returns a list of configurations that describe the functionality of the customer portal.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_billing::PortalConfiguration>> {
        client.get_query("/billing_portal/configurations", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_billing::PortalConfiguration> {
        stripe::ListPaginator::from_params("/billing_portal/configurations", self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalConfiguration<'a> {
    /// The business information shown to customers in the portal.
    pub business_profile: BusinessProfile<'a>,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Information about the features available in the portal.
    pub features: CreatePortalConfigurationFeatures<'a>,
    /// The hosted login page for this configuration.
    ///
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<CreatePortalConfigurationLoginPage>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreatePortalConfiguration<'a> {
    pub fn new(
        business_profile: BusinessProfile<'a>,
        features: CreatePortalConfigurationFeatures<'a>,
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
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePortalConfigurationFeatures<'a> {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreatePortalConfigurationFeaturesCustomerUpdate<'a>>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<CreatePortalConfigurationFeaturesInvoiceHistory>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<CreatePortalConfigurationFeaturesPaymentMethodUpdate>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<CreatePortalConfigurationFeaturesSubscriptionCancel<'a>>,
    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause: Option<SubscriptionPauseParam>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreatePortalConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> CreatePortalConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalConfigurationFeaturesCustomerUpdate<'a> {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [AllowedUpdates]>,
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl<'a> CreatePortalConfigurationFeaturesCustomerUpdate<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { allowed_updates: Default::default(), enabled }
    }
}
/// Information about showing the billing history in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalConfigurationFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl CreatePortalConfigurationFeaturesInvoiceHistory {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about updating payment methods in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalConfigurationFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl CreatePortalConfigurationFeaturesPaymentMethodUpdate {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalConfigurationFeaturesSubscriptionCancel<'a> {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<CreatePortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a>>,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ProrationBehavior>,
}
impl<'a> CreatePortalConfigurationFeaturesSubscriptionCancel<'a> {
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
pub struct CreatePortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    pub options: &'a [Options],
}
impl<'a> CreatePortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    pub fn new(enabled: bool, options: &'a [Options]) -> Self {
        Self { enabled, options }
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalConfigurationFeaturesSubscriptionUpdate<'a> {
    /// The types of subscription updates that are supported.
    ///
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates: &'a [DefaultAllowedUpdates],
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The list of products that support subscription updates.
    pub products: &'a [SubscriptionUpdateProductParam<'a>],
    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ProrationBehavior>,
}
impl<'a> CreatePortalConfigurationFeaturesSubscriptionUpdate<'a> {
    pub fn new(
        default_allowed_updates: &'a [DefaultAllowedUpdates],
        enabled: bool,
        products: &'a [SubscriptionUpdateProductParam<'a>],
    ) -> Self {
        Self { default_allowed_updates, enabled, products, proration_behavior: Default::default() }
    }
}
/// The hosted login page for this configuration.
///
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    pub enabled: bool,
}
impl CreatePortalConfigurationLoginPage {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
impl<'a> CreatePortalConfiguration<'a> {
    /// Creates a configuration that describes the functionality and behavior of a PortalSession.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_billing::PortalConfiguration> {
        client.send_form("/billing_portal/configurations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePortalConfiguration<'a> {
    /// Whether the configuration is active and can be used to create portal sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The business information shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile<'a>>,
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
    pub features: Option<UpdatePortalConfigurationFeatures<'a>>,
    /// The hosted login page for this configuration.
    ///
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<UpdatePortalConfigurationLoginPage>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdatePortalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePortalConfigurationFeatures<'a> {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<UpdatePortalConfigurationFeaturesCustomerUpdate<'a>>,
    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<UpdatePortalConfigurationFeaturesInvoiceHistory>,
    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<UpdatePortalConfigurationFeaturesPaymentMethodUpdate>,
    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<UpdatePortalConfigurationFeaturesSubscriptionCancel<'a>>,
    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause: Option<SubscriptionPauseParam>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<UpdatePortalConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> UpdatePortalConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePortalConfigurationFeaturesCustomerUpdate<'a> {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<&'a [AllowedUpdates]>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl<'a> UpdatePortalConfigurationFeaturesCustomerUpdate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about showing the billing history in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePortalConfigurationFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl UpdatePortalConfigurationFeaturesInvoiceHistory {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about updating payment methods in the portal.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePortalConfigurationFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
impl UpdatePortalConfigurationFeaturesPaymentMethodUpdate {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Information about canceling subscriptions in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePortalConfigurationFeaturesSubscriptionCancel<'a> {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<UpdatePortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a>>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ProrationBehavior>,
}
impl<'a> UpdatePortalConfigurationFeaturesSubscriptionCancel<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Which cancellation reasons will be given as options to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<&'a [Options]>,
}
impl<'a> UpdatePortalConfigurationFeaturesSubscriptionCancelCancellationReason<'a> {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, options: Default::default() }
    }
}
/// Information about updating subscriptions in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePortalConfigurationFeaturesSubscriptionUpdate<'a> {
    /// The types of subscription updates that are supported.
    ///
    /// When empty, subscriptions are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allowed_updates: Option<&'a [DefaultAllowedUpdates]>,
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The list of products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<&'a [SubscriptionUpdateProductParam<'a>]>,
    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<ProrationBehavior>,
}
impl<'a> UpdatePortalConfigurationFeaturesSubscriptionUpdate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The hosted login page for this configuration.
///
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdatePortalConfigurationLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    ///
    /// Set to `false` to deactivate the `login_page.url`.
    pub enabled: bool,
}
impl UpdatePortalConfigurationLoginPage {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
impl<'a> UpdatePortalConfiguration<'a> {
    /// Updates a configuration that describes the functionality of the customer portal.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_billing::portal_configuration::BillingPortalConfigurationId,
    ) -> stripe::Response<stripe_billing::PortalConfiguration> {
        client.send_form(
            &format!(
                "/billing_portal/configurations/{configuration}",
                configuration = configuration
            ),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePortalConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePortalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePortalConfiguration<'a> {
    /// Retrieves a configuration that describes the functionality of the customer portal.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_billing::portal_configuration::BillingPortalConfigurationId,
    ) -> stripe::Response<stripe_billing::PortalConfiguration> {
        client.get_query(
            &format!(
                "/billing_portal/configurations/{configuration}",
                configuration = configuration
            ),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BusinessProfile<'a> {
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
impl<'a> BusinessProfile<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AllowedUpdates {
    Address,
    Email,
    Name,
    Phone,
    Shipping,
    TaxId,
}

impl AllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use AllowedUpdates::*;
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

impl std::str::FromStr for AllowedUpdates {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AllowedUpdates::*;
        match s {
            "address" => Ok(Address),
            "email" => Ok(Email),
            "name" => Ok(Name),
            "phone" => Ok(Phone),
            "shipping" => Ok(Shipping),
            "tax_id" => Ok(TaxId),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Options {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl Options {
    pub fn as_str(self) -> &'static str {
        use Options::*;
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

impl std::str::FromStr for Options {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Options::*;
        match s {
            "customer_service" => Ok(CustomerService),
            "low_quality" => Ok(LowQuality),
            "missing_features" => Ok(MissingFeatures),
            "other" => Ok(Other),
            "switched_service" => Ok(SwitchedService),
            "too_complex" => Ok(TooComplex),
            "too_expensive" => Ok(TooExpensive),
            "unused" => Ok(Unused),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Options {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Options {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    AtPeriodEnd,
    Immediately,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        use Mode::*;
        match self {
            AtPeriodEnd => "at_period_end",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for Mode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Mode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Mode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl ProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use ProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for ProrationBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct SubscriptionPauseParam {
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl SubscriptionPauseParam {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl DefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        use DefaultAllowedUpdates::*;
        match self {
            Price => "price",
            PromotionCode => "promotion_code",
            Quantity => "quantity",
        }
    }
}

impl std::str::FromStr for DefaultAllowedUpdates {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for DefaultAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DefaultAllowedUpdates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
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
