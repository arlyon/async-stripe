/// A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
///
/// For more details see <<https://stripe.com/docs/api/financial_connections/sessions/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialConnectionsSession {
    /// The account holder for whom accounts are collected in this session.
    pub account_holder: Option<stripe_misc::BankConnectionsResourceAccountholder>,
    /// The accounts that were collected as part of this Session.
    pub accounts: stripe_types::List<stripe_misc::FinancialConnectionsAccount>,
    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: Option<String>,
    pub filters: Option<stripe_misc::BankConnectionsResourceLinkAccountSessionFilters>,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsSessionId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Permissions requested for accounts collected during this session.
    pub permissions: Vec<stripe_misc::FinancialConnectionsSessionPermissions>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<stripe_misc::FinancialConnectionsSessionPrefetch>>,
    /// For webview integrations only.
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    pub return_url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FinancialConnectionsSession").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct FinancialConnectionsSessionBuilder {
    account_holder: Option<Option<stripe_misc::BankConnectionsResourceAccountholder>>,
    accounts: Option<stripe_types::List<stripe_misc::FinancialConnectionsAccount>>,
    client_secret: Option<Option<String>>,
    filters: Option<Option<stripe_misc::BankConnectionsResourceLinkAccountSessionFilters>>,
    id: Option<stripe_misc::FinancialConnectionsSessionId>,
    livemode: Option<bool>,
    permissions: Option<Vec<stripe_misc::FinancialConnectionsSessionPermissions>>,
    prefetch: Option<Option<Vec<stripe_misc::FinancialConnectionsSessionPrefetch>>>,
    return_url: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for FinancialConnectionsSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<FinancialConnectionsSession>,
        builder: FinancialConnectionsSessionBuilder,
    }

    impl Visitor for Place<FinancialConnectionsSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: FinancialConnectionsSessionBuilder {
                    account_holder: Deserialize::default(),
                    accounts: Deserialize::default(),
                    client_secret: Deserialize::default(),
                    filters: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    permissions: Deserialize::default(),
                    prefetch: Deserialize::default(),
                    return_url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder" => Deserialize::begin(&mut self.builder.account_holder),
                "accounts" => Deserialize::begin(&mut self.builder.accounts),
                "client_secret" => Deserialize::begin(&mut self.builder.client_secret),
                "filters" => Deserialize::begin(&mut self.builder.filters),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "permissions" => Deserialize::begin(&mut self.builder.permissions),
                "prefetch" => Deserialize::begin(&mut self.builder.prefetch),
                "return_url" => Deserialize::begin(&mut self.builder.return_url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account_holder),
                Some(accounts),
                Some(client_secret),
                Some(filters),
                Some(id),
                Some(livemode),
                Some(permissions),
                Some(prefetch),
                Some(return_url),
            ) = (
                self.builder.account_holder.take(),
                self.builder.accounts.take(),
                self.builder.client_secret.take(),
                self.builder.filters.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.permissions.take(),
                self.builder.prefetch.take(),
                self.builder.return_url.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(FinancialConnectionsSession {
                account_holder,
                accounts,
                client_secret,
                filters,
                id,
                livemode,
                permissions,
                prefetch,
                return_url,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for FinancialConnectionsSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("FinancialConnectionsSession", 10)?;
        s.serialize_field("account_holder", &self.account_holder)?;
        s.serialize_field("accounts", &self.accounts)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("filters", &self.filters)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("permissions", &self.permissions)?;
        s.serialize_field("prefetch", &self.prefetch)?;
        s.serialize_field("return_url", &self.return_url)?;

        s.serialize_field("object", "financial_connections.session")?;
        s.end()
    }
}
impl stripe_types::Object for FinancialConnectionsSession {
    type Id = stripe_misc::FinancialConnectionsSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(FinancialConnectionsSessionId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsSessionPermissions {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsSessionPermissions::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            PaymentMethod => "payment_method",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsSessionPermissions {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsSessionPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsSessionPermissions"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsSessionPermissions)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for FinancialConnectionsSessionPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsSessionPermissions> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsSessionPermissions::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsSessionPermissions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FinancialConnectionsSessionPrefetch {
    Balances,
    Ownership,
    Transactions,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FinancialConnectionsSessionPrefetch {
    pub fn as_str(&self) -> &str {
        use FinancialConnectionsSessionPrefetch::*;
        match self {
            Balances => "balances",
            Ownership => "ownership",
            Transactions => "transactions",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FinancialConnectionsSessionPrefetch {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsSessionPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "FinancialConnectionsSessionPrefetch"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FinancialConnectionsSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FinancialConnectionsSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FinancialConnectionsSessionPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FinancialConnectionsSessionPrefetch)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for FinancialConnectionsSessionPrefetch {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<FinancialConnectionsSessionPrefetch> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FinancialConnectionsSessionPrefetch::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsSessionPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
