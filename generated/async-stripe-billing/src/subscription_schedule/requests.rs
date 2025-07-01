use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListSubscriptionScheduleBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    canceled_at: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<stripe_types::RangeQueryTs>,
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
    released_at: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListSubscriptionScheduleBuilder {
    fn new() -> Self {
        Self {
            canceled_at: None,
            completed_at: None,
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            released_at: None,
            scheduled: None,
            starting_after: None,
        }
    }
}
/// Retrieves the list of your subscription schedules.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListSubscriptionSchedule {
    inner: ListSubscriptionScheduleBuilder,
}
impl ListSubscriptionSchedule {
    /// Construct a new `ListSubscriptionSchedule`.
    pub fn new() -> Self {
        Self { inner: ListSubscriptionScheduleBuilder::new() }
    }
    /// Only return subscription schedules that were created canceled the given date interval.
    pub fn canceled_at(mut self, canceled_at: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.canceled_at = Some(canceled_at.into());
        self
    }
    /// Only return subscription schedules that completed during the given date interval.
    pub fn completed_at(mut self, completed_at: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.completed_at = Some(completed_at.into());
        self
    }
    /// Only return subscription schedules that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return subscription schedules for the given customer.
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
    /// Only return subscription schedules that were released during the given date interval.
    pub fn released_at(mut self, released_at: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.released_at = Some(released_at.into());
        self
    }
    /// Only return subscription schedules that have not started yet.
    pub fn scheduled(mut self, scheduled: impl Into<bool>) -> Self {
        self.inner.scheduled = Some(scheduled.into());
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
impl Default for ListSubscriptionSchedule {
    fn default() -> Self {
        Self::new()
    }
}
impl ListSubscriptionSchedule {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::SubscriptionSchedule>>
    {
        stripe_client_core::ListPaginator::new_list("/subscription_schedules", &self.inner)
    }
}

impl StripeRequest for ListSubscriptionSchedule {
    type Output = stripe_types::List<stripe_shared::SubscriptionSchedule>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/subscription_schedules").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveSubscriptionScheduleBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveSubscriptionScheduleBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing subscription schedule.
/// You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveSubscriptionSchedule {
    inner: RetrieveSubscriptionScheduleBuilder,
    schedule: stripe_shared::SubscriptionScheduleId,
}
impl RetrieveSubscriptionSchedule {
    /// Construct a new `RetrieveSubscriptionSchedule`.
    pub fn new(schedule: impl Into<stripe_shared::SubscriptionScheduleId>) -> Self {
        Self { schedule: schedule.into(), inner: RetrieveSubscriptionScheduleBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveSubscriptionSchedule {
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

impl StripeRequest for RetrieveSubscriptionSchedule {
    type Output = stripe_shared::SubscriptionSchedule;

    fn build(&self) -> RequestBuilder {
        let schedule = &self.schedule;
        RequestBuilder::new(StripeMethod::Get, format!("/subscription_schedules/{schedule}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateSubscriptionScheduleBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_settings: Option<CreateSubscriptionScheduleDefaultSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_behavior: Option<stripe_shared::SubscriptionScheduleEndBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phases: Option<Vec<CreateSubscriptionSchedulePhases>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<CreateSubscriptionScheduleStartDate>,
}
impl CreateSubscriptionScheduleBuilder {
    fn new() -> Self {
        Self {
            customer: None,
            default_settings: None,
            end_behavior: None,
            expand: None,
            from_subscription: None,
            metadata: None,
            phases: None,
            start_date: None,
        }
    }
}
/// Object representing the subscription schedule's default settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettings {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Default settings for automatic tax computation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSubscriptionScheduleDefaultSettingsAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CreateSubscriptionScheduleDefaultSettingsCollectionMethod>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateSubscriptionScheduleDefaultSettingsInvoiceSettings>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs>,
}
impl CreateSubscriptionScheduleDefaultSettings {
    pub fn new() -> Self {
        Self {
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            default_payment_method: None,
            description: None,
            invoice_settings: None,
            on_behalf_of: None,
            transfer_data: None,
        }
    }
}
impl Default for CreateSubscriptionScheduleDefaultSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Default settings for automatic tax computation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiability>,
}
impl CreateSubscriptionScheduleDefaultSettingsAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType,
}
impl CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiability {
    pub fn new(
        type_: impl Into<CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType"))
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionScheduleDefaultSettingsBillingCycleAnchor",
            )
        })
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionScheduleDefaultSettingsCollectionMethod",
            )
        })
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    /// The account tax IDs associated with the subscription schedule.
    /// Will be set on invoices generated by the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuer>,
}
impl CreateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    pub fn new() -> Self {
        Self { account_tax_ids: None, days_until_due: None, issuer: None }
    }
}
impl Default for CreateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType,
}
impl CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType"))
    }
}
/// List representing phases of the subscription schedule.
/// Each phase can be customized to have different durations, plans, and coupons.
/// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhases {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<CreateSubscriptionSchedulePhasesAddInvoiceItems>>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSubscriptionSchedulePhasesAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<CreateSubscriptionSchedulePhasesBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CreateSubscriptionSchedulePhasesCollectionMethod>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The coupons to redeem into discounts for the schedule phase.
    /// If not specified, inherits the discount from the subscription's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The date at which this phase of the subscription schedule ends.
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<stripe_types::Timestamp>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateSubscriptionSchedulePhasesInvoiceSettings>,
    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: Vec<CreateSubscriptionSchedulePhasesItems>,
    /// Integer representing the multiplier applied to the price interval.
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Controls whether the subscription schedule should create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase if there is a difference in billing configuration.
    /// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration (item price, quantity, etc.) of the current phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateSubscriptionSchedulePhasesProrationBehavior>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs>,
    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    /// Sets the phase to trialing from the start date to this date.
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<stripe_types::Timestamp>,
}
impl CreateSubscriptionSchedulePhases {
    pub fn new(items: impl Into<Vec<CreateSubscriptionSchedulePhasesItems>>) -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            currency: None,
            default_payment_method: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            end_date: None,
            invoice_settings: None,
            items: items.into(),
            iterations: None,
            metadata: None,
            on_behalf_of: None,
            proration_behavior: None,
            transfer_data: None,
            trial: None,
            trial_end: None,
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
/// You may pass up to 20 items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesAddInvoiceItems {
    /// The coupons to redeem into discounts for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the price object. One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceData>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl CreateSubscriptionSchedulePhasesAddInvoiceItems {
    pub fn new() -> Self {
        Self { discounts: None, price: None, price_data: None, quantity: None, tax_rates: None }
    }
}
impl Default for CreateSubscriptionSchedulePhasesAddInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
/// One of `price` or `price_data` is required.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge or a negative integer representing the amount to credit to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior"))
    }
}
/// Automatic tax settings for this phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateSubscriptionSchedulePhasesAutomaticTaxLiability>,
}
impl CreateSubscriptionSchedulePhasesAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType,
}
impl CreateSubscriptionSchedulePhasesAutomaticTaxLiability {
    pub fn new(
        type_: impl Into<CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionSchedulePhasesAutomaticTaxLiabilityType",
            )
        })
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionSchedulePhasesBillingCycleAnchor",
            )
        })
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl CreateSubscriptionSchedulePhasesCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionSchedulePhasesCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionSchedulePhasesCollectionMethod",
            )
        })
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesInvoiceSettings {
    /// The account tax IDs associated with this phase of the subscription schedule.
    /// Will be set on invoices generated by this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateSubscriptionSchedulePhasesInvoiceSettingsIssuer>,
}
impl CreateSubscriptionSchedulePhasesInvoiceSettings {
    pub fn new() -> Self {
        Self { account_tax_ids: None, days_until_due: None, issuer: None }
    }
}
impl Default for CreateSubscriptionSchedulePhasesInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType,
}
impl CreateSubscriptionSchedulePhasesInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionSchedulePhasesInvoiceSettingsIssuerType",
            )
        })
    }
}
/// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a configuration item.
    /// Metadata on a configuration item will update the underlying subscription item's `metadata` when the phase is entered, adding new keys and replacing existing keys.
    /// Individual keys in the subscription item's `metadata` can be unset by posting an empty value to them in the configuration item's `metadata`.
    /// To unset all keys in the subscription item's `metadata`, update the subscription item directly or unset every key individually from the configuration item's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The plan ID to subscribe to. You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateSubscriptionSchedulePhasesItemsPriceData>,
    /// Quantity for the given price.
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl CreateSubscriptionSchedulePhasesItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for CreateSubscriptionSchedulePhasesItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: CreateSubscriptionSchedulePhasesItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreateSubscriptionSchedulePhasesItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<CreateSubscriptionSchedulePhasesItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Controls whether the subscription schedule should create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase if there is a difference in billing configuration.
/// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration (item price, quantity, etc.) of the current phase.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionSchedulePhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreateSubscriptionSchedulePhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionSchedulePhasesProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionSchedulePhasesProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionSchedulePhasesProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionSchedulePhasesProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionSchedulePhasesProrationBehavior",
            )
        })
    }
}
/// When the subscription schedule starts.
/// We recommend using `now` so that it starts the subscription immediately.
/// You can also use a Unix timestamp to backdate the subscription so that it starts on a past date, or set a future date for the subscription to start on.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionScheduleStartDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Creates a new subscription schedule object.
/// Each customer can have up to 500 active or scheduled subscriptions.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionSchedule {
    inner: CreateSubscriptionScheduleBuilder,
}
impl CreateSubscriptionSchedule {
    /// Construct a new `CreateSubscriptionSchedule`.
    pub fn new() -> Self {
        Self { inner: CreateSubscriptionScheduleBuilder::new() }
    }
    /// The identifier of the customer to create the subscription schedule for.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// Object representing the subscription schedule's default settings.
    pub fn default_settings(
        mut self,
        default_settings: impl Into<CreateSubscriptionScheduleDefaultSettings>,
    ) -> Self {
        self.inner.default_settings = Some(default_settings.into());
        self
    }
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.
    /// `cancel` will end the subscription schedule and cancel the underlying subscription.
    pub fn end_behavior(
        mut self,
        end_behavior: impl Into<stripe_shared::SubscriptionScheduleEndBehavior>,
    ) -> Self {
        self.inner.end_behavior = Some(end_behavior.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Migrate an existing subscription to be managed by a subscription schedule.
    /// If this parameter is set, a subscription schedule will be created using the subscription's item(s), set to auto-renew using the subscription's interval.
    /// When using this parameter, other parameters (such as phase values) cannot be set.
    /// To create a subscription schedule with other modifications, we recommend making two separate API calls.
    pub fn from_subscription(mut self, from_subscription: impl Into<String>) -> Self {
        self.inner.from_subscription = Some(from_subscription.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// List representing phases of the subscription schedule.
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    pub fn phases(mut self, phases: impl Into<Vec<CreateSubscriptionSchedulePhases>>) -> Self {
        self.inner.phases = Some(phases.into());
        self
    }
    /// When the subscription schedule starts.
    /// We recommend using `now` so that it starts the subscription immediately.
    /// You can also use a Unix timestamp to backdate the subscription so that it starts on a past date, or set a future date for the subscription to start on.
    pub fn start_date(
        mut self,
        start_date: impl Into<CreateSubscriptionScheduleStartDate>,
    ) -> Self {
        self.inner.start_date = Some(start_date.into());
        self
    }
}
impl Default for CreateSubscriptionSchedule {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateSubscriptionSchedule {
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

impl StripeRequest for CreateSubscriptionSchedule {
    type Output = stripe_shared::SubscriptionSchedule;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/subscription_schedules").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateSubscriptionScheduleBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_settings: Option<UpdateSubscriptionScheduleDefaultSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_behavior: Option<stripe_shared::SubscriptionScheduleEndBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phases: Option<Vec<UpdateSubscriptionSchedulePhases>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<UpdateSubscriptionScheduleProrationBehavior>,
}
impl UpdateSubscriptionScheduleBuilder {
    fn new() -> Self {
        Self {
            default_settings: None,
            end_behavior: None,
            expand: None,
            metadata: None,
            phases: None,
            proration_behavior: None,
        }
    }
}
/// Object representing the subscription schedule's default settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettings {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Default settings for automatic tax computation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateSubscriptionScheduleDefaultSettingsAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UpdateSubscriptionScheduleDefaultSettingsCollectionMethod>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs>,
}
impl UpdateSubscriptionScheduleDefaultSettings {
    pub fn new() -> Self {
        Self {
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            default_payment_method: None,
            description: None,
            invoice_settings: None,
            on_behalf_of: None,
            transfer_data: None,
        }
    }
}
impl Default for UpdateSubscriptionScheduleDefaultSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Default settings for automatic tax computation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiability>,
}
impl UpdateSubscriptionScheduleDefaultSettingsAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType,
}
impl UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiability {
    pub fn new(
        type_: impl Into<UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSubscriptionScheduleDefaultSettingsAutomaticTaxLiabilityType"))
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionScheduleDefaultSettingsBillingCycleAnchor",
            )
        })
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleDefaultSettingsCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod",
            )
        })
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    /// The account tax IDs associated with the subscription schedule.
    /// Will be set on invoices generated by the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuer>,
}
impl UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    pub fn new() -> Self {
        Self { account_tax_ids: None, days_until_due: None, issuer: None }
    }
}
impl Default for UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType,
}
impl UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSubscriptionScheduleDefaultSettingsInvoiceSettingsIssuerType"))
    }
}
/// List representing phases of the subscription schedule.
/// Each phase can be customized to have different durations, plans, and coupons.
/// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
/// Note that past phases can be omitted.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhases {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<UpdateSubscriptionSchedulePhasesAddInvoiceItems>>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateSubscriptionSchedulePhasesAutomaticTax>,
    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<UpdateSubscriptionSchedulePhasesBillingCycleAnchor>,
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholdsParam>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UpdateSubscriptionSchedulePhasesCollectionMethod>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the default payment method for the subscription schedule.
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    /// Subscription description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The coupons to redeem into discounts for the schedule phase.
    /// If not specified, inherits the discount from the subscription's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The date at which this phase of the subscription schedule ends.
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<UpdateSubscriptionSchedulePhasesEndDate>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateSubscriptionSchedulePhasesInvoiceSettings>,
    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: Vec<UpdateSubscriptionSchedulePhasesItems>,
    /// Integer representing the multiplier applied to the price interval.
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Controls whether the subscription schedule should create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase if there is a difference in billing configuration.
    /// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration (item price, quantity, etc.) of the current phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<UpdateSubscriptionSchedulePhasesProrationBehavior>,
    /// The date at which this phase of the subscription schedule starts or `now`.
    /// Must be set on the first phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<UpdateSubscriptionSchedulePhasesStartDate>,
    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataSpecs>,
    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    /// Sets the phase to trialing from the start date to this date.
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<UpdateSubscriptionSchedulePhasesTrialEnd>,
}
impl UpdateSubscriptionSchedulePhases {
    pub fn new(items: impl Into<Vec<UpdateSubscriptionSchedulePhasesItems>>) -> Self {
        Self {
            add_invoice_items: None,
            application_fee_percent: None,
            automatic_tax: None,
            billing_cycle_anchor: None,
            billing_thresholds: None,
            collection_method: None,
            currency: None,
            default_payment_method: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            end_date: None,
            invoice_settings: None,
            items: items.into(),
            iterations: None,
            metadata: None,
            on_behalf_of: None,
            proration_behavior: None,
            start_date: None,
            transfer_data: None,
            trial: None,
            trial_end: None,
        }
    }
}
/// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
/// You may pass up to 20 items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAddInvoiceItems {
    /// The coupons to redeem into discounts for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the price object. One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceData>,
    /// Quantity for this item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpdateSubscriptionSchedulePhasesAddInvoiceItems {
    pub fn new() -> Self {
        Self { discounts: None, price: None, price_data: None, quantity: None, tax_rates: None }
    }
}
impl Default for UpdateSubscriptionSchedulePhasesAddInvoiceItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
/// One of `price` or `price_data` is required.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    pub product: String,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge or a negative integer representing the amount to credit to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for UpdateSubscriptionSchedulePhasesAddInvoiceItemsPriceDataTaxBehavior"))
    }
}
/// Automatic tax settings for this phase.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdateSubscriptionSchedulePhasesAutomaticTaxLiability>,
}
impl UpdateSubscriptionSchedulePhasesAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType,
}
impl UpdateSubscriptionSchedulePhasesAutomaticTaxLiability {
    pub fn new(
        type_: impl Into<UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionSchedulePhasesAutomaticTaxLiabilityType",
            )
        })
    }
}
/// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
/// Cannot be set to `phase_start` if this phase specifies a trial.
/// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}
impl UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match self {
            Automatic => "automatic",
            PhaseStart => "phase_start",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesBillingCycleAnchor::*;
        match s {
            "automatic" => Ok(Automatic),
            "phase_start" => Ok(PhaseStart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionSchedulePhasesBillingCycleAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionSchedulePhasesBillingCycleAnchor",
            )
        })
    }
}
/// Either `charge_automatically`, or `send_invoice`.
/// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically` on creation.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl UpdateSubscriptionSchedulePhasesCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionSchedulePhasesCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionSchedulePhasesCollectionMethod",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule ends.
/// If set, `iterations` must not be set.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionSchedulePhasesEndDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesInvoiceSettings {
    /// The account tax IDs associated with this phase of the subscription schedule.
    /// Will be set on invoices generated by this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuer>,
}
impl UpdateSubscriptionSchedulePhasesInvoiceSettings {
    pub fn new() -> Self {
        Self { account_tax_ids: None, days_until_due: None, issuer: None }
    }
}
impl Default for UpdateSubscriptionSchedulePhasesInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType,
}
impl UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuer {
    pub fn new(
        type_: impl Into<UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType>,
    ) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionSchedulePhasesInvoiceSettingsIssuerType",
            )
        })
    }
}
/// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<ItemBillingThresholdsParam>,
    /// The coupons to redeem into discounts for the subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a configuration item.
    /// Metadata on a configuration item will update the underlying subscription item's `metadata` when the phase is entered, adding new keys and replacing existing keys.
    /// Individual keys in the subscription item's `metadata` can be unset by posting an empty value to them in the configuration item's `metadata`.
    /// To unset all keys in the subscription item's `metadata`, update the subscription item directly or unset every key individually from the configuration item's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The plan ID to subscribe to. You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateSubscriptionSchedulePhasesItemsPriceData>,
    /// Quantity for the given price.
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpdateSubscriptionSchedulePhasesItems {
    pub fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            metadata: None,
            plan: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpdateSubscriptionSchedulePhasesItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpdateSubscriptionSchedulePhasesItemsPriceData {
    pub fn new(
        currency: impl Into<stripe_types::Currency>,
        product: impl Into<String>,
        recurring: impl Into<UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring>,
    ) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: recurring.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateSubscriptionSchedulePhasesItemsPriceDataRecurring {
    pub fn new(
        interval: impl Into<UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval>,
    ) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionSchedulePhasesItemsPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionSchedulePhasesItemsPriceDataTaxBehavior",
            )
        })
    }
}
/// Controls whether the subscription schedule should create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase if there is a difference in billing configuration.
/// It's different from the request-level [proration_behavior](https://stripe.com/docs/api/subscription_schedules/update#update_subscription_schedule-proration_behavior) parameter which controls what happens if the update request affects the billing configuration (item price, quantity, etc.) of the current phase.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionSchedulePhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpdateSubscriptionSchedulePhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionSchedulePhasesProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionSchedulePhasesProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionSchedulePhasesProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionSchedulePhasesProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionSchedulePhasesProrationBehavior",
            )
        })
    }
}
/// The date at which this phase of the subscription schedule starts or `now`.
/// Must be set on the first phase.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionSchedulePhasesStartDate {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Sets the phase to trialing from the start date to this date.
/// Must be before the phase end date, can not be combined with `trial`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionSchedulePhasesTrialEnd {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// If the update changes the billing configuration (item price, quantity, etc.) of the current phase, indicates how prorations from this change should be handled.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionScheduleProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpdateSubscriptionScheduleProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionScheduleProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionScheduleProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionScheduleProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionScheduleProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionScheduleProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionScheduleProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionScheduleProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionScheduleProrationBehavior",
            )
        })
    }
}
/// Updates an existing subscription schedule.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateSubscriptionSchedule {
    inner: UpdateSubscriptionScheduleBuilder,
    schedule: stripe_shared::SubscriptionScheduleId,
}
impl UpdateSubscriptionSchedule {
    /// Construct a new `UpdateSubscriptionSchedule`.
    pub fn new(schedule: impl Into<stripe_shared::SubscriptionScheduleId>) -> Self {
        Self { schedule: schedule.into(), inner: UpdateSubscriptionScheduleBuilder::new() }
    }
    /// Object representing the subscription schedule's default settings.
    pub fn default_settings(
        mut self,
        default_settings: impl Into<UpdateSubscriptionScheduleDefaultSettings>,
    ) -> Self {
        self.inner.default_settings = Some(default_settings.into());
        self
    }
    /// Behavior of the subscription schedule and underlying subscription when it ends.
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.
    /// `cancel` will end the subscription schedule and cancel the underlying subscription.
    pub fn end_behavior(
        mut self,
        end_behavior: impl Into<stripe_shared::SubscriptionScheduleEndBehavior>,
    ) -> Self {
        self.inner.end_behavior = Some(end_behavior.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// List representing phases of the subscription schedule.
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    /// Note that past phases can be omitted.
    pub fn phases(mut self, phases: impl Into<Vec<UpdateSubscriptionSchedulePhases>>) -> Self {
        self.inner.phases = Some(phases.into());
        self
    }
    /// If the update changes the billing configuration (item price, quantity, etc.) of the current phase, indicates how prorations from this change should be handled.
    /// The default value is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: impl Into<UpdateSubscriptionScheduleProrationBehavior>,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior.into());
        self
    }
}
impl UpdateSubscriptionSchedule {
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

impl StripeRequest for UpdateSubscriptionSchedule {
    type Output = stripe_shared::SubscriptionSchedule;

