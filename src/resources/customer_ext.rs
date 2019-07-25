use crate::config::{Client, Response};
use crate::ids::{BankAccountId, CardId, CustomerId, PaymentSourceId};
use crate::params::Deleted;
use crate::resources::{BankAccount, Customer, PaymentSource, PaymentSourceParams, Source};
use serde_derive::{Deserialize, Serialize};

impl Customer {
    /// Attaches a source to a customer, does not change default Source for the Customer
    ///
    /// For more details see [https://stripe.com/docs/api#attach_source](https://stripe.com/docs/api#attach_source).
    pub fn attach_source(
        client: &Client,
        customer_id: &CustomerId,
        source: PaymentSourceParams,
    ) -> Response<PaymentSource> {
        #[derive(Serialize)]
        struct AttachSource {
            source: PaymentSourceParams,
        }
        let params = AttachSource { source };
        client.post_form(&format!("/customers/{}/sources", customer_id), params)
    }

    /// Detaches a source from a customer
    ///
    /// For more details see [https://stripe.com/docs/api#detach_source](https://stripe.com/docs/api#detach_source).
    pub fn detach_source(
        client: &Client,
        customer_id: &CustomerId,
        source_id: &PaymentSourceId,
    ) -> Response<DetachedSource> {
        client.delete(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Retrieves a Card, BankAccount, or Source for a Customer
    pub fn retrieve_source(
        client: &Client,
        customer_id: &CustomerId,
        source_id: &PaymentSourceId,
    ) -> Response<PaymentSource> {
        client.get(&format!("/customers/{}/sources/{}", customer_id, source_id))
    }

    /// Verifies a Bank Account for a Customer.
    ///
    /// For more details see https://stripe.com/docs/api/customer_bank_accounts/verify.
    pub fn verify_bank_account(
        client: &Client,
        customer_id: &CustomerId,
        bank_account_id: &BankAccountId,
        params: VerifyBankAccount<'_>,
    ) -> Response<BankAccount> {
        client.post_form(
            &format!("/customers/{}/sources/{}/verify", customer_id, bank_account_id),
            params,
        )
    }
}

/// The set of parameters that can be used when verifying a Bank Account.
///
/// For more details see https://stripe.com/docs/api/customer_bank_accounts/verify?lang=curl.
#[derive(Clone, Debug, Default, Serialize)]
pub struct VerifyBankAccount<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<&'a str>,
}

impl VerifyBankAccount<'_> {
    pub fn new() -> Self {
        VerifyBankAccount { amounts: None, verification_method: None }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum DetachedSource {
    BankAccount(Deleted<BankAccountId>),
    Card(Deleted<CardId>),
    Source(Source),
}
