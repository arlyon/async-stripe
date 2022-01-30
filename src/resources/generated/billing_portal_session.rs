// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{BillingPortalSessionId, CustomerId};
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::BillingPortalConfiguration;

/// The resource representing a Stripe "PortalSession".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingPortalSession {
    /// Unique identifier for the object.
    pub id: BillingPortalSessionId,

    /// The configuration used by this session, describing the features available.
    pub configuration: Expandable<BillingPortalConfiguration>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the customer for this session.
    pub customer: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The IETF language tag of the locale Customer Portal is displayed in.
    ///
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<Box<BillingPortalSessionLocale>>,

    /// The account for which the session was created on behalf of.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<String>>,

    /// The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: String,

    /// The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}

impl BillingPortalSession {
    /// Creates a session of the customer portal.
    pub fn create(
        client: &Client,
        params: CreateBillingPortalSession<'_>,
    ) -> Response<BillingPortalSession> {
        client.post_form("/billing_portal/sessions", &params)
    }
}

impl Object for BillingPortalSession {
    type Id = BillingPortalSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "billing_portal.session"
    }
}

/// The parameters for `BillingPortalSession::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateBillingPortalSession<'a> {
    /// The ID of an existing [configuration](https://stripe.com/docs/api/customer_portal/configuration) to use for this session, describing its functionality and features.
    ///
    /// If not specified, the session uses the default configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<&'a str>,

    /// The ID of an existing customer.
    pub customer: CustomerId,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The IETF language tag of the locale Customer Portal is displayed in.
    ///
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<BillingPortalSessionLocale>,

    /// The `on_behalf_of` account to use for this session.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}

impl<'a> CreateBillingPortalSession<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateBillingPortalSession {
            configuration: Default::default(),
            customer,
            expand: Default::default(),
            locale: Default::default(),
            on_behalf_of: Default::default(),
            return_url: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `BillingPortalSession`'s `locale` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingPortalSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-CA")]
    EnCa,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-IE")]
    EnIe,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "en-NZ")]
    EnNz,
    #[serde(rename = "en-SG")]
    EnSg,
    Es,
    #[serde(rename = "es-419")]
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
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
    #[serde(rename = "pt-BR")]
    PtBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    #[serde(rename = "zh-HK")]
    ZhHk,
    #[serde(rename = "zh-TW")]
    ZhTw,
}

impl BillingPortalSessionLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            BillingPortalSessionLocale::Auto => "auto",
            BillingPortalSessionLocale::Bg => "bg",
            BillingPortalSessionLocale::Cs => "cs",
            BillingPortalSessionLocale::Da => "da",
            BillingPortalSessionLocale::De => "de",
            BillingPortalSessionLocale::El => "el",
            BillingPortalSessionLocale::En => "en",
            BillingPortalSessionLocale::EnAu => "en-AU",
            BillingPortalSessionLocale::EnCa => "en-CA",
            BillingPortalSessionLocale::EnGb => "en-GB",
            BillingPortalSessionLocale::EnIe => "en-IE",
            BillingPortalSessionLocale::EnIn => "en-IN",
            BillingPortalSessionLocale::EnNz => "en-NZ",
            BillingPortalSessionLocale::EnSg => "en-SG",
            BillingPortalSessionLocale::Es => "es",
            BillingPortalSessionLocale::Es419 => "es-419",
            BillingPortalSessionLocale::Et => "et",
            BillingPortalSessionLocale::Fi => "fi",
            BillingPortalSessionLocale::Fil => "fil",
            BillingPortalSessionLocale::Fr => "fr",
            BillingPortalSessionLocale::FrCa => "fr-CA",
            BillingPortalSessionLocale::Hr => "hr",
            BillingPortalSessionLocale::Hu => "hu",
            BillingPortalSessionLocale::Id => "id",
            BillingPortalSessionLocale::It => "it",
            BillingPortalSessionLocale::Ja => "ja",
            BillingPortalSessionLocale::Ko => "ko",
            BillingPortalSessionLocale::Lt => "lt",
            BillingPortalSessionLocale::Lv => "lv",
            BillingPortalSessionLocale::Ms => "ms",
            BillingPortalSessionLocale::Mt => "mt",
            BillingPortalSessionLocale::Nb => "nb",
            BillingPortalSessionLocale::Nl => "nl",
            BillingPortalSessionLocale::Pl => "pl",
            BillingPortalSessionLocale::Pt => "pt",
            BillingPortalSessionLocale::PtBr => "pt-BR",
            BillingPortalSessionLocale::Ro => "ro",
            BillingPortalSessionLocale::Ru => "ru",
            BillingPortalSessionLocale::Sk => "sk",
            BillingPortalSessionLocale::Sl => "sl",
            BillingPortalSessionLocale::Sv => "sv",
            BillingPortalSessionLocale::Th => "th",
            BillingPortalSessionLocale::Tr => "tr",
            BillingPortalSessionLocale::Vi => "vi",
            BillingPortalSessionLocale::Zh => "zh",
            BillingPortalSessionLocale::ZhHk => "zh-HK",
            BillingPortalSessionLocale::ZhTw => "zh-TW",
        }
    }
}

impl AsRef<str> for BillingPortalSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BillingPortalSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for BillingPortalSessionLocale {
    fn default() -> Self {
        Self::Auto
    }
}
