use serde_derive::Serialize;

use crate::client::{Client, Response};
use crate::resources::SetupIntent;
use crate::SetupIntentId;

#[derive(Clone, Debug, Serialize)]
pub struct ConfirmSetupIntent {
    /// The client secret if on the client side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Specifies which payment method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    //Mandate data and payment method options not implemented.  If you want
    //something better, create an issue and lets fix
    /// Where to redirect the user after they log out of their dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CancelSetupIntent {}

impl SetupIntent {
    pub fn confirm(
        client: &Client,
        setup_id: &SetupIntentId,
        params: ConfirmSetupIntent,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/confirm", setup_id), &params)
    }

    pub fn cancel(
        _client: &Client,
        _setup_id: &SetupIntentId,
        _params: CancelSetupIntent,
    ) -> Response<SetupIntent> {
        unimplemented!()
    }

    pub fn retrieve_ext(_client: &Client, _setup_intent_secret: &str) -> Response<SetupIntent> {
        unimplemented!()
        //client.get(&format!("/setup_intents/{}", setup_intent_secret))
    }
}

#[cfg(test)]
mod test {}
