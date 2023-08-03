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
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SecretServiceResourceSecretObject,
    /// The plaintext secret value to be stored.
    pub payload: Option<String>,
    pub scope: stripe_connect::SecretServiceResourceScope,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SecretServiceResourceSecretObject {
    AppsSecret,
}

impl SecretServiceResourceSecretObject {
    pub fn as_str(self) -> &'static str {
        use SecretServiceResourceSecretObject::*;
        match self {
            AppsSecret => "apps.secret",
        }
    }
}

impl std::str::FromStr for SecretServiceResourceSecretObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SecretServiceResourceSecretObject::*;
        match s {
            "apps.secret" => Ok(AppsSecret),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for SecretServiceResourceSecretObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SecretServiceResourceSecretObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SecretServiceResourceSecretObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for SecretServiceResourceSecretObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecretServiceResourceSecretObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SecretServiceResourceSecretObject"))
    }
}
impl stripe_types::Object for SecretServiceResourceSecret {
    type Id = stripe_connect::secret_service_resource_secret::AppsSecretId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(AppsSecretId);
pub mod requests;
