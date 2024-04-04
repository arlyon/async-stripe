#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListBillingPortalConfiguration<'a> {
    /// Only return configurations that are active or inactive (e.g., pass `true` to only list active configurations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A cursor for use in pagination.
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
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListBillingPortalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListBillingPortalConfiguration<'a> {
    /// Returns a list of configurations that describe the functionality of the customer portal.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_billing::BillingPortalConfiguration>> {
        client.get_query("/billing_portal/configurations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_billing::BillingPortalConfiguration>> {
        stripe::ListPaginator::from_list_params("/billing_portal/configurations", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveBillingPortalConfiguration<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveBillingPortalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveBillingPortalConfiguration<'a> {
    /// Retrieves a configuration that describes the functionality of the customer portal.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_billing::BillingPortalConfigurationId,
    ) -> stripe::Response<stripe_billing::BillingPortalConfiguration> {
        client.get_query(&format!("/billing_portal/configurations/{configuration}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingPortalConfiguration<'a> {
    /// The business information shown to customers in the portal.
    pub business_profile: CreateBillingPortalConfigurationBusinessProfile<'a>,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Information about the features available in the portal.
    pub features: CreateBillingPortalConfigurationFeatures<'a>,
    /// The hosted login page for this configuration.
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<CreateBillingPortalConfigurationLoginPage>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> CreateBillingPortalConfiguration<'a> {
    pub fn new(
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause: Option<SubscriptionPauseParam>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> CreateBillingPortalConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
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
    type Err = ();
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
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
impl<'a> CreateBillingPortalConfiguration<'a> {
    /// Creates a configuration that describes the functionality and behavior of a PortalSession
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_billing::BillingPortalConfiguration> {
        client.send_form("/billing_portal/configurations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateBillingPortalConfiguration<'a> {
    /// Whether the configuration is active and can be used to create portal sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The business information shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<UpdateBillingPortalConfigurationBusinessProfile<'a>>,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Information about the features available in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<UpdateBillingPortalConfigurationFeatures<'a>>,
    /// The hosted login page for this configuration.
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<UpdateBillingPortalConfigurationLoginPage>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateBillingPortalConfiguration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The business information shown to customers in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
    }
}
/// Information about the features available in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause: Option<SubscriptionPauseParam>,
    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<UpdateBillingPortalConfigurationFeaturesSubscriptionUpdate<'a>>,
}
impl<'a> UpdateBillingPortalConfigurationFeatures<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about updating the customer details in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesCustomerUpdateAllowedUpdates::*;
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
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
    type Err = ();
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
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
/// Information about updating subscriptions in the portal.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateDefaultAllowedUpdates::*;
        match s {
            "price" => Ok(Price),
            "promotion_code" => Ok(PromotionCode),
            "quantity" => Ok(Quantity),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateBillingPortalConfigurationFeaturesSubscriptionUpdateProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(()),
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
impl<'a> UpdateBillingPortalConfiguration<'a> {
    /// Updates a configuration that describes the functionality of the customer portal.
    pub fn send(
        &self,
        client: &stripe::Client,
        configuration: &stripe_billing::BillingPortalConfigurationId,
    ) -> stripe::Response<stripe_billing::BillingPortalConfiguration> {
        client.send_form(
            &format!("/billing_portal/configurations/{configuration}"),
            self,
            http_types::Method::Post,
        )
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
