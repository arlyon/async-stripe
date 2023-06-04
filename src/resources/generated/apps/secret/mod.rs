/// Secret Store is an API that allows Stripe Apps developers to securely persist secrets for use by UI Extensions and app backends.
///
/// The primary resource in Secret Store is a `secret`.
///
/// Other apps can't view secrets created by an app.
/// Additionally, secrets are scoped to provide further permission control.  All Dashboard users and the app backend share `account` scoped secrets.
/// Use the `account` scope for secrets that don't change per-user, like a third-party API key.  A `user` scoped secret is accessible by the app backend and one specific Dashboard user.
/// Use the `user` scope for per-user secrets like per-user OAuth tokens, where different users might have different permissions.  Related guide: [Store data between page reloads](https://stripe.com/docs/stripe-apps/store-auth-data-custom-objects).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Secret {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// If true, indicates that this secret has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub expires_at: Option<crate::Timestamp>,
    /// Unique identifier for the object.
    pub id: crate::apps::secret::AppsSecretId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A name for the secret that's unique within the scope.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SecretObject,
    /// The plaintext secret value to be stored.
    pub payload: Option<String>,
    pub scope: crate::apps::secret::scope::Scope,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Secret {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SecretObject {
    #[serde(rename = "apps.secret")]
    AppsSecret,
}

impl SecretObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppsSecret => "apps.secret",
        }
    }
}

impl AsRef<str> for SecretObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SecretObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for Secret {
    type Id = crate::apps::secret::AppsSecretId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(AppsSecretId);
pub mod requests;
pub mod scope;
pub use scope::Scope;
