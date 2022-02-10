use serde_derive::Serialize;

use crate::config::{Client, Response};
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

pub trait SetupIntentExt {
    fn confirm(
        client: &Client,
        setup_id: &SetupIntentId,
        params: ConfirmSetupIntent,
    ) -> Response<Self>
    where
        Self: Sized;

    #[cfg(features = "idempotency")]
    fn confirm_with_idempotency(
        client: &Client,
        setup_id: &SetupIntentId,
        params: ConfirmSetupIntent,
        idem_key: &str,
    ) -> Response<Self>
    where
        Self: Sized;

    fn cancel(
        client: &Client,
        setup_id: &SetupIntentId,
        params: CancelSetupIntent,
    ) -> Response<Self>
    where
        Self: Sized;

    fn retrieve_ext(client: &Client, setup_intent_secret: &String) -> Response<Self>
    where
        Self: Sized;
}

impl SetupIntentExt for SetupIntent {
    fn confirm(
        client: &Client,
        setup_id: &SetupIntentId,
        params: ConfirmSetupIntent,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/confirm", setup_id), &params, None)
    }

    #[cfg(features = "idempotency")]
    fn confirm_with_idempotency(
        client: &Client,
        setup_id: &SetupIntentId,
        params: ConfirmSetupIntent,
        idem_key: &str,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/confirm", setup_id), &params, Some(idem_key))
    }

    fn cancel(
        _client: &Client,
        _setup_id: &SetupIntentId,
        _params: CancelSetupIntent,
    ) -> Response<SetupIntent> {
        unimplemented!()
    }

    fn retrieve_ext(_client: &Client, _setup_intent_secret: &String) -> Response<SetupIntent> {
        unimplemented!()
        //client.get(&format!("/setup_intents/{}", setup_intent_secret))
    }
}

#[cfg(test)]
mod test {}
