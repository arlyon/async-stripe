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
    pub created: stripe_types::Timestamp,
    /// If true, indicates that this secret has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_connect::apps::secret::AppsSecretId,
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
    pub scope: stripe_connect::apps::secret::scope::Scope,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SecretObject {
    AppsSecret,
}

impl SecretObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppsSecret => "apps.secret",
        }
    }
}

impl std::str::FromStr for SecretObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "apps.secret" => Ok(Self::AppsSecret),

            _ => Err(()),
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
impl serde::Serialize for SecretObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecretObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SecretObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SecretObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SecretObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SecretObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Secret {
    type Id = stripe_connect::apps::secret::AppsSecretId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(AppsSecretId);
pub mod scope;
pub use scope::Scope;
