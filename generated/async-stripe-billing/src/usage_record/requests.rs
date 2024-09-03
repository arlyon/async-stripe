use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateSubscriptionItemUsageRecordBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<CreateSubscriptionItemUsageRecordAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    quantity: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<CreateSubscriptionItemUsageRecordTimestamp>,
}
impl<'a> CreateSubscriptionItemUsageRecordBuilder<'a> {
    fn new(quantity: u64) -> Self {
        Self { action: None, expand: None, quantity, timestamp: None }
    }
}
/// Valid values are `increment` (default) or `set`.
/// When using `increment` the specified `quantity` will be added to the usage at the specified timestamp.
/// The `set` action will overwrite the usage quantity at that timestamp.
/// If the subscription has [billing thresholds](https://stripe.com/docs/api/subscriptions/object#subscription_object-billing_thresholds), `increment` is the only allowed value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionItemUsageRecordAction {
    Increment,
    Set,
}
impl CreateSubscriptionItemUsageRecordAction {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionItemUsageRecordAction::*;
        match self {
            Increment => "increment",
            Set => "set",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionItemUsageRecordAction {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionItemUsageRecordAction::*;
        match s {
            "increment" => Ok(Increment),
            "set" => Ok(Set),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionItemUsageRecordAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionItemUsageRecordAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionItemUsageRecordAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionItemUsageRecordAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSubscriptionItemUsageRecordAction")
        })
    }
}
/// The timestamp for the usage event.
/// This timestamp must be within the current billing period of the subscription of the provided `subscription_item`, and must not be in the future.
/// When passing `"now"`, Stripe records usage for the current time.
/// Default is `"now"` if a value is not provided.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionItemUsageRecordTimestamp {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Creates a usage record for a specified subscription item and date, and fills it with a quantity.
///
/// Usage records provide `quantity` information that Stripe uses to track how much a customer is using your service.
/// With usage information and the pricing model set up by the [metered billing](https://stripe.com/docs/billing/subscriptions/metered-billing) plan, Stripe helps you send accurate invoices to your customers.
///
/// The default calculation for usage is to add up all the `quantity` values of the usage records within a billing period.
/// You can change this default behavior with the billing plan’s `aggregate_usage` [parameter](https://stripe.com/docs/api/plans/create#create_plan-aggregate_usage).
/// When there is more than one usage record with the same timestamp, Stripe adds the `quantity` values together.
/// In most cases, this is the desired resolution, however, you can change this behavior with the `action` parameter.
///
/// The default pricing model for metered billing is [per-unit pricing](https://stripe.com/docs/api/plans/object#plan_object-billing_scheme).
/// For finer granularity, you can configure metered billing to have a [tiered pricing](https://stripe.com/docs/billing/subscriptions/tiers) model.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateSubscriptionItemUsageRecord<'a> {
    inner: CreateSubscriptionItemUsageRecordBuilder<'a>,
    subscription_item: &'a stripe_shared::SubscriptionItemId,
}
impl<'a> CreateSubscriptionItemUsageRecord<'a> {
    /// Construct a new `CreateSubscriptionItemUsageRecord`.
    pub fn new(subscription_item: &'a stripe_shared::SubscriptionItemId, quantity: u64) -> Self {
        Self { subscription_item, inner: CreateSubscriptionItemUsageRecordBuilder::new(quantity) }
    }
    /// Valid values are `increment` (default) or `set`.
    /// When using `increment` the specified `quantity` will be added to the usage at the specified timestamp.
    /// The `set` action will overwrite the usage quantity at that timestamp.
    /// If the subscription has [billing thresholds](https://stripe.com/docs/api/subscriptions/object#subscription_object-billing_thresholds), `increment` is the only allowed value.
    pub fn action(mut self, action: CreateSubscriptionItemUsageRecordAction) -> Self {
        self.inner.action = Some(action);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The timestamp for the usage event.
    /// This timestamp must be within the current billing period of the subscription of the provided `subscription_item`, and must not be in the future.
    /// When passing `"now"`, Stripe records usage for the current time.
    /// Default is `"now"` if a value is not provided.
    pub fn timestamp(mut self, timestamp: CreateSubscriptionItemUsageRecordTimestamp) -> Self {
        self.inner.timestamp = Some(timestamp);
        self
    }
}
impl CreateSubscriptionItemUsageRecord<'_> {
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

impl StripeRequest for CreateSubscriptionItemUsageRecord<'_> {
    type Output = stripe_billing::UsageRecord;

    fn build(&self) -> RequestBuilder {
        let subscription_item = self.subscription_item;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/subscription_items/{subscription_item}/usage_records"),
        )
        .form(&self.inner)
    }
}
