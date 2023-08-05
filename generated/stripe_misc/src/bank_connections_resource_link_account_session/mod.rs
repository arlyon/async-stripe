/// A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BankConnectionsResourceLinkAccountSession {
    /// The account holder for whom accounts are collected in this session.
    pub account_holder: Option<stripe_misc::BankConnectionsResourceAccountholder>,
    /// The accounts that were collected as part of this Session.
    pub accounts: stripe_types::List<stripe_misc::BankConnectionsResourceLinkedAccount>,
    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<stripe_misc::BankConnectionsResourceLinkAccountSessionFilters>,
    /// Unique identifier for the object.
    pub id:
        stripe_misc::bank_connections_resource_link_account_session::FinancialConnectionsSessionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Permissions requested for accounts collected during this session.
    pub permissions: Vec<BankConnectionsResourceLinkAccountSessionPermissions>,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
/// Permissions requested for accounts collected during this session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceLinkAccountSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl BankConnectionsResourceLinkAccountSessionPermissions {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceLinkAccountSessionPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkAccountSessionPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkAccountSessionPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for BankConnectionsResourceLinkAccountSessionPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceLinkAccountSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BankConnectionsResourceLinkAccountSessionPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkAccountSessionPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for BankConnectionsResourceLinkAccountSessionPermissions",
            )
        })
    }
}
impl stripe_types::Object for BankConnectionsResourceLinkAccountSession {
    type Id =
        stripe_misc::bank_connections_resource_link_account_session::FinancialConnectionsSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(FinancialConnectionsSessionId);
#[cfg(feature = "bank_connections_resource_link_account_session")]
mod requests;
#[cfg(feature = "bank_connections_resource_link_account_session")]
pub use requests::*;
