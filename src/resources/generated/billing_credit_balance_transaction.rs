// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{BillingCreditBalanceTransactionId};
use crate::params::{Expandable, Object, Timestamp};
use crate::resources::{BillingCreditGrant, BillingCreditGrantsResourceAmount, Invoice, TestHelpersTestClock};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "CreditBalanceTransaction".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditBalanceTransaction {
    /// Unique identifier for the object.
    pub id: BillingCreditBalanceTransactionId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Credit details for this balance transaction.
    ///
    /// Only present if type is `credit`.
    pub credit: Option<BillingCreditGrantsResourceBalanceCredit>,

    /// The credit grant associated with this balance transaction.
    pub credit_grant: Expandable<BillingCreditGrant>,

    /// Debit details for this balance transaction.
    ///
    /// Only present if type is `debit`.
    pub debit: Option<BillingCreditGrantsResourceBalanceDebit>,

    /// The effective time of this balance transaction.
    pub effective_at: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// ID of the test clock this credit balance transaction belongs to.
    pub test_clock: Option<Expandable<TestHelpersTestClock>>,

    /// The type of balance transaction (credit or debit).
    #[serde(rename = "type")]
    pub type_: Option<BillingCreditBalanceTransactionType>,
}

impl Object for BillingCreditBalanceTransaction {
    type Id = BillingCreditBalanceTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing.credit_balance_transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceBalanceCredit {

    pub amount: BillingCreditGrantsResourceAmount,

    /// The type of credit transaction.
    #[serde(rename = "type")]
    pub type_: BillingCreditGrantsResourceBalanceCreditType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceBalanceDebit {

    pub amount: BillingCreditGrantsResourceAmount,

    /// Details of how the credits were applied to an invoice.
    ///
    /// Only present if `type` is `credits_applied`.
    pub credits_applied: Option<BillingCreditGrantsResourceBalanceCreditsApplied>,

    /// The type of debit transaction.
    #[serde(rename = "type")]
    pub type_: BillingCreditGrantsResourceBalanceDebitType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingCreditGrantsResourceBalanceCreditsApplied {

    /// The invoice to which the credits were applied.
    pub invoice: Expandable<Invoice>,

    /// The invoice line item to which the credits were applied.
    pub invoice_line_item: String,
}

/// An enum representing the possible values of an `BillingCreditBalanceTransaction`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingCreditBalanceTransactionType {
    Credit,
    Debit,
}

impl BillingCreditBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingCreditBalanceTransactionType::Credit => "credit",
            BillingCreditBalanceTransactionType::Debit => "debit",
        }
    }
}

impl AsRef<str> for BillingCreditBalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingCreditBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingCreditBalanceTransactionType {
    fn default() -> Self {
        Self::Credit
    }
}

/// An enum representing the possible values of an `BillingCreditGrantsResourceBalanceCredit`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingCreditGrantsResourceBalanceCreditType {
    CreditsGranted,
}

impl BillingCreditGrantsResourceBalanceCreditType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingCreditGrantsResourceBalanceCreditType::CreditsGranted => "credits_granted",
        }
    }
}

impl AsRef<str> for BillingCreditGrantsResourceBalanceCreditType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingCreditGrantsResourceBalanceCreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingCreditGrantsResourceBalanceCreditType {
    fn default() -> Self {
        Self::CreditsGranted
    }
}

/// An enum representing the possible values of an `BillingCreditGrantsResourceBalanceDebit`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingCreditGrantsResourceBalanceDebitType {
    CreditsApplied,
    CreditsExpired,
    CreditsVoided,
}

impl BillingCreditGrantsResourceBalanceDebitType {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingCreditGrantsResourceBalanceDebitType::CreditsApplied => "credits_applied",
            BillingCreditGrantsResourceBalanceDebitType::CreditsExpired => "credits_expired",
            BillingCreditGrantsResourceBalanceDebitType::CreditsVoided => "credits_voided",
        }
    }
}

impl AsRef<str> for BillingCreditGrantsResourceBalanceDebitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingCreditGrantsResourceBalanceDebitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingCreditGrantsResourceBalanceDebitType {
    fn default() -> Self {
        Self::CreditsApplied
    }
}
