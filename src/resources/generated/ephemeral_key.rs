// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CustomerId, EphemeralKeyId, IssuingCardId};
use crate::params::{Deleted, Expand, Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "EphemeralKey".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EphemeralKey {
    /// Unique identifier for the object.
    pub id: EphemeralKeyId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Time at which the key will expire.
    ///
    /// Measured in seconds since the Unix epoch.
    pub expires: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The key's secret.
    ///
    /// You can use this value to make authorized requests to the Stripe API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl EphemeralKey {
    /// Creates a short-lived API key for a given resource.
    pub fn create(client: &Client, params: CreateEphemeralKey<'_>) -> Response<EphemeralKey> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/ephemeral_keys", &params)
    }

    /// Invalidates a short-lived API key for a given resource.
    pub fn delete(client: &Client, id: &EphemeralKeyId) -> Response<Deleted<EphemeralKeyId>> {
        client.delete(&format!("/ephemeral_keys/{}", id))
    }
}

impl Object for EphemeralKey {
    type Id = EphemeralKeyId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "ephemeral_key"
    }
}

/// The parameters for `EphemeralKey::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateEphemeralKey<'a> {
    /// The ID of the Customer you'd like to modify using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The ID of the Issuing Card you'd like to access using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<IssuingCardId>,

    /// A single-use token, created by Stripe.js, used for creating ephemeral keys for Issuing Cards without exchanging sensitive information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<&'a str>,

    /// The ID of the Identity VerificationSession you'd like to access using the resulting ephemeral key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<&'a str>,
}

impl<'a> CreateEphemeralKey<'a> {
    pub fn new() -> Self {
        CreateEphemeralKey {
            customer: Default::default(),
            expand: Default::default(),
            issuing_card: Default::default(),
            nonce: Default::default(),
            verification_session: Default::default(),
        }
    }
}
