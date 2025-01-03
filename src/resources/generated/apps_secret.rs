// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{AppsSecretId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SecretServiceResourceSecret".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppsSecret {
    /// Unique identifier for the object.
    pub id: AppsSecretId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// If true, indicates that this secret has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// The Unix timestamp for the expiry time of the secret, after which the secret deletes.
    pub expires_at: Option<Timestamp>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A name for the secret that's unique within the scope.
    pub name: String,

    /// The plaintext secret value to be stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    pub scope: SecretServiceResourceScope,
}

impl Object for AppsSecret {
    type Id = AppsSecretId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "apps.secret"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SecretServiceResourceScope {

    /// The secret scope type.
    #[serde(rename = "type")]
    pub type_: SecretServiceResourceScopeType,

    /// The user ID, if type is set to "user".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// An enum representing the possible values of an `SecretServiceResourceScope`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
}

impl SecretServiceResourceScopeType {
    pub fn as_str(self) -> &'static str {
        match self {
            SecretServiceResourceScopeType::Account => "account",
            SecretServiceResourceScopeType::User => "user",
        }
    }
}

impl AsRef<str> for SecretServiceResourceScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SecretServiceResourceScopeType {
    fn default() -> Self {
        Self::Account
    }
}
