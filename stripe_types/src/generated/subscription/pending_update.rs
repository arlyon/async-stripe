/// Pending Updates store the changes pending from a previous update that will be applied
/// to the Subscription upon successful payment.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PendingUpdate {
    /// If the update is applied, determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    ///
    /// The timestamp is in UTC format.
    pub billing_cycle_anchor: Option<stripe_types::Timestamp>,
    /// The point after which the changes reflected by this update will be discarded and no longer applied.
    pub expires_at: stripe_types::Timestamp,
    /// List of subscription items, each with an attached plan, that will be set if the update is applied.
    pub subscription_items: Option<Vec<stripe_types::subscription_item::SubscriptionItem>>,
    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time, if the update is applied.
    pub trial_end: Option<stripe_types::Timestamp>,
    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    pub trial_from_plan: Option<bool>,
}
