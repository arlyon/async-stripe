#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::checkout::session::payment_method_options::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<crate::checkout::session::payment_method_options::affirm::Affirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<
        crate::checkout::session::payment_method_options::afterpay_clearpay::AfterpayClearpay,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::checkout::session::payment_method_options::alipay::Alipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit:
        Option<crate::checkout::session::payment_method_options::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::checkout::session::payment_method_options::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact:
        Option<crate::checkout::session::payment_method_options::bancontact::Bancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<crate::checkout::session::payment_method_options::boleto::Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::checkout::session::payment_method_options::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<crate::checkout::session::payment_method_options::customer_balance::CustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<crate::checkout::session::payment_method_options::eps::Eps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<crate::checkout::session::payment_method_options::fpx::Fpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<crate::checkout::session::payment_method_options::giropay::Giropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<crate::checkout::session::payment_method_options::grab_pay::GrabPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::checkout::session::payment_method_options::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::checkout::session::payment_method_options::klarna::Klarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<crate::checkout::session::payment_method_options::konbini::Konbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<crate::checkout::session::payment_method_options::oxxo::Oxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::checkout::session::payment_method_options::p24::P24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<crate::checkout::session::payment_method_options::paynow::Paynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<crate::checkout::session::payment_method_options::pix::Pix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::checkout::session::payment_method_options::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::checkout::session::payment_method_options::sofort::Sofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<crate::checkout::session::payment_method_options::us_bank_account::UsBankAccount>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod affirm;
pub use affirm::Affirm;
pub mod afterpay_clearpay;
pub use afterpay_clearpay::AfterpayClearpay;
pub mod alipay;
pub use alipay::Alipay;
pub mod au_becs_debit;
pub use au_becs_debit::AuBecsDebit;
pub mod bacs_debit;
pub use bacs_debit::BacsDebit;
pub mod bancontact;
pub use bancontact::Bancontact;
pub mod boleto;
pub use boleto::Boleto;
pub mod card;
pub use card::Card;
pub mod bank_transfer;
pub use bank_transfer::BankTransfer;
pub mod customer_balance;
pub use customer_balance::CustomerBalance;
pub mod eps;
pub use eps::Eps;
pub mod fpx;
pub use fpx::Fpx;
pub mod giropay;
pub use giropay::Giropay;
pub mod grab_pay;
pub use grab_pay::GrabPay;
pub mod ideal;
pub use ideal::Ideal;
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
pub mod pix;
pub use pix::Pix;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod sofort;
pub use sofort::Sofort;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
