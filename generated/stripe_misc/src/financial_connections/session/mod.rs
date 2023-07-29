/// A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionObject {
    FinancialConnectionsSession,
}

impl SessionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialConnectionsSession => "financial_connections.session",
        }
    }
}

impl std::str::FromStr for SessionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "financial_connections.session" => Ok(Self::FinancialConnectionsSession),

            _ => Err(()),
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
impl serde::Serialize for SessionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SessionObject"))
    }
}
/// Permissions requested for accounts collected during this session.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for SessionPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "balances" => Ok(Self::Balances),
            "ownership" => Ok(Self::Ownership),
            "payment_method" => Ok(Self::PaymentMethod),
            "transactions" => Ok(Self::Transactions),

            _ => Err(()),
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
impl serde::Serialize for SessionPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SessionPermissions"))
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
pub use filters::Filters;
