/// The Billing customer portal is a Stripe-hosted UI for subscription and
/// billing management.
///
/// A portal configuration describes the functionality and features that you
/// want to provide to your customers through the portal.
///
/// A portal session describes the instantiation of the customer portal for
/// a particular customer. By visiting the session's URL, the customer
/// can manage their subscriptions and billing details. For security reasons,
/// sessions are short-lived and will expire if the customer does not visit the URL.
/// Create sessions on-demand when customers intend to manage their subscriptions
/// and billing details.
///
/// Related guide: [Customer management](/customer-management)
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingPortalSession {
    /// The configuration used by this session, describing the features available.
    pub configuration: stripe_types::Expandable<stripe_billing::BillingPortalConfiguration>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the customer for this session.
    pub customer: String,
    /// The ID of the account for this session.
    pub customer_account: Option<String>,
    /// Information about a specific flow for the customer to go through.
    /// See the [docs](https://docs.stripe.com/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    pub flow: Option<stripe_billing::PortalFlowsFlow>,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingPortalSessionId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The IETF language tag of the locale Customer Portal is displayed in.
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    pub locale: Option<stripe_shared::Locale>,
    /// The account for which the session was created on behalf of.
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://docs.stripe.com/connect/separate-charges-and-transfers#settlement-merchant).
    /// Use the [Accounts API](https://docs.stripe.com/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    pub on_behalf_of: Option<String>,
    /// The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: Option<String>,
    /// The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingPortalSession {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingPortalSession").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingPortalSessionBuilder {
    configuration: Option<stripe_types::Expandable<stripe_billing::BillingPortalConfiguration>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<String>,
    customer_account: Option<Option<String>>,
    flow: Option<Option<stripe_billing::PortalFlowsFlow>>,
    id: Option<stripe_billing::BillingPortalSessionId>,
    livemode: Option<bool>,
    locale: Option<Option<stripe_shared::Locale>>,
    on_behalf_of: Option<Option<String>>,
    return_url: Option<Option<String>>,
    url: Option<String>,
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

    impl Deserialize for BillingPortalSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingPortalSession>,
        builder: BillingPortalSessionBuilder,
    }

    impl Visitor for Place<BillingPortalSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingPortalSessionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingPortalSessionBuilder {
        type Out = BillingPortalSession;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "configuration" => Deserialize::begin(&mut self.configuration),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_account" => Deserialize::begin(&mut self.customer_account),
                "flow" => Deserialize::begin(&mut self.flow),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "locale" => Deserialize::begin(&mut self.locale),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "return_url" => Deserialize::begin(&mut self.return_url),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                configuration: None,
                created: None,
                customer: None,
                customer_account: Some(None),
                flow: Some(None),
                id: None,
                livemode: None,
                locale: Some(None),
                on_behalf_of: Some(None),
                return_url: Some(None),
                url: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(configuration),
                Some(created),
                Some(customer),
                Some(customer_account),
                Some(flow),
                Some(id),
                Some(livemode),
                Some(locale),
                Some(on_behalf_of),
                Some(return_url),
                Some(url),
            ) = (
                self.configuration.take(),
                self.created,
                self.customer.take(),
                self.customer_account.take(),
                self.flow.take(),
                self.id.take(),
                self.livemode,
                self.locale.take(),
                self.on_behalf_of.take(),
                self.return_url.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                configuration,
                created,
                customer,
                customer_account,
                flow,
                id,
                livemode,
                locale,
                on_behalf_of,
                return_url,
                url,
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

    impl ObjectDeser for BillingPortalSession {
        type Builder = BillingPortalSessionBuilder;
    }

    impl FromValueOpt for BillingPortalSession {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingPortalSessionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "configuration" => b.configuration = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "customer_account" => b.customer_account = FromValueOpt::from_value(v),
                    "flow" => b.flow = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "locale" => b.locale = FromValueOpt::from_value(v),
                    "on_behalf_of" => b.on_behalf_of = FromValueOpt::from_value(v),
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingPortalSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("BillingPortalSession", 12)?;
        s.serialize_field("configuration", &self.configuration)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
        s.serialize_field("flow", &self.flow)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("locale", &self.locale)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("return_url", &self.return_url)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "billing_portal.session")?;
        s.end()
    }
}
impl stripe_types::Object for BillingPortalSession {
    type Id = stripe_billing::BillingPortalSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingPortalSessionId);
/// The IETF language tag of the locale Customer Portal is displayed in.
/// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
pub type BillingPortalSessionLocale = stripe_shared::Locale;
