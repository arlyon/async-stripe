use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{AccountId, BankAccountId, CardId, ChargeId, SourceId, TokenId};
use crate::params::{Object, SearchList};
use crate::resources::{Charge, Rule};

/// The set of PaymentSource parameters that can be used to create a charge.
///
/// For more details see <https://stripe.com/docs/api/charges/create#create_charge-source>.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChargeSourceParams {
    Token(TokenId),
    Source(SourceId),
    Card(CardId),
    BankAccount(BankAccountId),
    Account(AccountId),
}

/// The set of parameters that can be used when capturing a charge object.
///
/// For more details see <https://stripe.com/docs/api#charge_capture>.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CaptureCharge<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

impl Charge {
    /// Capture captures a previously created charge with capture set to false.
    ///
    /// For more details see <https://stripe.com/docs/api#charge_capture>.
    pub fn capture(
        client: &Client,
        charge_id: &ChargeId,
        params: CaptureCharge<'_>,
    ) -> Response<Charge> {
        client.post_form(&format!("/charges/{}/capture", charge_id), params)
    }

    /// Searches for a charge.
    ///
    /// For more details see <https://stripe.com/docs/api/charges/search>.
    pub fn search(client: &Client, params: ChargeSearchParams) -> Response<SearchList<Charge>> {
        client.get_query("/charges/search", params)
    }
}

impl Object for Rule {
    type Id = String;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        ""
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChargeSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> ChargeSearchParams<'a> {
    pub fn new() -> ChargeSearchParams<'a> {
        ChargeSearchParams { query: String::new(), limit: None, page: None, expand: &[] }
    }
}
