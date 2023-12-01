// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{AccountId};
use crate::params::{Expand, Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ConnectEmbeddedMethodAccountSessionCreateMethodAccountSession".
///
/// For more details see <https://stripe.com/docs/api/account_sessions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AccountSession {

    /// The ID of the account the AccountSession was created for.
    pub account: String,

    /// The client secret of this AccountSession.
    ///
    /// Used on the client to set up secure access to the given `account`.  The client secret can be used to provide access to `account` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the connected account.
    /// Make sure that you have TLS enabled on any page that includes the client secret.  Refer to our docs to [setup Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components) and learn about how `client_secret` should be handled.
    pub client_secret: String,

    pub components: ConnectEmbeddedAccountSessionCreateComponents,

    /// The timestamp at which this AccountSession will expire.
    pub expires_at: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl AccountSession {

    /// Creates a AccountSession object that includes a single-use token that the platform can use on their front-end to grant client-side API access.
    pub fn create(client: &Client, params: CreateAccountSession<'_>) -> Response<AccountSession> {
        client.post_form("/account_sessions", &params)
    }
}

impl Object for AccountSession {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "account_session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {

    pub account_onboarding: ConnectEmbeddedBaseConfigClaim,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectEmbeddedBaseConfigClaim {

    /// Whether the embedded component is enabled.
    pub enabled: bool,
}

/// The parameters for `AccountSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateAccountSession<'a> {

    /// The identifier of the account to create an Account Session for.
    pub account: AccountId,

    /// Each key of the dictionary represents an embedded component, and each embedded component maps to its configuration (e.g.
    ///
    /// whether it has been enabled or not).
    pub components: CreateAccountSessionComponents,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl<'a> CreateAccountSession<'a> {
    pub fn new(account: AccountId, components: CreateAccountSessionComponents) -> Self {
        CreateAccountSession {
            account,
            components,
            expand: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponents {

    /// Configuration for the account onboarding embedded component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<CreateAccountSessionComponentsAccountOnboarding>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateAccountSessionComponentsAccountOnboarding {

    /// Whether the embedded component is enabled.
    pub enabled: bool,
}
