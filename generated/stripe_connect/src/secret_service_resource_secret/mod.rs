/// Secret Store is an API that allows Stripe Apps developers to securely persist secrets for use by UI Extensions and app backends.
///
/// The primary resource in Secret Store is a `secret`.
///
/// Other apps can't view secrets created by an app.
/// Additionally, secrets are scoped to provide further permission control.  All Dashboard users and the app backend share `account` scoped secrets.
/// Use the `account` scope for secrets that don't change per-user, like a third-party API key.  A `user` scoped secret is accessible by the app backend and one specific Dashboard user.
/// Use the `user` scope for per-user secrets like per-user OAuth tokens, where different users might have different permissions.  Related guide: [Store data between page reloads](https://stripe.com/docs/stripe-apps/store-auth-data-custom-objects).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecretServiceResourceSecret {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// If true, indicates that this secret has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_connect::secret_service_resource_secret::AppsSecretId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A name for the secret that's unique within the scope.
    pub name: String,
    /// The plaintext secret value to be stored.
    pub payload: Option<String>,
    pub scope: stripe_connect::SecretServiceResourceScope,
}
impl stripe_types::Object for SecretServiceResourceSecret {
    type Id = stripe_connect::secret_service_resource_secret::AppsSecretId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(AppsSecretId);
#[cfg(feature = "secret_service_resource_secret")]
mod requests;
#[cfg(feature = "secret_service_resource_secret")]
pub use requests::*;
