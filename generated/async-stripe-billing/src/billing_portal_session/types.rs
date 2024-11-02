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
/// Learn more in the [integration guide](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingPortalSession {
    /// The configuration used by this session, describing the features available.
    pub configuration: stripe_types::Expandable<stripe_billing::BillingPortalConfiguration>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the customer for this session.
    pub customer: String,
    /// Information about a specific flow for the customer to go through.
    /// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    pub flow: Option<stripe_billing::PortalFlowsFlow>,
    /// Unique identifier for the object.
    pub id: stripe_billing::BillingPortalSessionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The IETF language tag of the locale Customer Portal is displayed in.
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    pub locale: Option<stripe_billing::BillingPortalSessionLocale>,
    /// The account for which the session was created on behalf of.
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#settlement-merchant).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    pub on_behalf_of: Option<String>,
    /// The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: Option<String>,
    /// The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}
#[doc(hidden)]
pub struct BillingPortalSessionBuilder {
    configuration: Option<stripe_types::Expandable<stripe_billing::BillingPortalConfiguration>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<String>,
    flow: Option<Option<stripe_billing::PortalFlowsFlow>>,
    id: Option<stripe_billing::BillingPortalSessionId>,
    livemode: Option<bool>,
    locale: Option<Option<stripe_billing::BillingPortalSessionLocale>>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                configuration: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                flow: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                locale: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                return_url: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(configuration),
                Some(created),
                Some(customer),
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

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
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
        let mut s = s.serialize_struct("BillingPortalSession", 11)?;
        s.serialize_field("configuration", &self.configuration)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("customer", &self.customer)?;
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingPortalSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnMinusAu,
    EnMinusCa,
    EnMinusGb,
    EnMinusIe,
    EnMinusIn,
    EnMinusNz,
    EnMinusSg,
    Es,
    EsMinus419,
    Et,
    Fi,
    Fil,
    Fr,
    FrMinusCa,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    PtMinusBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhMinusHk,
    ZhMinusTw,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingPortalSessionLocale {
    pub fn as_str(&self) -> &str {
        use BillingPortalSessionLocale::*;
        match self {
            Auto => "auto",
            Bg => "bg",
            Cs => "cs",
            Da => "da",
            De => "de",
            El => "el",
            En => "en",
            EnMinusAu => "en-AU",
            EnMinusCa => "en-CA",
            EnMinusGb => "en-GB",
            EnMinusIe => "en-IE",
            EnMinusIn => "en-IN",
            EnMinusNz => "en-NZ",
            EnMinusSg => "en-SG",
            Es => "es",
            EsMinus419 => "es-419",
            Et => "et",
            Fi => "fi",
            Fil => "fil",
            Fr => "fr",
            FrMinusCa => "fr-CA",
            Hr => "hr",
            Hu => "hu",
            Id => "id",
            It => "it",
            Ja => "ja",
            Ko => "ko",
            Lt => "lt",
            Lv => "lv",
            Ms => "ms",
            Mt => "mt",
            Nb => "nb",
            Nl => "nl",
            Pl => "pl",
            Pt => "pt",
            PtMinusBr => "pt-BR",
            Ro => "ro",
            Ru => "ru",
            Sk => "sk",
            Sl => "sl",
            Sv => "sv",
            Th => "th",
            Tr => "tr",
            Vi => "vi",
            Zh => "zh",
            ZhMinusHk => "zh-HK",
            ZhMinusTw => "zh-TW",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingPortalSessionLocale {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingPortalSessionLocale::*;
        match s {
            "auto" => Ok(Auto),
            "bg" => Ok(Bg),
            "cs" => Ok(Cs),
            "da" => Ok(Da),
            "de" => Ok(De),
            "el" => Ok(El),
            "en" => Ok(En),
            "en-AU" => Ok(EnMinusAu),
            "en-CA" => Ok(EnMinusCa),
            "en-GB" => Ok(EnMinusGb),
            "en-IE" => Ok(EnMinusIe),
            "en-IN" => Ok(EnMinusIn),
            "en-NZ" => Ok(EnMinusNz),
            "en-SG" => Ok(EnMinusSg),
            "es" => Ok(Es),
            "es-419" => Ok(EsMinus419),
            "et" => Ok(Et),
            "fi" => Ok(Fi),
            "fil" => Ok(Fil),
            "fr" => Ok(Fr),
            "fr-CA" => Ok(FrMinusCa),
            "hr" => Ok(Hr),
            "hu" => Ok(Hu),
            "id" => Ok(Id),
            "it" => Ok(It),
            "ja" => Ok(Ja),
            "ko" => Ok(Ko),
            "lt" => Ok(Lt),
            "lv" => Ok(Lv),
            "ms" => Ok(Ms),
            "mt" => Ok(Mt),
            "nb" => Ok(Nb),
            "nl" => Ok(Nl),
            "pl" => Ok(Pl),
            "pt" => Ok(Pt),
            "pt-BR" => Ok(PtMinusBr),
            "ro" => Ok(Ro),
            "ru" => Ok(Ru),
            "sk" => Ok(Sk),
            "sl" => Ok(Sl),
            "sv" => Ok(Sv),
            "th" => Ok(Th),
            "tr" => Ok(Tr),
            "vi" => Ok(Vi),
            "zh" => Ok(Zh),
            "zh-HK" => Ok(ZhMinusHk),
            "zh-TW" => Ok(ZhMinusTw),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for BillingPortalSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingPortalSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for BillingPortalSessionLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BillingPortalSessionLocale {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingPortalSessionLocale> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BillingPortalSessionLocale::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingPortalSessionLocale);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingPortalSessionLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
