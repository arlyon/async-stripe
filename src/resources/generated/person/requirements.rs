#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Requirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<crate::account::requirements::alternative::Alternative>>,
    /// Fields that need to be collected to keep the person's account enabled.
    ///
    /// If not collected by the account's `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Vec<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<crate::account::requirements::errors::Errors>,
    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and the account's `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    /// Fields that weren't collected by the account's `current_deadline`.
    ///
    /// These fields need to be collected to enable the person's account.
    pub past_due: Vec<String>,
    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Requirements {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
