use stripe::{Client, Response};

impl stripe_core::payment_method::PaymentMethod {
    /// Creates a PaymentMethod object.
    ///
    /// Read the [Stripe.js reference](https://stripe.com/docs/stripe-js/reference#stripe-create-payment-method) to learn how to create PaymentMethods via Stripe.js.  Instead of creating a PaymentMethod directly, we recommend using the [PaymentIntents](https://stripe.com/docs/payments/accept-a-payment) API to accept a payment immediately or the [SetupIntent](https://stripe.com/docs/payments/save-and-reuse) API to collect payment method details ahead of a future payment.
    pub fn create(
        client: &Client,
        params: CreatePaymentMethod,
    ) -> Response<stripe_core::payment_method::PaymentMethod> {
        client.send_form("/payment_methods", params, http_types::Method::Post)
    }
    /// Retrieves a PaymentMethod object attached to the StripeAccount.
    ///
    /// To retrieve a payment method attached to a Customer, you should use [Retrieve a Customer’s PaymentMethods](https://stripe.com/docs/api/payment_methods/customer).
    pub fn retrieve(
        client: &Client,
        payment_method: &stripe_core::payment_method::PaymentMethodId,
        params: RetrievePaymentMethod,
    ) -> Response<stripe_core::payment_method::PaymentMethod> {
        client.get_query(
            &format!("/payment_methods/{payment_method}", payment_method = payment_method),
            params,
        )
    }
    /// Updates a PaymentMethod object.
    ///
    /// A PaymentMethod must be attached a customer to be updated.
    pub fn update(
        client: &Client,
        payment_method: &stripe_core::payment_method::PaymentMethodId,
        params: UpdatePaymentMethod,
    ) -> Response<stripe_core::payment_method::PaymentMethod> {
        client.send_form(
            &format!("/payment_methods/{payment_method}", payment_method = payment_method),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of PaymentMethods for Treasury flows.
    ///
    /// If you want to list the PaymentMethods attached to a Customer for payments, you should use the [List a Customer’s PaymentMethods](https://stripe.com/docs/api/payment_methods/customer_list) API instead.
    pub fn list(
        client: &Client,
        params: ListPaymentMethod,
    ) -> Response<stripe_types::List<stripe_core::payment_method::PaymentMethod>> {
        client.get_query("/payment_methods", params)
    }
    /// Attaches a PaymentMethod object to a Customer.
    ///
    /// To attach a new PaymentMethod to a customer for future payments, we recommend you use a [SetupIntent](https://stripe.com/docs/api/setup_intents)
    /// or a PaymentIntent with [setup_future_usage](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-setup_future_usage).
    /// These approaches will perform any necessary steps to set up the PaymentMethod for future payments.
    ///
    /// Using the `/v1/payment_methods/:id/attach` endpoint without first using a SetupIntent or PaymentIntent with `setup_future_usage` does not optimize the PaymentMethod for future use, which makes later declines and payment friction more likely. See [Optimizing cards for future payments](https://stripe.com/docs/payments/payment-intents#future-usage) for more information about setting up future payments.  To use this PaymentMethod as the default for invoice or subscription payments, set <a href="/docs/api/customers/update#update_customer-invoice_settings-default_payment_method">`invoice_settings.default_payment_method`</a>, on the Customer to the PaymentMethod’s ID.
    pub fn attach(
        client: &Client,
        payment_method: &stripe_core::payment_method::PaymentMethodId,
        params: AttachPaymentMethod,
    ) -> Response<stripe_core::payment_method::PaymentMethod> {
        client.send_form(
            &format!("/payment_methods/{payment_method}/attach", payment_method = payment_method),
            params,
            http_types::Method::Post,
        )
    }
    /// Detaches a PaymentMethod object from a Customer.
    ///
    /// After a PaymentMethod is detached, it can no longer be used for a payment or re-attached to a Customer.
    pub fn detach(
        client: &Client,
        payment_method: &stripe_core::payment_method::PaymentMethodId,
        params: DetachPaymentMethod,
    ) -> Response<stripe_core::payment_method::PaymentMethod> {
        client.send_form(
            &format!("/payment_methods/{payment_method}/detach", payment_method = payment_method),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethod<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreatePaymentMethodAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreatePaymentMethodAffirm>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreatePaymentMethodAfterpayClearpay>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreatePaymentMethodAlipay>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreatePaymentMethodAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreatePaymentMethodBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreatePaymentMethodBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreatePaymentMethodBillingDetails<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreatePaymentMethodBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreatePaymentMethodBoleto<'a>>,
    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    ///
    /// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
    /// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
    /// We strongly recommend using Stripe.js instead of interacting with this API directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreatePaymentMethodCard<'a>>,
    /// The `Customer` to whom the original PaymentMethod is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreatePaymentMethodCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreatePaymentMethodEps>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreatePaymentMethodFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreatePaymentMethodGiropay>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreatePaymentMethodGrabpay>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreatePaymentMethodIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<CreatePaymentMethodInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreatePaymentMethodKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreatePaymentMethodKonbini>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreatePaymentMethodLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreatePaymentMethodOxxo>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreatePaymentMethodP24>,
    /// The PaymentMethod to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreatePaymentMethodPaynow>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreatePaymentMethodPix>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreatePaymentMethodPromptpay>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreatePaymentMethodRadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreatePaymentMethodSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreatePaymentMethodSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreatePaymentMethodType>,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreatePaymentMethodUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreatePaymentMethodWechatPay>,
}
impl<'a> CreatePaymentMethod<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> CreatePaymentMethodAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodAffirm {}
impl CreatePaymentMethodAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodAfterpayClearpay {}
impl CreatePaymentMethodAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodAlipay {}
impl CreatePaymentMethodAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> CreatePaymentMethodAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> CreatePaymentMethodBacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodBancontact {}
impl CreatePaymentMethodBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreatePaymentMethodBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreatePaymentMethodBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodBillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreatePaymentMethodBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodBlik {}
impl CreatePaymentMethodBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> CreatePaymentMethodBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
///
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreatePaymentMethodCard<'a> {
    CardDetailsParams(CreatePaymentMethodCardDetailsParams<'a>),
    TokenParams(CreatePaymentMethodTokenParams<'a>),
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
///
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodCardDetailsParams<'a> {
    /// The card's CVC.
    ///
    /// It is highly recommended to always include this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<&'a str>,
    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /// The card number, as a string without any separators.
    pub number: &'a str,
}
impl<'a> CreatePaymentMethodCardDetailsParams<'a> {
    pub fn new(exp_month: i64, exp_year: i64, number: &'a str) -> Self {
        Self { cvc: Default::default(), exp_month, exp_year, number }
    }
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
///
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodTokenParams<'a> {
    pub token: &'a str,
}
impl<'a> CreatePaymentMethodTokenParams<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }
}
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodCustomerBalance {}
impl CreatePaymentMethodCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodEpsBank>,
}
impl CreatePaymentMethodEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl CreatePaymentMethodEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreatePaymentMethodFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: CreatePaymentMethodFpxBank,
}
impl CreatePaymentMethodFpx {
    pub fn new(bank: CreatePaymentMethodFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// Account holder type for FPX transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodFpxAccountHolderType {
    Company,
    Individual,
}

impl CreatePaymentMethodFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl CreatePaymentMethodFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodGiropay {}
impl CreatePaymentMethodGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodGrabpay {}
impl CreatePaymentMethodGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodIdealBank>,
}
impl CreatePaymentMethodIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl CreatePaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodInteracPresent {}
impl CreatePaymentMethodInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreatePaymentMethodKlarnaDob>,
}
impl CreatePaymentMethodKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Customer's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl CreatePaymentMethodKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodKonbini {}
impl CreatePaymentMethodKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodLink {}
impl CreatePaymentMethodLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodOxxo {}
impl CreatePaymentMethodOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreatePaymentMethodP24Bank>,
}
impl CreatePaymentMethodP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl CreatePaymentMethodP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodPaynow {}
impl CreatePaymentMethodPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodPix {}
impl CreatePaymentMethodPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodPromptpay {}
impl CreatePaymentMethodPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> CreatePaymentMethodRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> CreatePaymentMethodSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreatePaymentMethodSofortCountry,
}
impl CreatePaymentMethodSofort {
    pub fn new(country: CreatePaymentMethodSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl CreatePaymentMethodSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl CreatePaymentMethodType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreatePaymentMethodUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreatePaymentMethodUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreatePaymentMethodUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl CreatePaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentMethodUsBankAccountAccountType {
    Checking,
    Savings,
}

impl CreatePaymentMethodUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for CreatePaymentMethodUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentMethodUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreatePaymentMethodWechatPay {}
impl CreatePaymentMethodWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentMethod<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethod<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethod<'a> {
    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdatePaymentMethodAcssDebit>,
    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<UpdatePaymentMethodAffirm>,
    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdatePaymentMethodAuBecsDebit>,
    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdatePaymentMethodBacsDebit>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<UpdatePaymentMethodBillingDetails<'a>>,
    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdatePaymentMethodBlik>,
    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdatePaymentMethodCard>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdatePaymentMethodLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdatePaymentMethodSepaDebit>,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdatePaymentMethodUsBankAccount>,
}
impl<'a> UpdatePaymentMethod<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodAcssDebit {}
impl UpdatePaymentMethodAcssDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodAffirm {}
impl UpdatePaymentMethodAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodAuBecsDebit {}
impl UpdatePaymentMethodAuBecsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodBacsDebit {}
impl UpdatePaymentMethodBacsDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePaymentMethodBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdatePaymentMethodBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodBillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdatePaymentMethodBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodBlik {}
impl UpdatePaymentMethodBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `card` PaymentMethod, this hash contains the user's card details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodCard {
    /// Two-digit number representing the card's expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// Four-digit number representing the card's expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
}
impl UpdatePaymentMethodCard {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodLink {}
impl UpdatePaymentMethodLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodSepaDebit {}
impl UpdatePaymentMethodSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodUsBankAccount {
    /// Bank account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdatePaymentMethodUsBankAccountAccountHolderType>,
}
impl UpdatePaymentMethodUsBankAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Bank account type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentMethodUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl UpdatePaymentMethodUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentMethodUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListPaymentMethod<'a> {
    /// The ID of the customer whose PaymentMethods will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// A required filter on the list, based on the object `type` field.
    #[serde(rename = "type")]
    pub type_: ListPaymentMethodType,
}
impl<'a> ListPaymentMethod<'a> {
    pub fn new(type_: ListPaymentMethodType) -> Self {
        Self {
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            type_,
        }
    }
}
/// A required filter on the list, based on the object `type` field.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListPaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl ListPaymentMethodType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for ListPaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListPaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct AttachPaymentMethod<'a> {
    /// The ID of the customer to which to attach the PaymentMethod.
    pub customer: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> AttachPaymentMethod<'a> {
    pub fn new(customer: &'a str) -> Self {
        Self { customer, expand: Default::default() }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DetachPaymentMethod<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DetachPaymentMethod<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
