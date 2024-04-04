#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectEmbeddedPayoutsFeatures {
    /// Whether to allow payout schedule to be changed.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub edit_payout_schedule: bool,
    /// Whether to allow creation of instant payouts.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub instant_payouts: bool,
    /// Whether to allow creation of standard payouts.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub standard_payouts: bool,
}
