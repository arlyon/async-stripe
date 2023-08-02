#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PayoutSchedule {
    /// The number of days charges for the account will be held before being paid out.
    pub delay_days: u32,
    /// How frequently funds will be paid out.
    ///
    /// One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`.
    pub interval: String,
    /// The day of the month funds will be paid out.
    ///
    /// Only shown if `interval` is monthly.
    /// Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,
    /// The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc.
    ///
    /// Only shown if `interval` is weekly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<String>,
}
