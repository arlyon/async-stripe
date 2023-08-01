#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ExternalAccountRequirements {
    /// Fields that need to be collected to keep the external account enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Option<Vec<String>>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Option<Vec<stripe_types::account::requirements::errors::Errors>>,
    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the external account.
    pub past_due: Option<Vec<String>>,
    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Option<Vec<String>>,
}
