use serde::Serialize;

use crate::client::{Client, Response};
use crate::resources::SetupIntent;
use crate::{SetupIntentCancellationReason, SetupIntentId};

/// The set of parameters that can be used when confirming a setup_intent object.
///
/// For more details see <https://stripe.com/docs/api/setup_intents/confirm>
#[derive(Clone, Debug, Serialize)]
pub struct ConfirmSetupIntent {
    /// The client secret if on the client side
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Specifies which payment method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    // Mandate data and payment method options not implemented.  If you want
    // something better, create an issue and lets fix
    /// The URL to redirect your customer back to after they authenticate on the payment method’s app or site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

/// The set of parameters that can be used when canceling a setup_intent object.
///
/// For more details see <https://stripe.com/docs/api/setup_intents/cancel>
#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSetupIntent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<SetupIntentCancellationReason>,
}

impl SetupIntent {
    pub fn confirm(
        client: &Client,
        setup_id: &SetupIntentId,
        params: ConfirmSetupIntent,
    ) -> Response<SetupIntent> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/setup_intents/{}/confirm", setup_id), &params)
    }

    /// A SetupIntent object can be canceled when it is in one of these statuses: requires_payment_method, requires_confirmation, or requires_action.
    ///
    /// For more details see <https://stripe.com/docs/api/setup_intents/cancel>.
    pub fn cancel(
        client: &Client,
        setup_id: &SetupIntentId,
        params: CancelSetupIntent,
    ) -> Response<SetupIntent> {
        client.post_form(&format!("/setup_intents/{}/cancel", setup_id), params)
    }
}

#[cfg(test)]
mod test {}
