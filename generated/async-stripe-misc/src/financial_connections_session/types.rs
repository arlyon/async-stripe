/// A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
///
/// For more details see <<https://stripe.com/docs/api/financial_connections/sessions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct FinancialConnectionsSession {
    /// The account holder for whom accounts are collected in this session.
    pub account_holder: Option<stripe_misc::BankConnectionsResourceAccountholder>,
    /// The accounts that were collected as part of this Session.
    pub accounts: stripe_types::List<stripe_misc::FinancialConnectionsAccount>,
    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,
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
    pub return_url: Option<String>,
}
#[doc(hidden)]
pub struct FinancialConnectionsSessionBuilder {
    account_holder: Option<Option<stripe_misc::BankConnectionsResourceAccountholder>>,
    accounts: Option<stripe_types::List<stripe_misc::FinancialConnectionsAccount>>,
    client_secret: Option<String>,
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
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: FinancialConnectionsSessionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for FinancialConnectionsSessionBuilder {
        type Out = FinancialConnectionsSession;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder" => Deserialize::begin(&mut self.account_holder),
                "accounts" => Deserialize::begin(&mut self.accounts),
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "filters" => Deserialize::begin(&mut self.filters),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "permissions" => Deserialize::begin(&mut self.permissions),
                "prefetch" => Deserialize::begin(&mut self.prefetch),
                "return_url" => Deserialize::begin(&mut self.return_url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder: Deserialize::default(),
                accounts: Deserialize::default(),
                client_secret: Deserialize::default(),
                filters: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                permissions: Deserialize::default(),
                prefetch: Deserialize::default(),
                return_url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.account_holder.take(),
                self.accounts.take(),
                self.client_secret.take(),
                self.filters.take(),
                self.id.take(),
                self.livemode,
                self.permissions.take(),
                self.prefetch.take(),
                self.return_url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                account_holder,
                accounts,
                client_secret,
                filters,
                id,
                livemode,
                permissions,
                prefetch,
                return_url,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for FinancialConnectionsSession {
        type Builder = FinancialConnectionsSessionBuilder;
    }

    impl FromValueOpt for FinancialConnectionsSession {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = FinancialConnectionsSessionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_holder" => b.account_holder = FromValueOpt::from_value(v),
                    "accounts" => b.accounts = FromValueOpt::from_value(v),
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "filters" => b.filters = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "permissions" => b.permissions = FromValueOpt::from_value(v),
                    "prefetch" => b.prefetch = FromValueOpt::from_value(v),
                    "return_url" => b.return_url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsSessionPermissions::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "payment_method" => Ok(PaymentMethod),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
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
impl miniserde::Deserialize for FinancialConnectionsSessionPermissions {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsSessionPermissions> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            FinancialConnectionsSessionPermissions::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsSessionPermissions);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FinancialConnectionsSessionPrefetch::*;
        match s {
            "balances" => Ok(Balances),
            "ownership" => Ok(Ownership),
            "transactions" => Ok(Transactions),
            _ => Err(stripe_types::StripeParseError),
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
impl miniserde::Deserialize for FinancialConnectionsSessionPrefetch {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FinancialConnectionsSessionPrefetch> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(FinancialConnectionsSessionPrefetch::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FinancialConnectionsSessionPrefetch);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FinancialConnectionsSessionPrefetch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for FinancialConnectionsSessionPrefetch")
        })
    }
}
