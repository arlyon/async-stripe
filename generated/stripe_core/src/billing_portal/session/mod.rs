/// The Billing customer portal is a Stripe-hosted UI for subscription and
/// billing management.
///
/// A portal configuration describes the functionality and features that you
/// want to provide to your customers through the portal.
///
/// A portal session describes the instantiation of the customer portal for
/// a particular customer.
///
/// By visiting the session's URL, the customer can manage their subscriptions and billing details.
/// For security reasons, sessions are short-lived and will expire if the customer does not visit the URL. Create sessions on-demand when customers intend to manage their subscriptions and billing details.  Learn more in the [integration guide](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Session {
    /// The configuration used by this session, describing the features available.
    pub configuration:
        stripe_types::Expandable<stripe_core::billing_portal::configuration::Configuration>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the customer for this session.
    pub customer: String,
    /// Unique identifier for the object.
    pub id: stripe_core::billing_portal::session::BillingPortalSessionId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The IETF language tag of the locale Customer Portal is displayed in.
    ///
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    pub locale: Option<SessionLocale>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SessionObject,
    /// The account for which the session was created on behalf of.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    pub on_behalf_of: Option<String>,
    /// The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: Option<String>,
    /// The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Session {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The IETF language tag of the locale Customer Portal is displayed in.
///
/// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    #[serde(rename = "en-AU")]
    EnMinusAu,
    #[serde(rename = "en-CA")]
    EnMinusCa,
    #[serde(rename = "en-GB")]
    EnMinusGb,
    #[serde(rename = "en-IE")]
    EnMinusIe,
    #[serde(rename = "en-IN")]
    EnMinusIn,
    #[serde(rename = "en-NZ")]
    EnMinusNz,
    #[serde(rename = "en-SG")]
    EnMinusSg,
    Es,
    #[serde(rename = "es-419")]
    EsMinus419,
    Et,
    Fi,
    Fil,
    Fr,
    #[serde(rename = "fr-CA")]
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
    #[serde(rename = "pt-BR")]
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
    #[serde(rename = "zh-HK")]
    ZhMinusHk,
    #[serde(rename = "zh-TW")]
    ZhMinusTw,
}

impl SessionLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Bg => "bg",
            Self::Cs => "cs",
            Self::Da => "da",
            Self::De => "de",
            Self::El => "el",
            Self::En => "en",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIn => "en-IN",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusSg => "en-SG",
            Self::Es => "es",
            Self::EsMinus419 => "es-419",
            Self::Et => "et",
            Self::Fi => "fi",
            Self::Fil => "fil",
            Self::Fr => "fr",
            Self::FrMinusCa => "fr-CA",
            Self::Hr => "hr",
            Self::Hu => "hu",
            Self::Id => "id",
            Self::It => "it",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Lt => "lt",
            Self::Lv => "lv",
            Self::Ms => "ms",
            Self::Mt => "mt",
            Self::Nb => "nb",
            Self::Nl => "nl",
            Self::Pl => "pl",
            Self::Pt => "pt",
            Self::PtMinusBr => "pt-BR",
            Self::Ro => "ro",
            Self::Ru => "ru",
            Self::Sk => "sk",
            Self::Sl => "sl",
            Self::Sv => "sv",
            Self::Th => "th",
            Self::Tr => "tr",
            Self::Vi => "vi",
            Self::Zh => "zh",
            Self::ZhMinusHk => "zh-HK",
            Self::ZhMinusTw => "zh-TW",
        }
    }
}

impl AsRef<str> for SessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
    #[serde(rename = "billing_portal.session")]
    BillingPortalSession,
}

impl SessionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BillingPortalSession => "billing_portal.session",
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
impl stripe_types::Object for Session {
    type Id = stripe_core::billing_portal::session::BillingPortalSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(BillingPortalSessionId, "bps_");
pub mod requests;
