use crate::params::{Identifiable, Metadata};
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

impl<'a> ::serde::Serialize for BankAccountParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer,
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

/// The resource representing a Stripe bank account.
///
/// For more details see https://stripe.com/docs/api#customer_bank_account_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BankAccount {
    pub id: String,
    pub account_holder_name: String,
    pub account_holder_type: String, // (individual or company)
    pub bank_name: String,
    pub country: String,
    pub currency: Currency,
    pub customer: Option<String>,
    pub fingerprint: String,
    pub last4: String,
    pub metadata: Metadata,
    pub routing_number: String,
    pub status: String, // (new, validated, verified, verification_failed, errored)
}

impl Identifiable for BankAccount {
    fn id(&self) -> &str {
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
