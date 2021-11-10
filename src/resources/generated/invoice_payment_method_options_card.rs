// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "invoice_payment_method_options_card".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsCard {
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Box<Option<InvoicePaymentMethodOptionsCardRequestThreeDSecure>>,
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            InvoicePaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
