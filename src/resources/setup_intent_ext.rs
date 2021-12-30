use serde_derive::Serialize;

use crate::config::{Client, Response};
use crate::resources::SetupIntent;
use crate::SetupIntentId;

#[derive(Clone, Debug, Serialize)]
pub struct ConfirmSetupIntent {
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

    fn cancel(
        client: &Client,
        setup_id: &SetupIntentId,
        params: CancelSetupIntent,
    ) -> Response<Self>
    where
        Self: Sized;

    fn retreive_ext(
        client: &Client,
        setup_intent_secret: &String
    ) -> Response<Self>
        where 
            Self: Sized;
}

impl SetupIntentExt for SetupIntent {
    fn confirm(
        client: &Client,
        setup_id: &SetupIntentId,
        params: ConfirmSetupIntent,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/confirm", setup_id), &params)
    }

    fn cancel(
        _client: &Client,
        _setup_id: &SetupIntentId,
        _params: CancelSetupIntent,
    ) -> Response<SetupIntent> {
        unimplemented!()
    }

    fn retreive_ext(
        client: &Client,
        setup_intent_secret: &String
    ) -> Response<SetupIntent> {
        client.get(&format!("/setup_intents/{}", setup_intent_secret))
    }
}

#[cfg(test)]
mod test {}
