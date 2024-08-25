use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveConfirmationTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveConfirmationTokenBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an existing ConfirmationToken object
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveConfirmationToken {
    inner: RetrieveConfirmationTokenBuilder,
    confirmation_token: stripe_payment::ConfirmationTokenId,
}
impl RetrieveConfirmationToken {
    /// Construct a new `RetrieveConfirmationToken`.
    pub fn new(confirmation_token: impl Into<stripe_payment::ConfirmationTokenId>) -> Self {
        Self {
            confirmation_token: confirmation_token.into(),
            inner: RetrieveConfirmationTokenBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveConfirmationToken {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveConfirmationToken {
    type Output = stripe_payment::ConfirmationToken;

    fn build(&self) -> RequestBuilder {
        let confirmation_token = &self.confirmation_token;
        RequestBuilder::new(StripeMethod::Get, format!("/confirmation_tokens/{confirmation_token}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateConfirmationTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method_data: Option<CreateConfirmationTokenPaymentMethodData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_future_usage: Option<stripe_payment::ConfirmationTokenSetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping: Option<CreateConfirmationTokenShipping>,
}
impl CreateConfirmationTokenBuilder {
    fn new() -> Self {
        Self {
            expand: None,
            payment_method: None,
            payment_method_data: None,
            return_url: None,
            setup_future_usage: None,
            shipping: None,
        }
    }
}
/// If provided, this hash will be used to create a PaymentMethod.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodData {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateConfirmationTokenPaymentMethodDataAcssDebit>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub affirm: Option<miniserde::json::Value>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub afterpay_clearpay: Option<miniserde::json::Value>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub alipay: Option<miniserde::json::Value>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<CreateConfirmationTokenPaymentMethodDataAllowRedisplay>,
    /// If this is a AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub amazon_pay: Option<miniserde::json::Value>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateConfirmationTokenPaymentMethodDataAuBecsDebit>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateConfirmationTokenPaymentMethodDataBacsDebit>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub bancontact: Option<miniserde::json::Value>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateConfirmationTokenPaymentMethodDataBillingDetails>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub blik: Option<miniserde::json::Value>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateConfirmationTokenPaymentMethodDataBoleto>,
    /// If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub cashapp: Option<miniserde::json::Value>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub customer_balance: Option<miniserde::json::Value>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateConfirmationTokenPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateConfirmationTokenPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub giropay: Option<miniserde::json::Value>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub grabpay: Option<miniserde::json::Value>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateConfirmationTokenPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub interac_present: Option<miniserde::json::Value>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateConfirmationTokenPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub konbini: Option<miniserde::json::Value>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub link: Option<miniserde::json::Value>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub mobilepay: Option<miniserde::json::Value>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub oxxo: Option<miniserde::json::Value>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateConfirmationTokenPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paynow: Option<miniserde::json::Value>,
    /// If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub paypal: Option<miniserde::json::Value>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub pix: Option<miniserde::json::Value>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub promptpay: Option<miniserde::json::Value>,
    /// Options to configure Radar.
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreateConfirmationTokenPaymentMethodDataRadarOptions>,
    /// If this is a `Revolut Pay` PaymentMethod, this hash contains details about the Revolut Pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub revolut_pay: Option<miniserde::json::Value>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateConfirmationTokenPaymentMethodDataSepaDebit>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateConfirmationTokenPaymentMethodDataSofort>,
    /// If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub swish: Option<miniserde::json::Value>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreateConfirmationTokenPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateConfirmationTokenPaymentMethodDataUsBankAccount>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub wechat_pay: Option<miniserde::json::Value>,
    /// If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "stripe_types::with_serde_json_opt")]
    pub zip: Option<miniserde::json::Value>,
}
impl CreateConfirmationTokenPaymentMethodData {
    pub fn new(type_: impl Into<CreateConfirmationTokenPaymentMethodDataType>) -> Self {
        Self {
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            allow_redisplay: None,
            amazon_pay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            billing_details: None,
            blik: None,
            boleto: None,
            cashapp: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            klarna: None,
            konbini: None,
            link: None,
            metadata: None,
            mobilepay: None,
            oxxo: None,
            p24: None,
            paynow: None,
            paypal: None,
            pix: None,
            promptpay: None,
            radar_options: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            swish: None,
            type_: type_.into(),
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataAcssDebit {
    /// Customer's bank account number.
    pub account_number: String,
    /// Institution number of the customer's bank.
    pub institution_number: String,
    /// Transit number of the customer's bank.
    pub transit_number: String,
}
impl CreateConfirmationTokenPaymentMethodDataAcssDebit {
    pub fn new(
        account_number: impl Into<String>,
        institution_number: impl Into<String>,
        transit_number: impl Into<String>,
    ) -> Self {
        Self {
            account_number: account_number.into(),
            institution_number: institution_number.into(),
            transit_number: transit_number.into(),
        }
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to `unspecified`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateConfirmationTokenPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl CreateConfirmationTokenPaymentMethodDataAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateConfirmationTokenPaymentMethodDataAllowRedisplay",
            )
        })
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataAuBecsDebit {
    /// The account number for the bank account.
    pub account_number: String,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}
impl CreateConfirmationTokenPaymentMethodDataAuBecsDebit {
    pub fn new(account_number: impl Into<String>, bsb_number: impl Into<String>) -> Self {
        Self { account_number: account_number.into(), bsb_number: bsb_number.into() }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataBacsDebit {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Sort code of the bank account. (e.g., `10-20-30`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}
impl CreateConfirmationTokenPaymentMethodDataBacsDebit {
    pub fn new() -> Self {
        Self { account_number: None, sort_code: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataBacsDebit {
    fn default() -> Self {
        Self::new()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateConfirmationTokenPaymentMethodDataBillingDetailsAddress>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CreateConfirmationTokenPaymentMethodDataBillingDetails {
    pub fn new() -> Self {
        Self { address: None, email: None, name: None, phone: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataBillingDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Billing address.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataBillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateConfirmationTokenPaymentMethodDataBillingDetailsAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataBillingDetailsAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)
    pub tax_id: String,
}
impl CreateConfirmationTokenPaymentMethodDataBoleto {
    pub fn new(tax_id: impl Into<String>) -> Self {
        Self { tax_id: tax_id.into() }
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateConfirmationTokenPaymentMethodDataEpsBank>,
}
impl CreateConfirmationTokenPaymentMethodDataEps {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataEps {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateConfirmationTokenPaymentMethodDataEpsBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateConfirmationTokenPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataEpsBank::*;
        match self {
            ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            BankAustria => "bank_austria",
            BankhausCarlSpangler => "bankhaus_carl_spangler",
            BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            BawagPskAg => "bawag_psk_ag",
            BksBankAg => "bks_bank_ag",
            BrullKallmusBankAg => "brull_kallmus_bank_ag",
            BtvVierLanderBank => "btv_vier_lander_bank",
            CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            DeutscheBankAg => "deutsche_bank_ag",
            Dolomitenbank => "dolomitenbank",
            EasybankAg => "easybank_ag",
            ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            HypoTirolBankAg => "hypo_tirol_bank_ag",
            HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            MarchfelderBank => "marchfelder_bank",
            OberbankAg => "oberbank_ag",
            RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            SchoellerbankAg => "schoellerbank_ag",
            SpardaBankWien => "sparda_bank_wien",
            VolksbankGruppe => "volksbank_gruppe",
            VolkskreditbankAg => "volkskreditbank_ag",
            VrBankBraunau => "vr_bank_braunau",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataEpsBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataEpsBank::*;
        match s {
            "arzte_und_apotheker_bank" => Ok(ArzteUndApothekerBank),
            "austrian_anadi_bank_ag" => Ok(AustrianAnadiBankAg),
            "bank_austria" => Ok(BankAustria),
            "bankhaus_carl_spangler" => Ok(BankhausCarlSpangler),
            "bankhaus_schelhammer_und_schattera_ag" => Ok(BankhausSchelhammerUndSchatteraAg),
            "bawag_psk_ag" => Ok(BawagPskAg),
            "bks_bank_ag" => Ok(BksBankAg),
            "brull_kallmus_bank_ag" => Ok(BrullKallmusBankAg),
            "btv_vier_lander_bank" => Ok(BtvVierLanderBank),
            "capital_bank_grawe_gruppe_ag" => Ok(CapitalBankGraweGruppeAg),
            "deutsche_bank_ag" => Ok(DeutscheBankAg),
            "dolomitenbank" => Ok(Dolomitenbank),
            "easybank_ag" => Ok(EasybankAg),
            "erste_bank_und_sparkassen" => Ok(ErsteBankUndSparkassen),
            "hypo_alpeadriabank_international_ag" => Ok(HypoAlpeadriabankInternationalAg),
            "hypo_bank_burgenland_aktiengesellschaft" => Ok(HypoBankBurgenlandAktiengesellschaft),
            "hypo_noe_lb_fur_niederosterreich_u_wien" => Ok(HypoNoeLbFurNiederosterreichUWien),
            "hypo_oberosterreich_salzburg_steiermark" => Ok(HypoOberosterreichSalzburgSteiermark),
            "hypo_tirol_bank_ag" => Ok(HypoTirolBankAg),
            "hypo_vorarlberg_bank_ag" => Ok(HypoVorarlbergBankAg),
            "marchfelder_bank" => Ok(MarchfelderBank),
            "oberbank_ag" => Ok(OberbankAg),
            "raiffeisen_bankengruppe_osterreich" => Ok(RaiffeisenBankengruppeOsterreich),
            "schoellerbank_ag" => Ok(SchoellerbankAg),
            "sparda_bank_wien" => Ok(SpardaBankWien),
            "volksbank_gruppe" => Ok(VolksbankGruppe),
            "volkskreditbank_ag" => Ok(VolkskreditbankAg),
            "vr_bank_braunau" => Ok(VrBankBraunau),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataEpsBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataEpsBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataFpx {
    /// Account holder type for FPX transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: CreateConfirmationTokenPaymentMethodDataFpxBank,
}
impl CreateConfirmationTokenPaymentMethodDataFpx {
    pub fn new(bank: impl Into<CreateConfirmationTokenPaymentMethodDataFpxBank>) -> Self {
        Self { account_holder_type: None, bank: bank.into() }
    }
}
/// Account holder type for FPX transaction
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}
impl CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateConfirmationTokenPaymentMethodDataFpxAccountHolderType",
            )
        })
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateConfirmationTokenPaymentMethodDataFpxBank {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateConfirmationTokenPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataFpxBank::*;
        match self {
            AffinBank => "affin_bank",
            Agrobank => "agrobank",
            AllianceBank => "alliance_bank",
            Ambank => "ambank",
            BankIslam => "bank_islam",
            BankMuamalat => "bank_muamalat",
            BankOfChina => "bank_of_china",
            BankRakyat => "bank_rakyat",
            Bsn => "bsn",
            Cimb => "cimb",
            DeutscheBank => "deutsche_bank",
            HongLeongBank => "hong_leong_bank",
            Hsbc => "hsbc",
            Kfh => "kfh",
            Maybank2e => "maybank2e",
            Maybank2u => "maybank2u",
            Ocbc => "ocbc",
            PbEnterprise => "pb_enterprise",
            PublicBank => "public_bank",
            Rhb => "rhb",
            StandardChartered => "standard_chartered",
            Uob => "uob",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataFpxBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataFpxBank::*;
        match s {
            "affin_bank" => Ok(AffinBank),
            "agrobank" => Ok(Agrobank),
            "alliance_bank" => Ok(AllianceBank),
            "ambank" => Ok(Ambank),
            "bank_islam" => Ok(BankIslam),
            "bank_muamalat" => Ok(BankMuamalat),
            "bank_of_china" => Ok(BankOfChina),
            "bank_rakyat" => Ok(BankRakyat),
            "bsn" => Ok(Bsn),
            "cimb" => Ok(Cimb),
            "deutsche_bank" => Ok(DeutscheBank),
            "hong_leong_bank" => Ok(HongLeongBank),
            "hsbc" => Ok(Hsbc),
            "kfh" => Ok(Kfh),
            "maybank2e" => Ok(Maybank2e),
            "maybank2u" => Ok(Maybank2u),
            "ocbc" => Ok(Ocbc),
            "pb_enterprise" => Ok(PbEnterprise),
            "public_bank" => Ok(PublicBank),
            "rhb" => Ok(Rhb),
            "standard_chartered" => Ok(StandardChartered),
            "uob" => Ok(Uob),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataFpxBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataFpxBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateConfirmationTokenPaymentMethodDataIdealBank>,
}
impl CreateConfirmationTokenPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataIdeal {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateConfirmationTokenPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateConfirmationTokenPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataIdealBank::*;
        match self {
            AbnAmro => "abn_amro",
            AsnBank => "asn_bank",
            Bunq => "bunq",
            Handelsbanken => "handelsbanken",
            Ing => "ing",
            Knab => "knab",
            Moneyou => "moneyou",
            N26 => "n26",
            Nn => "nn",
            Rabobank => "rabobank",
            Regiobank => "regiobank",
            Revolut => "revolut",
            SnsBank => "sns_bank",
            TriodosBank => "triodos_bank",
            VanLanschot => "van_lanschot",
            Yoursafe => "yoursafe",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataIdealBank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataIdealBank::*;
        match s {
            "abn_amro" => Ok(AbnAmro),
            "asn_bank" => Ok(AsnBank),
            "bunq" => Ok(Bunq),
            "handelsbanken" => Ok(Handelsbanken),
            "ing" => Ok(Ing),
            "knab" => Ok(Knab),
            "moneyou" => Ok(Moneyou),
            "n26" => Ok(N26),
            "nn" => Ok(Nn),
            "rabobank" => Ok(Rabobank),
            "regiobank" => Ok(Regiobank),
            "revolut" => Ok(Revolut),
            "sns_bank" => Ok(SnsBank),
            "triodos_bank" => Ok(TriodosBank),
            "van_lanschot" => Ok(VanLanschot),
            "yoursafe" => Ok(Yoursafe),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataIdealBank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataIdealBank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataKlarna {
    /// Customer's date of birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreateConfirmationTokenPaymentMethodDataKlarnaDob>,
}
impl CreateConfirmationTokenPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self { dob: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataKlarna {
    fn default() -> Self {
        Self::new()
    }
}
/// Customer's date of birth
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl CreateConfirmationTokenPaymentMethodDataKlarnaDob {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateConfirmationTokenPaymentMethodDataP24Bank>,
}
impl CreateConfirmationTokenPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self { bank: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataP24 {
    fn default() -> Self {
        Self::new()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateConfirmationTokenPaymentMethodDataP24Bank {
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
    Velobank,
    VolkswagenBank,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateConfirmationTokenPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataP24Bank::*;
        match self {
            AliorBank => "alior_bank",
            BankMillennium => "bank_millennium",
            BankNowyBfgSa => "bank_nowy_bfg_sa",
            BankPekaoSa => "bank_pekao_sa",
            BankiSpbdzielcze => "banki_spbdzielcze",
            Blik => "blik",
            BnpParibas => "bnp_paribas",
            Boz => "boz",
            CitiHandlowy => "citi_handlowy",
            CreditAgricole => "credit_agricole",
            Envelobank => "envelobank",
            EtransferPocztowy24 => "etransfer_pocztowy24",
            GetinBank => "getin_bank",
            Ideabank => "ideabank",
            Ing => "ing",
            Inteligo => "inteligo",
            MbankMtransfer => "mbank_mtransfer",
            NestPrzelew => "nest_przelew",
            NoblePay => "noble_pay",
            PbacZIpko => "pbac_z_ipko",
            PlusBank => "plus_bank",
            SantanderPrzelew24 => "santander_przelew24",
            TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            ToyotaBank => "toyota_bank",
            Velobank => "velobank",
            VolkswagenBank => "volkswagen_bank",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataP24Bank {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataP24Bank::*;
        match s {
            "alior_bank" => Ok(AliorBank),
            "bank_millennium" => Ok(BankMillennium),
            "bank_nowy_bfg_sa" => Ok(BankNowyBfgSa),
            "bank_pekao_sa" => Ok(BankPekaoSa),
            "banki_spbdzielcze" => Ok(BankiSpbdzielcze),
            "blik" => Ok(Blik),
            "bnp_paribas" => Ok(BnpParibas),
            "boz" => Ok(Boz),
            "citi_handlowy" => Ok(CitiHandlowy),
            "credit_agricole" => Ok(CreditAgricole),
            "envelobank" => Ok(Envelobank),
            "etransfer_pocztowy24" => Ok(EtransferPocztowy24),
            "getin_bank" => Ok(GetinBank),
            "ideabank" => Ok(Ideabank),
            "ing" => Ok(Ing),
            "inteligo" => Ok(Inteligo),
            "mbank_mtransfer" => Ok(MbankMtransfer),
            "nest_przelew" => Ok(NestPrzelew),
            "noble_pay" => Ok(NoblePay),
            "pbac_z_ipko" => Ok(PbacZIpko),
            "plus_bank" => Ok(PlusBank),
            "santander_przelew24" => Ok(SantanderPrzelew24),
            "tmobile_usbugi_bankowe" => Ok(TmobileUsbugiBankowe),
            "toyota_bank" => Ok(ToyotaBank),
            "velobank" => Ok(Velobank),
            "volkswagen_bank" => Ok(VolkswagenBank),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataP24Bank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataP24Bank {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Options to configure Radar.
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataRadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}
impl CreateConfirmationTokenPaymentMethodDataRadarOptions {
    pub fn new() -> Self {
        Self { session: None }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataRadarOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataSepaDebit {
    /// IBAN of the bank account.
    pub iban: String,
}
impl CreateConfirmationTokenPaymentMethodDataSepaDebit {
    pub fn new(iban: impl Into<String>) -> Self {
        Self { iban: iban.into() }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreateConfirmationTokenPaymentMethodDataSofortCountry,
}
impl CreateConfirmationTokenPaymentMethodDataSofort {
    pub fn new(country: impl Into<CreateConfirmationTokenPaymentMethodDataSofortCountry>) -> Self {
        Self { country: country.into() }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateConfirmationTokenPaymentMethodDataSofortCountry {
    At,
    Be,
    De,
    Es,
    It,
    Nl,
}
impl CreateConfirmationTokenPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataSofortCountry::*;
        match self {
            At => "AT",
            Be => "BE",
            De => "DE",
            Es => "ES",
            It => "IT",
            Nl => "NL",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataSofortCountry {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataSofortCountry::*;
        match s {
            "AT" => Ok(At),
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "IT" => Ok(It),
            "NL" => Ok(Nl),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataSofortCountry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataSofortCountry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateConfirmationTokenPaymentMethodDataSofortCountry",
            )
        })
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateConfirmationTokenPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Mobilepay,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateConfirmationTokenPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Klarna => "klarna",
            Konbini => "konbini",
            Link => "link",
            Mobilepay => "mobilepay",
            Oxxo => "oxxo",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "mobilepay" => Ok(Mobilepay),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            _ => Ok(Self::Unknown),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateConfirmationTokenPaymentMethodDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl CreateConfirmationTokenPaymentMethodDataUsBankAccount {
    pub fn new() -> Self {
        Self {
            account_holder_type: None,
            account_number: None,
            account_type: None,
            financial_connections_account: None,
            routing_number: None,
        }
    }
}
impl Default for CreateConfirmationTokenPaymentMethodDataUsBankAccount {
    fn default() -> Self {
        Self::new()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}
impl CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType::*;
        match self {
            Company => "company",
            Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType::*;
        match s {
            "company" => Ok(Company),
            "individual" => Ok(Individual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountHolderType"))
    }
}
/// Account type: checkings or savings. Defaults to checking if omitted.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}
impl CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        use CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType::*;
        match self {
            Checking => "checking",
            Savings => "savings",
        }
    }
}

impl std::str::FromStr for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType::*;
        match s {
            "checking" => Ok(Checking),
            "savings" => Ok(Savings),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateConfirmationTokenPaymentMethodDataUsBankAccountAccountType"))
    }
}
/// Shipping information for this ConfirmationToken.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenShipping {
    /// Shipping address
    pub address: CreateConfirmationTokenShippingAddress,
    /// Recipient name.
    pub name: String,
    /// Recipient phone (including extension)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl CreateConfirmationTokenShipping {
    pub fn new(
        address: impl Into<CreateConfirmationTokenShippingAddress>,
        name: impl Into<String>,
    ) -> Self {
        Self { address: address.into(), name: name.into(), phone: None }
    }
}
/// Shipping address
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationTokenShippingAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CreateConfirmationTokenShippingAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateConfirmationTokenShippingAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Creates a test mode Confirmation Token server side for your integration tests.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateConfirmationToken {
    inner: CreateConfirmationTokenBuilder,
}
impl CreateConfirmationToken {
    /// Construct a new `CreateConfirmationToken`.
    pub fn new() -> Self {
        Self { inner: CreateConfirmationTokenBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// ID of an existing PaymentMethod.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.inner.payment_method = Some(payment_method.into());
        self
    }
    /// If provided, this hash will be used to create a PaymentMethod.
    pub fn payment_method_data(
        mut self,
        payment_method_data: impl Into<CreateConfirmationTokenPaymentMethodData>,
    ) -> Self {
        self.inner.payment_method_data = Some(payment_method_data.into());
        self
    }
    /// Return URL used to confirm the Intent.
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.inner.return_url = Some(return_url.into());
        self
    }
    /// Indicates that you intend to make future payments with this ConfirmationToken's payment method.
    ///
    /// The presence of this property will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    pub fn setup_future_usage(
        mut self,
        setup_future_usage: impl Into<stripe_payment::ConfirmationTokenSetupFutureUsage>,
    ) -> Self {
        self.inner.setup_future_usage = Some(setup_future_usage.into());
        self
    }
    /// Shipping information for this ConfirmationToken.
    pub fn shipping(mut self, shipping: impl Into<CreateConfirmationTokenShipping>) -> Self {
        self.inner.shipping = Some(shipping.into());
        self
    }
}
impl Default for CreateConfirmationToken {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateConfirmationToken {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateConfirmationToken {
    type Output = stripe_payment::ConfirmationToken;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/test_helpers/confirmation_tokens")
            .form(&self.inner)
    }
}
