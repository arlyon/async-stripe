#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SettingsPayouts {
    /// A Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account.
    ///
    /// See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details.
    /// Default value is `false` for Custom accounts, otherwise `true`.
    pub debit_negative_balances: bool,
    pub schedule: crate::account::payout_schedule::PayoutSchedule,
    /// The text that appears on the bank account statement for payouts.
    ///
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SettingsPayouts {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}