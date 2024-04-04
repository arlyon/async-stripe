/// A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
///
/// For more details see <<https://stripe.com/docs/api/financial_connections/sessions/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FinancialConnectionsSession {
    /// The account holder for whom accounts are collected in this session.
    pub account_holder: Option<stripe_misc::BankConnectionsResourceAccountholder>,
    /// The accounts that were collected as part of this Session.
    pub accounts: stripe_types::List<stripe_misc::FinancialConnectionsAccount>,
    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<stripe_misc::BankConnectionsResourceLinkAccountSessionFilters>,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsSessionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Permissions requested for accounts collected during this session.
    pub permissions: Vec<stripe_misc::FinancialConnectionsSessionPermissions>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<stripe_misc::FinancialConnectionsSessionPrefetch>>,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
impl stripe_types::Object for FinancialConnectionsSession {
    type Id = stripe_misc::FinancialConnectionsSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(FinancialConnectionsSessionId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}
impl FinancialConnectionsSessionPermissions {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsSessionPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsSessionPermissions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsSessionPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsSessionPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsSessionPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsSessionPermissions")
        })
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FinancialConnectionsSessionPrefetch {
    Balances,
    Ownership,
    Transactions,
}
impl FinancialConnectionsSessionPrefetch {
    pub fn as_str(self) -> &'static str {
        use FinancialConnectionsSessionPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
        }
    }
}

impl std::str::FromStr for FinancialConnectionsSessionPrefetch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsSessionPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for FinancialConnectionsSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for FinancialConnectionsSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for FinancialConnectionsSessionPrefetch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FinancialConnectionsSessionPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsSessionPrefetch")
        })
    }
}
