#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    /// Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<stripe_shared::ConnectAccountReference>,
}
