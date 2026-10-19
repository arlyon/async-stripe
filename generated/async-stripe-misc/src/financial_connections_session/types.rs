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
    pub bank_account_token: Option<stripe_shared::Token>,
    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: Option<String>,
    pub filters: Option<stripe_misc::BankConnectionsResourceLinkAccountSessionFilters>,
    /// Unique identifier for the object.
    pub id: stripe_misc::FinancialConnectionsSessionId,
    pub limits: Option<stripe_misc::BankConnectionsResourceLinkAccountSessionLimits>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub manual_entry: Option<stripe_misc::BankConnectionsResourceLinkAccountSessionManualEntry>,
    /// Permissions requested for accounts collected during this session.
    pub permissions: Vec<stripe_shared::Permissions>,
    /// Data features requested to be retrieved upon account creation.
    pub prefetch: Option<Vec<stripe_shared::Prefetch>>,
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
    bank_account_token: Option<Option<stripe_shared::Token>>,
    client_secret: Option<Option<String>>,
    filters: Option<Option<stripe_misc::BankConnectionsResourceLinkAccountSessionFilters>>,
    id: Option<stripe_misc::FinancialConnectionsSessionId>,
    limits: Option<Option<stripe_misc::BankConnectionsResourceLinkAccountSessionLimits>>,
    livemode: Option<bool>,
    manual_entry: Option<Option<stripe_misc::BankConnectionsResourceLinkAccountSessionManualEntry>>,
    permissions: Option<Vec<stripe_shared::Permissions>>,
    prefetch: Option<Option<Vec<stripe_shared::Prefetch>>>,
    return_url: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
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
                "bank_account_token" => Deserialize::begin(&mut self.bank_account_token),
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "filters" => Deserialize::begin(&mut self.filters),
                "id" => Deserialize::begin(&mut self.id),
                "limits" => Deserialize::begin(&mut self.limits),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "manual_entry" => Deserialize::begin(&mut self.manual_entry),
                "permissions" => Deserialize::begin(&mut self.permissions),
                "prefetch" => Deserialize::begin(&mut self.prefetch),
                "return_url" => Deserialize::begin(&mut self.return_url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder: Some(None),
                accounts: None,
                bank_account_token: Some(None),
                client_secret: Some(None),
                filters: Some(None),
                id: None,
                limits: Some(None),
                livemode: None,
                manual_entry: Some(None),
                permissions: None,
                prefetch: Some(None),
                return_url: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(account_holder),
                Some(accounts),
                Some(bank_account_token),
                Some(client_secret),
                Some(filters),
                Some(id),
                Some(limits),
                Some(livemode),
                Some(manual_entry),
                Some(permissions),
                Some(prefetch),
                Some(return_url),
            ) = (
                self.account_holder.take(),
                self.accounts.take(),
                self.bank_account_token.take(),
                self.client_secret.take(),
                self.filters.take(),
                self.id.take(),
                self.limits,
                self.livemode,
                self.manual_entry.take(),
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
                bank_account_token,
                client_secret,
                filters,
                id,
                limits,
                livemode,
                manual_entry,
                permissions,
                prefetch,
                return_url,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
                    "bank_account_token" => b.bank_account_token = FromValueOpt::from_value(v),
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "filters" => b.filters = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "limits" => b.limits = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "manual_entry" => b.manual_entry = FromValueOpt::from_value(v),
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
        let mut s = s.serialize_struct("FinancialConnectionsSession", 13)?;
        s.serialize_field("account_holder", &self.account_holder)?;
        s.serialize_field("accounts", &self.accounts)?;
        s.serialize_field("bank_account_token", &self.bank_account_token)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("filters", &self.filters)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("limits", &self.limits)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("manual_entry", &self.manual_entry)?;
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
/// Permissions requested for accounts collected during this session.
pub type FinancialConnectionsSessionPermissions = stripe_shared::Permissions;
/// Data features requested to be retrieved upon account creation.
pub type FinancialConnectionsSessionPrefetch = stripe_shared::Prefetch;
