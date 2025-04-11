// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ConfirmationTokenId};
use crate::params::{Expand, Expandable, Object, Timestamp};
use crate::resources::{Address, BillingDetails, CardDetails, CardPresent, Customer, PaymentFlowsPrivatePaymentMethodsAlipay, PaymentMethodAcssDebit, PaymentMethodAffirm, PaymentMethodAfterpayClearpay, PaymentMethodAlma, PaymentMethodAmazonPay, PaymentMethodAuBecsDebit, PaymentMethodBacsDebit, PaymentMethodBancontact, PaymentMethodBillie, PaymentMethodBlik, PaymentMethodBoleto, PaymentMethodCashapp, PaymentMethodCustomerBalance, PaymentMethodEps, PaymentMethodFpx, PaymentMethodGiropay, PaymentMethodGrabpay, PaymentMethodIdeal, PaymentMethodInteracPresent, PaymentMethodKakaoPay, PaymentMethodKlarna, PaymentMethodKonbini, PaymentMethodKrCard, PaymentMethodLink, PaymentMethodMobilepay, PaymentMethodMultibanco, PaymentMethodNaverPay, PaymentMethodNzBankAccount, PaymentMethodOxxo, PaymentMethodP24, PaymentMethodPayByBank, PaymentMethodPayco, PaymentMethodPaynow, PaymentMethodPaypal, PaymentMethodPix, PaymentMethodPromptpay, PaymentMethodRevolutPay, PaymentMethodSamsungPay, PaymentMethodSatispay, PaymentMethodSepaDebit, PaymentMethodSofort, PaymentMethodSwish, PaymentMethodTwint, PaymentMethodUsBankAccount, PaymentMethodWechatPay, PaymentMethodZip};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ConfirmationTokensResourceConfirmationToken".
///
/// For more details see <https://stripe.com/docs/api/confirmation_tokens/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationToken {
    /// Unique identifier for the object.
    pub id: ConfirmationTokenId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Time at which this ConfirmationToken expires and can no longer be used to confirm a PaymentIntent or SetupIntent.
    pub expires_at: Option<Timestamp>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Data used for generating a Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<ConfirmationTokensResourceMandateData>,

    /// ID of the PaymentIntent that this ConfirmationToken was used to confirm, or null if this ConfirmationToken has not yet been used.
    pub payment_intent: Option<String>,

    /// Payment-method-specific configuration for this ConfirmationToken.
    pub payment_method_options: Option<ConfirmationTokensResourcePaymentMethodOptions>,

    /// Payment details collected by the Payment Element, used to create a PaymentMethod when a PaymentIntent or SetupIntent is confirmed with this ConfirmationToken.
    pub payment_method_preview: Option<ConfirmationTokensResourcePaymentMethodPreview>,

    /// Return URL used to confirm the Intent.
    pub return_url: Option<String>,

    /// Indicates that you intend to make future payments with this ConfirmationToken's payment method.
    ///
    /// The presence of this property will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    pub setup_future_usage: Option<ConfirmationTokenSetupFutureUsage>,

    /// ID of the SetupIntent that this ConfirmationToken was used to confirm, or null if this ConfirmationToken has not yet been used.
    pub setup_intent: Option<String>,

    /// Shipping information collected on this ConfirmationToken.
    pub shipping: Option<ConfirmationTokensResourceShipping>,

    /// Indicates whether the Stripe SDK is used to handle confirmation flow.
    ///
    /// Defaults to `true` on ConfirmationToken.
    pub use_stripe_sdk: bool,
}

impl ConfirmationToken {

    /// Retrieves an existing ConfirmationToken object.
    pub fn retrieve(client: &Client, id: &ConfirmationTokenId, expand: &[&str]) -> Response<ConfirmationToken> {
        client.get_query(&format!("/confirmation_tokens/{}", id), Expand { expand })
    }
}

