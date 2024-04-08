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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    /// For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    pub on_behalf_of: Option<String>,
    /// The URL to redirect customers to when they click on the portal's link to return to your website.
    pub return_url: Option<String>,
    /// The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}
impl stripe_types::Object for BillingPortalSession {
    type Id = stripe_billing::BillingPortalSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(BillingPortalSessionId);
#[derive(Copy, Clone, Eq, PartialEq)]
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
    Unknown,
}
impl BillingPortalSessionLocale {
    pub fn as_str(self) -> &'static str {
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for BillingPortalSessionLocale {
    type Err = ();
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
            _ => Err(()),
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
impl<'de> serde::Deserialize<'de> for BillingPortalSessionLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(BillingPortalSessionLocale::Unknown))
    }
}
