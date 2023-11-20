/// An AccountSession allows a Connect platform to grant access to a connected account in Connect embedded components.
///
/// We recommend that you create an AccountSession each time you need to display an embedded component
/// to your user.
///
/// Do not save AccountSessions to your database as they expire relatively quickly, and cannot be used more than once.  Related guide: [Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConnectEmbeddedMethodAccountSessionCreateMethodAccountSession {
    /// The ID of the account the AccountSession was created for.
    pub account: String,
    /// The client secret of this AccountSession.
    ///
    /// Used on the client to set up secure access to the given `account`.  The client secret can be used to provide access to `account` from your frontend.
    /// It should not be stored, logged, or exposed to anyone other than the connected account.
    /// Make sure that you have TLS enabled on any page that includes the client secret.  Refer to our docs to [setup Connect embedded components](https://stripe.com/docs/connect/get-started-connect-embedded-components) and learn about how `client_secret` should be handled.
    pub client_secret: String,
    pub components: stripe_connect::ConnectEmbeddedAccountSessionCreateComponents,
    /// The timestamp at which this AccountSession will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
#[cfg(feature = "connect_embedded_method_account_session_create_method_account_session")]
mod requests;
#[cfg(feature = "connect_embedded_method_account_session_create_method_account_session")]
pub use requests::*;