impl Object for ConfirmationToken {
    type Id = ConfirmationTokenId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "confirmation_token"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationTokensResourceMandateData {

    pub customer_acceptance: ConfirmationTokensResourceMandateDataResourceCustomerAcceptance,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {

    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    pub online: Option<ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>,

    /// The type of customer acceptance information included with the Mandate.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline {

    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: Option<String>,

    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationTokensResourcePaymentMethodOptions {

    /// This hash contains the card payment method options.
    pub card: Option<ConfirmationTokensResourcePaymentMethodOptionsResourceCard>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationTokensResourcePaymentMethodOptionsResourceCard {

    /// The `cvc_update` Token collected from the Payment Element.
    pub cvc_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationTokensResourcePaymentMethodPreview {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodAffirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipay>,

    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    ///
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to “unspecified”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma: Option<PaymentMethodAlma>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<PaymentMethodAmazonPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie: Option<PaymentMethodBillie>,

    pub billing_details: BillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodCashapp>,

    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<Expandable<Customer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodEps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodFpx>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodGrabpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PaymentMethodInteracPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<PaymentMethodKakaoPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<PaymentMethodKrCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<PaymentMethodMobilepay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<PaymentMethodMultibanco>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<PaymentMethodNaverPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<PaymentMethodNzBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<PaymentMethodPayByBank>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco: Option<PaymentMethodPayco>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodPaynow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodPaypal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<PaymentMethodPix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodPromptpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodRevolutPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<PaymentMethodSamsungPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay: Option<PaymentMethodSatispay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<PaymentMethodSwish>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint: Option<PaymentMethodTwint>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: ConfirmationTokensResourcePaymentMethodPreviewType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodUsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodWechatPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<PaymentMethodZip>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfirmationTokensResourceShipping {

    pub address: Address,

    /// Recipient name.
    pub name: String,

    /// Recipient phone (including extension).
    pub phone: Option<String>,
}

/// An enum representing the possible values of an `ConfirmationToken`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationTokenSetupFutureUsage {
    OffSession,
    OnSession,
}

impl ConfirmationTokenSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            ConfirmationTokenSetupFutureUsage::OffSession => "off_session",
            ConfirmationTokenSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for ConfirmationTokenSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmationTokenSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ConfirmationTokenSetupFutureUsage {
    fn default() -> Self {
        Self::OffSession
    }
}

/// An enum representing the possible values of an `ConfirmationTokensResourcePaymentMethodPreview`'s `allow_redisplay` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}

impl ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Always => "always",
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Limited => "limited",
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `ConfirmationTokensResourcePaymentMethodPreview`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationTokensResourcePaymentMethodPreviewType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
    UsBankAccount,
    WechatPay,
    Zip,
}

impl ConfirmationTokensResourcePaymentMethodPreviewType {
    pub fn as_str(self) -> &'static str {
        match self {
            ConfirmationTokensResourcePaymentMethodPreviewType::AcssDebit => "acss_debit",
            ConfirmationTokensResourcePaymentMethodPreviewType::Affirm => "affirm",
            ConfirmationTokensResourcePaymentMethodPreviewType::AfterpayClearpay => "afterpay_clearpay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Alipay => "alipay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Alma => "alma",
            ConfirmationTokensResourcePaymentMethodPreviewType::AmazonPay => "amazon_pay",
            ConfirmationTokensResourcePaymentMethodPreviewType::AuBecsDebit => "au_becs_debit",
            ConfirmationTokensResourcePaymentMethodPreviewType::BacsDebit => "bacs_debit",
            ConfirmationTokensResourcePaymentMethodPreviewType::Bancontact => "bancontact",
            ConfirmationTokensResourcePaymentMethodPreviewType::Billie => "billie",
            ConfirmationTokensResourcePaymentMethodPreviewType::Blik => "blik",
            ConfirmationTokensResourcePaymentMethodPreviewType::Boleto => "boleto",
            ConfirmationTokensResourcePaymentMethodPreviewType::Card => "card",
            ConfirmationTokensResourcePaymentMethodPreviewType::CardPresent => "card_present",
            ConfirmationTokensResourcePaymentMethodPreviewType::Cashapp => "cashapp",
            ConfirmationTokensResourcePaymentMethodPreviewType::CustomerBalance => "customer_balance",
            ConfirmationTokensResourcePaymentMethodPreviewType::Eps => "eps",
            ConfirmationTokensResourcePaymentMethodPreviewType::Fpx => "fpx",
            ConfirmationTokensResourcePaymentMethodPreviewType::Giropay => "giropay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Grabpay => "grabpay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Ideal => "ideal",
            ConfirmationTokensResourcePaymentMethodPreviewType::InteracPresent => "interac_present",
            ConfirmationTokensResourcePaymentMethodPreviewType::KakaoPay => "kakao_pay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Klarna => "klarna",
            ConfirmationTokensResourcePaymentMethodPreviewType::Konbini => "konbini",
            ConfirmationTokensResourcePaymentMethodPreviewType::KrCard => "kr_card",
            ConfirmationTokensResourcePaymentMethodPreviewType::Link => "link",
            ConfirmationTokensResourcePaymentMethodPreviewType::Mobilepay => "mobilepay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Multibanco => "multibanco",
            ConfirmationTokensResourcePaymentMethodPreviewType::NaverPay => "naver_pay",
            ConfirmationTokensResourcePaymentMethodPreviewType::NzBankAccount => "nz_bank_account",
            ConfirmationTokensResourcePaymentMethodPreviewType::Oxxo => "oxxo",
            ConfirmationTokensResourcePaymentMethodPreviewType::P24 => "p24",
            ConfirmationTokensResourcePaymentMethodPreviewType::PayByBank => "pay_by_bank",
            ConfirmationTokensResourcePaymentMethodPreviewType::Payco => "payco",
            ConfirmationTokensResourcePaymentMethodPreviewType::Paynow => "paynow",
            ConfirmationTokensResourcePaymentMethodPreviewType::Paypal => "paypal",
            ConfirmationTokensResourcePaymentMethodPreviewType::Pix => "pix",
            ConfirmationTokensResourcePaymentMethodPreviewType::Promptpay => "promptpay",
            ConfirmationTokensResourcePaymentMethodPreviewType::RevolutPay => "revolut_pay",
            ConfirmationTokensResourcePaymentMethodPreviewType::SamsungPay => "samsung_pay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Satispay => "satispay",
            ConfirmationTokensResourcePaymentMethodPreviewType::SepaDebit => "sepa_debit",
            ConfirmationTokensResourcePaymentMethodPreviewType::Sofort => "sofort",
            ConfirmationTokensResourcePaymentMethodPreviewType::Swish => "swish",
            ConfirmationTokensResourcePaymentMethodPreviewType::Twint => "twint",
            ConfirmationTokensResourcePaymentMethodPreviewType::UsBankAccount => "us_bank_account",
            ConfirmationTokensResourcePaymentMethodPreviewType::WechatPay => "wechat_pay",
            ConfirmationTokensResourcePaymentMethodPreviewType::Zip => "zip",
        }
    }
}

impl AsRef<str> for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn default() -> Self {
        Self::AcssDebit
    }
}
