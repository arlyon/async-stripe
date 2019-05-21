use crate::ids::{BankAccountId, CustomerId};
use crate::params::{Metadata, Paginate};
use crate::resources::Currency;
use serde::ser::SerializeStruct;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct BankAccountParams<'a> {
    pub country: &'a str,
    pub currency: Currency,
    pub account_holder_name: Option<&'a str>,
    pub account_holder_type: Option<&'a str>,
    pub routing_number: Option<&'a str>,
    pub account_number: &'a str,
}

/// The set of parameters that can be used when verifying a Bank Account.
///
/// For more details see https://stripe.com/docs/api/customer_bank_accounts/verify?lang=curl.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BankAccountVerifyParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<&'a str>,
}

impl<'a> serde::Serialize for BankAccountParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = serializer.serialize_struct("BankAccountParams", 6)?;
        s.serialize_field("object", "bank_account")?;
        s.serialize_field("country", &self.country)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("account_holder_name", &self.account_holder_name)?;
        s.serialize_field("routing_number", &self.routing_number)?;
        s.serialize_field("account_number", &self.account_number)?;
        s.end()
    }
}

/// An enum representing the possible values of a `BankAccounts`'s `account_holder_type` field.
///
/// For more details see [https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-account_holder_type](https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-account_holder_type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccountHolderType {
    Individual,
    Company,
}

/// An enum representing the possible values of a `BankAccounts`'s `account_holder_type` field.
///
/// For more details see [https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-status](https://stripe.com/docs/api/customer_bank_accounts/object#customer_bank_account_object-status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BankAccountStatus {
    /// A bank account that hasn’t had any activity or validation performed is new.
    New,
    /// If Stripe can determine that the bank account exists, its status will be validated.
    ///
    /// Note that there often isn’t enough information to know (e.g., for smaller credit unions),
    /// and the validation is not always run.
    Validated,
    /// If customer bank account verification has succeeded, the bank account status will be verified.
    Verified,
    /// If the verification failed for any reason, such as microdeposit failure, the status will be verification_failed.
    VerificationFailed,
    /// If a transfer sent to this bank account fails, we’ll set the status to errored and will not continue to send transfers until the bank details are updated.
    Errored,
}

/// The resource representing a Stripe bank account.
///
/// For more details see [https://stripe.com/docs/api#customer_bank_account_object](https://stripe.com/docs/api#customer_bank_account_object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BankAccount {
    /// Unique identifier for the object.
    pub id: BankAccountId,
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The type of entity that holds the account. This can be either `individual` or `company`.
    pub account_holder_type: AccountHolderType,
    /// Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    pub bank_name: String,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: String, // TODO: Do we want to define an `enum` for country?
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: Currency,
    /// The customer the bank account belongs to.
    pub customer: Option<CustomerId>, // TODO: Expandable
    /// Always true for a deleted object.
    #[serde(default)]
    pub deleted: bool,
    /// Uniquely identifies this particular bank account. You can use this attribute to check whether
    /// two bank accounts are the same.
    pub fingerprint: String,
    pub last4: String,
    /// Set of key-value pairs that you can attach to an object. This can be useful for storing
    /// additional information about the object in a structured format.
    pub metadata: Metadata,
    /// The routing transit number for the bank account
    pub routing_number: String,
    /// For bank accounts, possible values are `New`, `Validated`, `Verified`, `VerificationFailed` or `Errored`.
    ///
    /// For external accounts, possible values are `New` and `Errored`. Validations aren't run
    /// against external accounts because they're only used for payouts. This means the other statuses
    /// don't apply. If a transfer fails, the status is set to `Errored` and transfers are stopped
    /// until account details are updated.
    pub status: BankAccountStatus,
}

impl Paginate for BankAccount {
    fn cursor(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AchCreditTransfer {
    pub account_number: String,
    pub routing_number: String,
    pub fingerprint: String,
    pub bank_name: String,
    pub swift_code: String,
}
