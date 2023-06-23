/// A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Session {
    /// The account holder for whom accounts are collected in this session.
    pub account_holder:
        Option<stripe_misc::financial_connections::account::account_holder::AccountHolder>,
    /// The accounts that were collected as part of this Session.
    pub accounts: stripe_types::List<stripe_misc::financial_connections::account::Account>,
    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<stripe_misc::financial_connections::session::filters::Filters>,
    /// Unique identifier for the object.
    pub id: stripe_misc::financial_connections::session::FinancialConnectionsSessionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SessionObject,
    /// Permissions requested for accounts collected during this session.
    pub permissions: Vec<SessionPermissions>,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Session {
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
pub enum SessionObject {
    #[serde(rename = "financial_connections.session")]
    FinancialConnectionsSession,
}

impl SessionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsSession => "financial_connections.session",
        }
    }
}

impl AsRef<str> for SessionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Permissions requested for accounts collected during this session.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl SessionPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for SessionPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Session {
    type Id = stripe_misc::financial_connections::session::FinancialConnectionsSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsSessionId);
pub mod filters;
pub mod requests;
pub use filters::Filters;
