use crate::{Client, Response};

impl crate::billing_portal::session::Session {
    /// Creates a session of the customer portal.
    pub fn create(
        client: &Client,
        params: CreateSession,
    ) -> Response<crate::billing_portal::session::Session> {
        client.send_form("/billing_portal/sessions", params, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSession<'a> {
    /// The ID of an existing [configuration](https://stripe.com/docs/api/customer_portal/configuration) to use for this session, describing its functionality and features.
    ///
    /// If not specified, the session uses the default configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<&'a str>,
    /// The ID of an existing customer.
    pub customer: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The IETF language tag of the locale Customer Portal is displayed in.
    ///
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CreateSessionLocale>,
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
impl<'a> CreateSession<'a> {
    pub fn new(customer: &'a str) -> Self {
        Self {
            configuration: Default::default(),
            customer,
            expand: Default::default(),
            locale: Default::default(),
            on_behalf_of: Default::default(),
            return_url: Default::default(),
        }
    }
}
/// The IETF language tag of the locale Customer Portal is displayed in.
///
/// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSessionLocale {
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

impl CreateSessionLocale {
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

impl AsRef<str> for CreateSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
