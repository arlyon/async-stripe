
/// Creates a usage record for a specified subscription item and date, and fills it with a quantity.
///
/// Usage records provide `quantity` information that Stripe uses to track how much a customer is using your service.
///
/// With usage information and the pricing model set up by the [metered billing](https://stripe.com/docs/billing/subscriptions/metered-billing) plan, Stripe helps you send accurate invoices to your customers.  The default calculation for usage is to add up all the `quantity` values of the usage records within a billing period.
/// You can change this default behavior with the billing plan’s `aggregate_usage` [parameter](https://stripe.com/docs/api/plans/create#create_plan-aggregate_usage).
/// When there is more than one usage record with the same timestamp, Stripe adds the `quantity` values together.
/// In most cases, this is the desired resolution, however, you can change this behavior with the `action` parameter.  The default pricing model for metered billing is [per-unit pricing](https://stripe.com/docs/api/plans/object#plan_object-billing_scheme).
/// For finer granularity, you can configure metered billing to have a [tiered pricing](https://stripe.com/docs/billing/subscriptions/tiers) model.
pub fn create(
    client: &stripe::Client,
    subscription_item: &stripe_types::subscription_item::SubscriptionItemId,
    params: CreateUsageRecord,
) -> stripe::Response<stripe_billing::usage_record::UsageRecord> {
    client.send_form(
        &format!(
            "/subscription_items/{subscription_item}/usage_records",
            subscription_item = subscription_item
        ),
        params,
        http_types::Method::Post,
    )
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateUsageRecord<'a> {
    /// Valid values are `increment` (default) or `set`.
    ///
    /// When using `increment` the specified `quantity` will be added to the usage at the specified timestamp.
    /// The `set` action will overwrite the usage quantity at that timestamp.
    /// If the subscription has [billing thresholds](https://stripe.com/docs/api/subscriptions/object#subscription_object-billing_thresholds), `increment` is the only allowed value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<CreateUsageRecordAction>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The usage quantity for the specified timestamp.
    pub quantity: u64,
    /// The timestamp for the usage event.
    ///
    /// This timestamp must be within the current billing period of the subscription of the provided `subscription_item`, and must not be in the future.
    /// When passing `"now"`, Stripe records usage for the current time.
    /// Default is `"now"` if a value is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<CreateUsageRecordTimestamp>,
}
impl<'a> CreateUsageRecord<'a> {
    pub fn new(quantity: u64) -> Self {
        Self {
            action: Default::default(),
            expand: Default::default(),
            quantity,
            timestamp: Default::default(),
        }
    }
}
/// Valid values are `increment` (default) or `set`.
///
/// When using `increment` the specified `quantity` will be added to the usage at the specified timestamp.
/// The `set` action will overwrite the usage quantity at that timestamp.
/// If the subscription has [billing thresholds](https://stripe.com/docs/api/subscriptions/object#subscription_object-billing_thresholds), `increment` is the only allowed value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateUsageRecordAction {
    Increment,
    Set,
}

impl CreateUsageRecordAction {
    pub fn as_str(self) -> &'static str {
        use CreateUsageRecordAction::*;
        match self {
            Increment => "increment",
            Set => "set",
        }
    }
}

impl std::str::FromStr for CreateUsageRecordAction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateUsageRecordAction::*;
        match s {
            "increment" => Ok(Increment),
            "set" => Ok(Set),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateUsageRecordAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateUsageRecordAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateUsageRecordAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The timestamp for the usage event.
///
/// This timestamp must be within the current billing period of the subscription of the provided `subscription_item`, and must not be in the future.
/// When passing `"now"`, Stripe records usage for the current time.
/// Default is `"now"` if a value is not provided.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreateUsageRecordTimestamp {
    Now,
    I64(i64),
}
