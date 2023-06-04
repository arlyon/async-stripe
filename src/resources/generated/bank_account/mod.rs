/// These bank accounts are payment methods on `Customer` objects.
///
/// On the other hand [External Accounts](https://stripe.com/docs/api#external_accounts) are transfer
/// destinations on `Account` objects for [Custom accounts](https://stripe.com/docs/connect/custom-accounts).
/// They can be bank accounts or debit cards as well, and are documented in the links above.
///
/// Related guide: [Bank Debits and Transfers](https://stripe.com/docs/payments/bank-debits-transfers).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BankAccount {
    /// The ID of the account that the bank account is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::Expandable<crate::account::Account>>,
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: Option<String>,
    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    pub account_holder_type: Option<String>,
    /// The bank account type.
    ///
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,
    /// A set of available payout methods for this bank account.
    ///
    /// Only values from this set should be passed as the `method` when creating a payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<BankAccountAvailablePayoutMethods>>,
    /// Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    pub bank_name: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: String,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: crate::Currency,
    /// The ID of the customer that the bank account is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::customer::Customer>>,
    /// Whether this bank account is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Unique identifier for the object.
    pub id: crate::bank_account::BankAccountId,
    /// The last four digits of the bank account number.
    pub last4: String,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::Metadata>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: BankAccountObject,
    /// The routing transit number for the bank account.
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
    pub status: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BankAccount {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// A set of available payout methods for this bank account.
///
/// Only values from this set should be passed as the `method` when creating a payout.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BankAccountAvailablePayoutMethods {
    Instant,
    Standard,
}

impl BankAccountAvailablePayoutMethods {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "instant",
            Self::Standard => "standard",
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum BankAccountObject {
    BankAccount,
}

impl BankAccountObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
        }
    }
}

impl AsRef<str> for BankAccountObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankAccountObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for BankAccount {
    type Id = crate::bank_account::BankAccountId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(BankAccountId, "ba_" | "card_");
pub mod deleted;
pub mod requests;
pub use deleted::DeletedBankAccount;
