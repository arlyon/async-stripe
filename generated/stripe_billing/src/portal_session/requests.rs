#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSession<'a> {
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
    /// Information about a specific flow for the customer to go through.
    ///
    /// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_data: Option<CreatePortalSessionFlowData<'a>>,
    /// The IETF language tag of the locale Customer Portal is displayed in.
    ///
    /// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CreatePortalSessionLocale>,
    /// The `on_behalf_of` account to use for this session.
    ///
    /// When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal.
    /// For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#on-behalf-of).
    /// Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreatePortalSession<'a> {
    pub fn new(customer: &'a str) -> Self {
        Self {
            configuration: Default::default(),
            customer,
            expand: Default::default(),
            flow_data: Default::default(),
            locale: Default::default(),
            on_behalf_of: Default::default(),
            return_url: Default::default(),
        }
    }
}
/// Information about a specific flow for the customer to go through.
///
/// See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSessionFlowData<'a> {
    /// Behavior after the flow is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<CreatePortalSessionFlowDataAfterCompletion<'a>>,
    /// Configuration when `flow_data.type=subscription_cancel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<CreatePortalSessionFlowDataSubscriptionCancel<'a>>,
    /// Configuration when `flow_data.type=subscription_update`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<CreatePortalSessionFlowDataSubscriptionUpdate<'a>>,
    /// Configuration when `flow_data.type=subscription_update_confirm`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update_confirm:
        Option<CreatePortalSessionFlowDataSubscriptionUpdateConfirm<'a>>,
    /// Type of flow that the customer will go through.
    #[serde(rename = "type")]
    pub type_: CreatePortalSessionFlowDataType,
}
impl<'a> CreatePortalSessionFlowData<'a> {
    pub fn new(type_: CreatePortalSessionFlowDataType) -> Self {
        Self {
            after_completion: Default::default(),
            subscription_cancel: Default::default(),
            subscription_update: Default::default(),
            subscription_update_confirm: Default::default(),
            type_,
        }
    }
}
/// Behavior after the flow is completed.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSessionFlowDataAfterCompletion<'a> {
    /// Configuration when `after_completion.type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation:
        Option<CreatePortalSessionFlowDataAfterCompletionHostedConfirmation<'a>>,
    /// Configuration when `after_completion.type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreatePortalSessionFlowDataAfterCompletionRedirect<'a>>,
    /// The specified behavior after the flow is completed.
    #[serde(rename = "type")]
    pub type_: CreatePortalSessionFlowDataAfterCompletionType,
}
impl<'a> CreatePortalSessionFlowDataAfterCompletion<'a> {
    pub fn new(type_: CreatePortalSessionFlowDataAfterCompletionType) -> Self {
        Self { hosted_confirmation: Default::default(), redirect: Default::default(), type_ }
    }
}
/// Configuration when `after_completion.type=hosted_confirmation`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePortalSessionFlowDataAfterCompletionHostedConfirmation<'a> {
    /// A custom message to display to the customer after the flow is completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<&'a str>,
}
impl<'a> CreatePortalSessionFlowDataAfterCompletionHostedConfirmation<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration when `after_completion.type=redirect`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSessionFlowDataAfterCompletionRedirect<'a> {
    /// The URL the customer will be redirected to after the flow is completed.
    pub return_url: &'a str,
}
impl<'a> CreatePortalSessionFlowDataAfterCompletionRedirect<'a> {
    pub fn new(return_url: &'a str) -> Self {
        Self { return_url }
    }
}
/// The specified behavior after the flow is completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePortalSessionFlowDataAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
}

impl CreatePortalSessionFlowDataAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        use CreatePortalSessionFlowDataAfterCompletionType::*;
        match self {
            HostedConfirmation => "hosted_confirmation",
            PortalHomepage => "portal_homepage",
            Redirect => "redirect",
        }
    }
}

