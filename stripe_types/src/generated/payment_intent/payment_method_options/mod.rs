#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit:
        Option<stripe_types::payment_intent::payment_method_options::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types::payment_intent::payment_method_options::affirm::Affirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<
        stripe_types::payment_intent::payment_method_options::afterpay_clearpay::AfterpayClearpay,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types::payment_intent::payment_method_options::alipay::Alipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit:
        Option<stripe_types::payment_intent::payment_method_options::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit:
        Option<stripe_types::payment_intent::payment_method_options::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact:
        Option<stripe_types::payment_intent::payment_method_options::bancontact::Bancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::payment_intent::payment_method_options::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types::payment_intent::payment_method_options::boleto::Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::payment_intent::payment_method_options::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present:
        Option<stripe_types::payment_intent::payment_method_options::card_present::CardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<
        stripe_types::payment_intent::payment_method_options::customer_balance::CustomerBalance,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types::payment_intent::payment_method_options::eps::Eps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types::payment_intent::payment_method_options::fpx::Fpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types::payment_intent::payment_method_options::giropay::Giropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types::payment_intent::payment_method_options::grabpay::Grabpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::payment_intent::payment_method_options::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<
        stripe_types::payment_intent::payment_method_options::interac_present::InteracPresent,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::payment_intent::payment_method_options::klarna::Klarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types::payment_intent::payment_method_options::konbini::Konbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::payment_intent::payment_method_options::link::Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types::payment_intent::payment_method_options::oxxo::Oxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types::payment_intent::payment_method_options::p24::P24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types::payment_intent::payment_method_options::paynow::Paynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types::payment_intent::payment_method_options::pix::Pix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay:
        Option<stripe_types::payment_intent::payment_method_options::promptpay::Promptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit:
        Option<stripe_types::payment_intent::payment_method_options::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::payment_intent::payment_method_options::sofort::Sofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<
        stripe_types::payment_intent::payment_method_options::us_bank_account::UsBankAccount,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay:
        Option<stripe_types::payment_intent::payment_method_options::wechat_pay::WechatPay>,
}
pub mod blik_mandate_options_off_session_details;
pub use blik_mandate_options_off_session_details::BlikMandateOptionsOffSessionDetails;
pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod au_becs_debit;
pub use au_becs_debit::AuBecsDebit;
pub mod blik;
pub use blik::Blik;
pub mod card;
pub use card::Card;
pub mod eps;
pub use eps::Eps;
pub mod link;
pub use link::Link;
pub mod sepa_debit_mandate_options;
pub use sepa_debit_mandate_options::SepaDebitMandateOptions;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
pub mod affirm;
pub use affirm::Affirm;
pub mod afterpay_clearpay;
pub use afterpay_clearpay::AfterpayClearpay;
pub mod alipay;
pub use alipay::Alipay;
pub mod bacs_debit;
pub use bacs_debit::BacsDebit;
pub mod bancontact;
pub use bancontact::Bancontact;
pub mod boleto;
pub use boleto::Boleto;
pub mod card_present;
pub use card_present::CardPresent;
pub mod customer_balance;
pub use customer_balance::CustomerBalance;
pub mod bank_transfer;
pub use bank_transfer::BankTransfer;
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
pub mod sofort;
pub use sofort::Sofort;
pub mod wechat_pay;
pub use wechat_pay::WechatPay;
