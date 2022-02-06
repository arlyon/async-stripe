// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::CapabilityId;
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::Account;
use crate::{AccountRequirementsAlternative, AccountRequirementsError, CapabilityStatus};

/// The resource representing a Stripe "AccountCapability".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Capability {
    /// The identifier for the capability.
    pub id: CapabilityId,

    /// The account for which the capability enables functionality.
    pub account: Expandable<Account>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<Box<AccountCapabilityFutureRequirements>>,

    /// Whether the capability has been requested.
    pub requested: bool,

    /// Time at which the capability was requested.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<Box<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Box<AccountCapabilityRequirements>>,

    /// The status of the capability.
    ///
    /// Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: CapabilityStatus,
}

impl Object for Capability {
    type Id = CapabilityId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "capability"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCapabilityFutureRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Box<Vec<AccountRequirementsAlternative>>>,

    /// Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty.
    ///
    /// After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on the capability's enablement state prior to transitioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<Box<Timestamp>>,

    /// Fields that need to be collected to keep the capability enabled.
    ///
    /// If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Vec<String>,

    /// This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<Box<String>>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well.
    pub eventually_due: Vec<String>,

    /// Fields that weren't collected by `requirements.current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    /// New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCapabilityRequirements {
    /// Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Box<Vec<AccountRequirementsAlternative>>>,

    /// Date by which the fields in `currently_due` must be collected to keep the capability enabled for the account.
    ///
    /// These fields may disable the capability sooner if the next threshold is reached before they are collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<Box<Timestamp>>,

    /// Fields that need to be collected to keep the capability enabled.
    ///
    /// If not collected by `current_deadline`, these fields appear in `past_due` as well, and the capability is disabled.
    pub currently_due: Vec<String>,

    /// If the capability is disabled, this string describes why.
    ///
    /// Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`.  `rejected.unsupported_business` means that the account's business is not supported by the capability.
    /// For example, payment methods may restrict the businesses they support in their terms of service:  - [Afterpay Clearpay's terms of service](/afterpay-clearpay/legal#restricted-businesses)  If you believe that the rejection is in error, please contact support at https://support.stripe.com/contact/ for assistance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<Box<String>>,

    /// Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    pub eventually_due: Vec<String>,

    /// Fields that weren't collected by `current_deadline`.
    ///
    /// These fields need to be collected to enable the capability on the account.
    pub past_due: Vec<String>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// Will be an empty array unless an asynchronous verification is pending.
    /// If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    pub pending_verification: Vec<String>,
}