impl std::str::FromStr for CreatePortalSessionFlowDataAfterCompletionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePortalSessionFlowDataAfterCompletionType::*;
        match s {
            "hosted_confirmation" => Ok(HostedConfirmation),
            "portal_homepage" => Ok(PortalHomepage),
            "redirect" => Ok(Redirect),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePortalSessionFlowDataAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePortalSessionFlowDataAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePortalSessionFlowDataAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePortalSessionFlowDataAfterCompletionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Configuration when `flow_data.type=subscription_cancel`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSessionFlowDataSubscriptionCancel<'a> {
    /// The ID of the subscription to be canceled.
    pub subscription: &'a str,
}
impl<'a> CreatePortalSessionFlowDataSubscriptionCancel<'a> {
    pub fn new(subscription: &'a str) -> Self {
        Self { subscription }
    }
}
/// Configuration when `flow_data.type=subscription_update`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSessionFlowDataSubscriptionUpdate<'a> {
    /// The ID of the subscription to be updated.
    pub subscription: &'a str,
}
impl<'a> CreatePortalSessionFlowDataSubscriptionUpdate<'a> {
    pub fn new(subscription: &'a str) -> Self {
        Self { subscription }
    }
}
/// Configuration when `flow_data.type=subscription_update_confirm`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSessionFlowDataSubscriptionUpdateConfirm<'a> {
    /// The coupon or promotion code to apply to this subscription update.
    ///
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [CreatePortalSessionFlowDataSubscriptionUpdateConfirmDiscounts<'a>]>,
    /// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
    ///
    /// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
    pub items: &'a [CreatePortalSessionFlowDataSubscriptionUpdateConfirmItems<'a>],
    /// The ID of the subscription to be updated.
    pub subscription: &'a str,
}
impl<'a> CreatePortalSessionFlowDataSubscriptionUpdateConfirm<'a> {
    pub fn new(
        items: &'a [CreatePortalSessionFlowDataSubscriptionUpdateConfirmItems<'a>],
        subscription: &'a str,
    ) -> Self {
        Self { discounts: Default::default(), items, subscription }
    }
}
/// The coupon or promotion code to apply to this subscription update.
///
/// Currently, only up to one may be specified.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePortalSessionFlowDataSubscriptionUpdateConfirmDiscounts<'a> {
    /// The ID of the coupon to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// The ID of a promotion code to apply to this subscription update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> CreatePortalSessionFlowDataSubscriptionUpdateConfirmDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The [subscription item](https://stripe.com/docs/api/subscription_items) to be updated through this flow.
///
/// Currently, only up to one may be specified and subscriptions with multiple items are not updatable.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePortalSessionFlowDataSubscriptionUpdateConfirmItems<'a> {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: &'a str,
    /// The price the customer should subscribe to through this flow.
    ///
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
impl<'a> CreatePortalSessionFlowDataSubscriptionUpdateConfirmItems<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id, price: Default::default(), quantity: Default::default() }
    }
}
/// Type of flow that the customer will go through.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePortalSessionFlowDataType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
}

impl CreatePortalSessionFlowDataType {
    pub fn as_str(self) -> &'static str {
        use CreatePortalSessionFlowDataType::*;
        match self {
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
        }
    }
}

impl std::str::FromStr for CreatePortalSessionFlowDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePortalSessionFlowDataType::*;
        match s {
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreatePortalSessionFlowDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePortalSessionFlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePortalSessionFlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePortalSessionFlowDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The IETF language tag of the locale Customer Portal is displayed in.
///
/// If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreatePortalSessionLocale {
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
}

impl CreatePortalSessionLocale {
    pub fn as_str(self) -> &'static str {
        use CreatePortalSessionLocale::*;
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
        }
    }
}

impl std::str::FromStr for CreatePortalSessionLocale {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePortalSessionLocale::*;
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

impl AsRef<str> for CreatePortalSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePortalSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreatePortalSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreatePortalSessionLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreatePortalSession<'a> {
    /// Creates a session of the customer portal.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_billing::PortalSession> {
        client.send_form("/billing_portal/sessions", self, http_types::Method::Post)
    }
}
