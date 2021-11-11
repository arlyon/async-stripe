// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::BankAccountId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Account, BankAccountStatus, Currency, Customer};

/// The resource representing a Stripe "BankAccount".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BankAccount {
    /// Unique identifier for the object.
    pub id: BankAccountId,

    /// The ID of the account that the bank account is associated with.
    pub account: Box<Option<Expandable<Account>>>,

    /// The name of the person or business that owns the bank account.
    pub account_holder_name: Box<Option<String>>,

    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    pub account_holder_type: Box<Option<String>>,

    /// The bank account type.
    ///
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    pub account_type: Box<Option<String>>,

    /// A set of available payout methods for this bank account.
    ///
    /// Only values from this set should be passed as the `method` when creating a payout.
    pub available_payout_methods: Box<Option<Vec<BankAccountAvailablePayoutMethods>>>,

    /// Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    pub bank_name: Box<Option<String>>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Box<Option<String>>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: Currency,

    /// The ID of the customer that the bank account is associated with.
    pub customer: Box<Option<Expandable<Customer>>>,

    /// Whether this bank account is the default external account for its currency.
    pub default_for_currency: Box<Option<bool>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Box<Option<String>>,

    /// The last four digits of the bank account number.
    pub last4: Box<Option<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The routing transit number for the bank account.
    pub routing_number: Box<Option<String>>,

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

/// An enum representing the possible values of an `BankAccount`'s `available_payout_methods` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountAvailablePayoutMethods {
    Instant,
    Standard,
}

impl BankAccountAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        match self {
            BankAccountAvailablePayoutMethods::Instant => "instant",
            BankAccountAvailablePayoutMethods::Standard => "standard",
        }
    }
}

impl AsRef<str> for BankAccountAvailablePayoutMethods {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankAccountAvailablePayoutMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
