#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<crate::account::requirements::alternative::Alternative>>,
    /// Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty.
    ///
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning.
    pub current_deadline: Option<crate::Timestamp>,
    /// Fields that need to be collected to keep the account enabled.
    ///
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Option<Vec<String>>,
    /// This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    pub disabled_reason: Option<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<crate::account::requirements::errors::Errors>>,
    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Option<Vec<String>>,
    /// Fields that weren't collected by `requirements.current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Option<Vec<String>>,
    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Option<Vec<String>>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FutureRequirements {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
