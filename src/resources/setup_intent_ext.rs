use serde::Serialize;

use crate::client::{Client, Response};
use crate::resources::SetupIntent;
use crate::{PaymentMethodId, SetupIntentCancellationReason, SetupIntentId};

/// The set of parameters that can be used when confirming a setup_intent object.
///
/// For more details see <https://stripe.com/docs/api/setup_intents/confirm>
#[derive(Clone, Debug, Serialize)]
pub struct ConfirmSetupIntent {
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// This hash contains details about the mandate to create
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<MandateData>,

    /// When included, this hash creates a PaymentMethod that is set as the payment_method value in the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<crate::UpdatePaymentIntentPaymentMethodData>,

    /// Payment method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<crate::UpdatePaymentIntentPaymentMethodOptions>,

    /// The URL to redirect your customer back to after they authenticate on the payment method’s app or site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next actions.
    pub use_stripe_sdk: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct MandateData {
    pub customer_acceptance: crate::CustomerAcceptance,
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
