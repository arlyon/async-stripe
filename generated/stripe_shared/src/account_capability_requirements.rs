#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountCapabilityRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    pub alternatives: Option<Vec<stripe_shared::AccountRequirementsAlternative>>,
    /// Date by which the fields in `currently_due` must be collected to keep the capability enabled for the account.
    /// These fields may disable the capability sooner if the next threshold is reached before they are collected.
    pub current_deadline: Option<stripe_types::Timestamp>,
    /// Fields that need to be collected to keep the capability enabled.
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the capability is disabled.
    pub currently_due: Vec<String>,
    /// If the capability is disabled, this string describes why.
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.
    ///
    /// `rejected.unsupported_business` means that the account's business is not supported by the capability.
    /// For example, payment methods may restrict the businesses they support in their terms of service:.
    ///
    /// - [Afterpay Clearpay's terms of service](/afterpay-clearpay/legal#restricted-businesses)
    ///
    /// If you believe that the rejection is in error, please contact support at <https://support.stripe.com/contact/> for assistance.
    pub disabled_reason: Option<String>,
    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<stripe_shared::AccountRequirementsError>,
    /// Fields that need to be collected assuming all volume thresholds are reached.
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Vec<String>,
    /// Fields that weren't collected by `current_deadline`.
    /// These fields need to be collected to enable the capability on the account.
    pub past_due: Vec<String>,
    /// Fields that may become required depending on the results of verification or review.
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}
