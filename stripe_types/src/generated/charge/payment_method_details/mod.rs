#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<
        stripe_types::charge::payment_method_details::ach_credit_transfer::AchCreditTransfer,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<stripe_types::charge::payment_method_details::ach_debit::AchDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::charge::payment_method_details::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types::charge::payment_method_details::affirm::Affirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<stripe_types::charge::payment_method_details::afterpay_clearpay::AfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types::charge::payment_method_details::alipay::Alipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit:
        Option<stripe_types::charge::payment_method_details::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types::charge::payment_method_details::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types::charge::payment_method_details::bancontact::Bancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::charge::payment_method_details::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types::charge::payment_method_details::boleto::Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::charge::payment_method_details::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present:
        Option<stripe_types::charge::payment_method_details::card_present::CardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types::charge::payment_method_details::cashapp::Cashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<stripe_types::charge::payment_method_details::customer_balance::CustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types::charge::payment_method_details::eps::Eps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types::charge::payment_method_details::fpx::Fpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types::charge::payment_method_details::giropay::Giropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types::charge::payment_method_details::grabpay::Grabpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::charge::payment_method_details::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present:
        Option<stripe_types::charge::payment_method_details::interac_present::InteracPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::charge::payment_method_details::klarna::Klarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types::charge::payment_method_details::konbini::Konbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::charge::payment_method_details::link::Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types::charge::payment_method_details::multibanco::Multibanco>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types::charge::payment_method_details::oxxo::Oxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types::charge::payment_method_details::p24::P24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types::charge::payment_method_details::paynow::Paynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::charge::payment_method_details::paypal::Paypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types::charge::payment_method_details::pix::Pix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types::charge::payment_method_details::promptpay::Promptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<
        stripe_types::charge::payment_method_details::sepa_credit_transfer::SepaCreditTransfer,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::charge::payment_method_details::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::charge::payment_method_details::sofort::Sofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_account:
        Option<stripe_types::charge::payment_method_details::stripe_account::StripeAccount>,
    /// The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<stripe_types::charge::payment_method_details::us_bank_account::UsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<stripe_types::charge::payment_method_details::wechat::Wechat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types::charge::payment_method_details::wechat_pay::WechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types::charge::payment_method_details::zip::Zip>,
}
pub mod alipay;
pub use alipay::Alipay;
pub mod ach_credit_transfer;
pub use ach_credit_transfer::AchCreditTransfer;
pub mod ach_debit;
pub use ach_debit::AchDebit;
pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod affirm;
pub use affirm::Affirm;
pub mod afterpay_clearpay;
pub use afterpay_clearpay::AfterpayClearpay;
pub mod au_becs_debit;
pub use au_becs_debit::AuBecsDebit;
pub mod bacs_debit;
pub use bacs_debit::BacsDebit;
pub mod bancontact;
pub use bancontact::Bancontact;
pub mod blik;
pub use blik::Blik;
pub mod boleto;
pub use boleto::Boleto;
pub mod card;
pub use card::Card;
pub mod card_present;
pub use card_present::CardPresent;
pub mod cashapp;
pub use cashapp::Cashapp;
pub mod customer_balance;
pub use customer_balance::CustomerBalance;
pub mod eps;
pub use eps::Eps;
pub mod fpx;
pub use fpx::Fpx;
pub mod giropay;
pub use giropay::Giropay;
pub mod grabpay;
pub use grabpay::Grabpay;
pub mod ideal;
pub use ideal::Ideal;
pub mod interac_present;
pub use interac_present::InteracPresent;
pub mod klarna;
pub use klarna::Klarna;
pub mod konbini;
pub use konbini::Konbini;
pub mod store;
pub use store::Store;
pub mod link;
pub use link::Link;
pub mod multibanco;
pub use multibanco::Multibanco;
pub mod oxxo;
pub use oxxo::Oxxo;
pub mod p24;
pub use p24::P24;
pub mod paynow;
pub use paynow::Paynow;
pub mod paypal;
pub use paypal::Paypal;
pub mod pix;
pub use pix::Pix;
pub mod promptpay;
pub use promptpay::Promptpay;
pub mod sepa_credit_transfer;
pub use sepa_credit_transfer::SepaCreditTransfer;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod sofort;
pub use sofort::Sofort;
pub mod stripe_account;
pub use stripe_account::StripeAccount;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
pub mod wechat;
pub use wechat::Wechat;
pub mod wechat_pay;
pub use wechat_pay::WechatPay;
pub mod zip;
pub use zip::Zip;