    fn build(&self) -> RequestBuilder {
        let schedule = &self.schedule;
        RequestBuilder::new(StripeMethod::Post, format!("/subscription_schedules/{schedule}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CancelSubscriptionScheduleBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prorate: Option<bool>,
}
impl CancelSubscriptionScheduleBuilder {
    fn new() -> Self {
        Self { expand: None, invoice_now: None, prorate: None }
    }
}
/// Cancels a subscription schedule and its associated subscription immediately (if the subscription schedule has an active subscription).
/// A subscription schedule can only be canceled if its status is `not_started` or `active`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelSubscriptionSchedule {
    inner: CancelSubscriptionScheduleBuilder,
    schedule: stripe_shared::SubscriptionScheduleId,
}
impl CancelSubscriptionSchedule {
    /// Construct a new `CancelSubscriptionSchedule`.
    pub fn new(schedule: impl Into<stripe_shared::SubscriptionScheduleId>) -> Self {
        Self { schedule: schedule.into(), inner: CancelSubscriptionScheduleBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// If the subscription schedule is `active`, indicates if a final invoice will be generated that contains any un-invoiced metered usage and new/pending proration invoice items.
    /// Defaults to `true`.
    pub fn invoice_now(mut self, invoice_now: impl Into<bool>) -> Self {
        self.inner.invoice_now = Some(invoice_now.into());
        self
    }
    /// If the subscription schedule is `active`, indicates if the cancellation should be prorated.
    /// Defaults to `true`.
    pub fn prorate(mut self, prorate: impl Into<bool>) -> Self {
        self.inner.prorate = Some(prorate.into());
        self
    }
}
impl CancelSubscriptionSchedule {
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

impl StripeRequest for CancelSubscriptionSchedule {
    type Output = stripe_shared::SubscriptionSchedule;

    fn build(&self) -> RequestBuilder {
        let schedule = &self.schedule;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/subscription_schedules/{schedule}/cancel"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReleaseSubscriptionScheduleBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_cancel_date: Option<bool>,
}
impl ReleaseSubscriptionScheduleBuilder {
    fn new() -> Self {
        Self { expand: None, preserve_cancel_date: None }
    }
}
/// Releases the subscription schedule immediately, which will stop scheduling of its phases, but leave any existing subscription in place.
/// A schedule can only be released if its status is `not_started` or `active`.
/// If the subscription schedule is currently associated with a subscription, releasing it will remove its `subscription` property and set the subscription’s ID to the `released_subscription` property.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReleaseSubscriptionSchedule {
    inner: ReleaseSubscriptionScheduleBuilder,
    schedule: stripe_shared::SubscriptionScheduleId,
}
impl ReleaseSubscriptionSchedule {
    /// Construct a new `ReleaseSubscriptionSchedule`.
    pub fn new(schedule: impl Into<stripe_shared::SubscriptionScheduleId>) -> Self {
        Self { schedule: schedule.into(), inner: ReleaseSubscriptionScheduleBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Keep any cancellation on the subscription that the schedule has set
    pub fn preserve_cancel_date(mut self, preserve_cancel_date: impl Into<bool>) -> Self {
        self.inner.preserve_cancel_date = Some(preserve_cancel_date.into());
        self
    }
}
impl ReleaseSubscriptionSchedule {
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

impl StripeRequest for ReleaseSubscriptionSchedule {
    type Output = stripe_shared::SubscriptionSchedule;

    fn build(&self) -> RequestBuilder {
        let schedule = &self.schedule;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/subscription_schedules/{schedule}/release"),
        )
        .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct BillingThresholdsParam {
    /// Monetary threshold that triggers the subscription to advance to a new billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}
impl BillingThresholdsParam {
    pub fn new() -> Self {
        Self { amount_gte: None, reset_billing_cycle_anchor: None }
    }
}
impl Default for BillingThresholdsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct TransferDataSpecs {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
impl TransferDataSpecs {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount_percent: None, destination: destination.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct DiscountsDataParam {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
impl DiscountsDataParam {
    pub fn new() -> Self {
        Self { coupon: None, discount: None, promotion_code: None }
    }
}
impl Default for DiscountsDataParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ItemBillingThresholdsParam {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl ItemBillingThresholdsParam {
    pub fn new(usage_gte: impl Into<i64>) -> Self {
        Self { usage_gte: usage_gte.into() }
    }
}
