// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingCreditGrantId};
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{BillingCreditGrantsResourceAmount, Customer, TestHelpersTestClock};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CreditGrant".
///
/// For more details see <https://stripe.com/docs/api/billing/credit-grant/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrant {
    /// Unique identifier for the object.
    pub id: BillingCreditGrantId,

    pub amount: BillingCreditGrantsResourceAmount,

    pub applicability_config: BillingCreditGrantsResourceApplicabilityConfig,

    /// The category of this credit grant.
    ///
    /// This is for tracking purposes and isn't displayed to the customer.
    pub category: BillingCreditGrantCategory,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// ID of the customer receiving the billing credits.
    pub customer: Expandable<Customer>,

    /// The time when the billing credits become effective-when they're eligible for use.
    pub effective_at: Option<Timestamp>,

    /// The time when the billing credits expire.
    ///
    /// If not present, the billing credits don't expire.
    pub expires_at: Option<Timestamp>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// A descriptive name shown in dashboard.
    pub name: Option<String>,

    /// The priority for applying this credit grant.
    ///
    /// The highest priority is 0 and the lowest is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    /// ID of the test clock this credit grant belongs to.
    pub test_clock: Option<Expandable<TestHelpersTestClock>>,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: Timestamp,

    /// The time when this credit grant was voided.
    ///
    /// If not present, the credit grant hasn't been voided.
    pub voided_at: Option<Timestamp>,
}

impl Object for BillingCreditGrant {
    type Id = BillingCreditGrantId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing.credit_grant"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceApplicabilityConfig {

    pub scope: BillingCreditGrantsResourceScope,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceScope {

    /// The price type that credit grants can apply to.
    ///
    /// We currently only support the `metered` price type.
    /// This refers to prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
    /// Cannot be used in combination with `prices`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<BillingCreditGrantsResourceScopePriceType>,

    /// The prices that credit grants can apply to.
    ///
    /// We currently only support `metered` prices.
    /// This refers to prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
    /// Cannot be used in combination with `price_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<BillingCreditGrantsResourceApplicablePrice>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceApplicablePrice {

    /// Unique identifier for the object.
    pub id: Option<String>,
}

/// An enum representing the possible values of an `BillingCreditGrant`'s `category` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingCreditGrantCategory {
    Paid,
    Promotional,
}

impl BillingCreditGrantCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingCreditGrantCategory::Paid => "paid",
            BillingCreditGrantCategory::Promotional => "promotional",
        }
    }
}

impl AsRef<str> for BillingCreditGrantCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingCreditGrantCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingCreditGrantCategory {
    fn default() -> Self {
        Self::Paid
    }
}

/// An enum representing the possible values of an `BillingCreditGrantsResourceScope`'s `price_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingCreditGrantsResourceScopePriceType {
    Metered,
}

impl BillingCreditGrantsResourceScopePriceType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingCreditGrantsResourceScopePriceType::Metered => "metered",
        }
    }
}

impl AsRef<str> for BillingCreditGrantsResourceScopePriceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingCreditGrantsResourceScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingCreditGrantsResourceScopePriceType {
    fn default() -> Self {
        Self::Metered
    }
}
