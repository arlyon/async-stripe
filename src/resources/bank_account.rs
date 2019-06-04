// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::BankAccountId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Account, AccountHolderType, Currency, Customer};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "BankAccount".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BankAccount {
    /// Unique identifier for the object.
    pub id: BankAccountId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Expandable<Account>>,

    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,

    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,

    /// Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// Whether this bank account is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<Currency>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The routing transit number for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,

    /// For bank accounts, possible values are `new`, `validated`, `verified`, `verification_failed`, or `errored`.
    ///
    /// A bank account that hasn't had any activity or validation performed is `new`.
    /// If Stripe can determine that the bank account exists, its status will be `validated`.
    /// Note that there often isn’t enough information to know (e.g., for smaller credit unions), and the validation is not always run.
    /// If customer bank account verification has succeeded, the bank account status will be `verified`.
    /// If the verification failed for any reason, such as microdeposit failure, the status will be `verification_failed`.
    /// If a transfer sent to this bank account fails, we'll set the status to `errored` and will not continue to send transfers until the bank details are updated.  For external accounts, possible values are `new` and `errored`.
    /// Validations aren't run against external accounts because they're only used for payouts.
    /// This means the other statuses don't apply.
    /// If a transfer fails, the status is set to `errored` and transfers are stopped until account details are updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BankAccountStatus>,
}

impl Object for BankAccount {
    type Id = BankAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "bank_account"
    }
}

/// An enum representing the possible values of an `BankAccount`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountStatus {
    Errored,
    New,
    Validated,
    VerificationFailed,
    Verified,
}
