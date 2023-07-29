#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SettingsDashboard {
    /// The display name for this account.
    ///
    /// This is used on the Stripe Dashboard to differentiate between accounts.
    pub display_name: Option<String>,
    /// The timezone used in the Stripe Dashboard for this account.
    ///
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    pub timezone: Option<String>,
}
