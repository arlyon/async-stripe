#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    pub acss_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    pub affirm_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    pub afterpay_clearpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Alma capability of the account, or whether the account can directly process Alma payments.
    pub alma_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the AmazonPay capability of the account, or whether the account can directly process AmazonPay payments.
    pub amazon_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the `app_distribution` capability of the account, or whether the platform can distribute apps to other accounts.
    pub app_distribution: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    pub au_becs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    pub bacs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    pub bancontact_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    pub bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Billie capability of the account, or whether the account can directly process Billie payments.
    pub billie_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Bizum capability of the account, or whether the account can directly process Bizum payments.
    pub bizum_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    pub blik_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    pub boleto_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    pub card_issuing: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    pub card_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    pub cartes_bancaires_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
    pub cashapp_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Crypto capability of the account, or whether the account can directly process Crypto payments.
    pub crypto_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    pub eps_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    pub fpx_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the GB customer_balance payments (GBP currency) capability of the account, or whether the account can directly process GB customer_balance charges.
    pub gb_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    pub giropay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    pub grabpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    pub ideal_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
    pub india_international_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    pub jcb_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Japanese customer_balance payments (JPY currency) capability of the account, or whether the account can directly process Japanese customer_balance charges.
    pub jp_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the KakaoPay capability of the account, or whether the account can directly process KakaoPay payments.
    pub kakao_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    pub klarna_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    pub konbini_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the KrCard capability of the account, or whether the account can directly process KrCard payments.
    pub kr_card_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the legacy payments capability of the account.
    pub legacy_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    pub link_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the MB WAY payments capability of the account, or whether the account can directly process MB WAY charges.
    pub mb_way_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the MobilePay capability of the account, or whether the account can directly process MobilePay charges.
    pub mobilepay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Multibanco payments capability of the account, or whether the account can directly process Multibanco charges.
    pub multibanco_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Mexican customer_balance payments (MXN currency) capability of the account, or whether the account can directly process Mexican customer_balance charges.
    pub mx_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the NaverPay capability of the account, or whether the account can directly process NaverPay payments.
    pub naver_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the New Zealand BECS Direct Debit payments capability of the account, or whether the account can directly process New Zealand BECS Direct Debit charges.
    pub nz_bank_account_becs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    pub oxxo_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    pub p24_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the pay_by_bank payments capability of the account, or whether the account can directly process pay_by_bank charges.
    pub pay_by_bank_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Payco capability of the account, or whether the account can directly process Payco payments.
    pub payco_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    pub paynow_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the PayTo capability of the account, or whether the account can directly process PayTo charges.
    pub payto_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the pix payments capability of the account, or whether the account can directly process pix charges.
    pub pix_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    pub promptpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
    pub revolut_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the SamsungPay capability of the account, or whether the account can directly process SamsungPay payments.
    pub samsung_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Satispay capability of the account, or whether the account can directly process Satispay payments.
    pub satispay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Scalapay capability of the account, or whether the account can directly process Scalapay payments.
    pub scalapay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the SEPA customer_balance payments (EUR currency) capability of the account, or whether the account can directly process SEPA customer_balance charges.
    pub sepa_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    pub sepa_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    pub sofort_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Sunbit capability of the account, or whether the account can directly process Sunbit payments.
    pub sunbit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Swish capability of the account, or whether the account can directly process Swish payments.
    pub swish_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the tax reporting 1099-K (US) capability of the account.
    pub tax_reporting_us_1099_k: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    pub tax_reporting_us_1099_misc: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    pub transfers: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the banking capability, or whether the account can have bank accounts.
    pub treasury: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the TWINT capability of the account, or whether the account can directly process TWINT charges.
    pub twint_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the upi payments capability of the account, or whether the account can directly process upi charges.
    pub upi_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    pub us_bank_account_ach_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the US customer_balance payments (USD currency) capability of the account, or whether the account can directly process US customer_balance charges.
    pub us_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    pub zip_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountCapabilities").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountCapabilitiesBuilder {
    acss_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    affirm_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    afterpay_clearpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    alma_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    amazon_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    app_distribution: Option<stripe_shared::AccountCapabilitiesStatus>,
    au_becs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    bacs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    bancontact_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    billie_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    bizum_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    blik_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    boleto_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    card_issuing: Option<stripe_shared::AccountCapabilitiesStatus>,
    card_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    cartes_bancaires_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    cashapp_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    crypto_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    eps_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    fpx_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    gb_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    giropay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    grabpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    ideal_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    india_international_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    jcb_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    jp_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    kakao_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    klarna_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    konbini_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    kr_card_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    legacy_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    link_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    mb_way_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    mobilepay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    multibanco_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    mx_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    naver_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    nz_bank_account_becs_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    oxxo_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    p24_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    pay_by_bank_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    payco_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    paynow_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    payto_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    pix_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    promptpay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    revolut_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    samsung_pay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    satispay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    scalapay_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    sepa_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    sepa_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    sofort_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    sunbit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    swish_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    tax_reporting_us_1099_k: Option<stripe_shared::AccountCapabilitiesStatus>,
    tax_reporting_us_1099_misc: Option<stripe_shared::AccountCapabilitiesStatus>,
    transfers: Option<stripe_shared::AccountCapabilitiesStatus>,
    treasury: Option<stripe_shared::AccountCapabilitiesStatus>,
    twint_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    upi_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    us_bank_account_ach_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    us_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    zip_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountCapabilities {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCapabilities>,
        builder: AccountCapabilitiesBuilder,
    }

    impl Visitor for Place<AccountCapabilities> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountCapabilitiesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountCapabilitiesBuilder {
        type Out = AccountCapabilities;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit_payments)
                }
                "affirm_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.affirm_payments)
                }
                "afterpay_clearpay_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.afterpay_clearpay_payments,
                ),
                "alma_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.alma_payments)
                }
                "amazon_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amazon_pay_payments)
                }
                "app_distribution" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.app_distribution)
                }
                "au_becs_debit_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.au_becs_debit_payments,
                ),
                "bacs_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bacs_debit_payments)
                }
                "bancontact_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bancontact_payments)
                }
                "bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.bank_transfer_payments,
                ),
                "billie_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.billie_payments)
                }
                "bizum_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bizum_payments)
                }
                "blik_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.blik_payments)
                }
                "boleto_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto_payments)
                }
                "card_issuing" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_issuing)
                }
                "card_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_payments)
                }
                "cartes_bancaires_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.cartes_bancaires_payments,
                ),
                "cashapp_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp_payments)
                }
                "crypto_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.crypto_payments)
                }
                "eps_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.eps_payments)
                }
                "fpx_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.fpx_payments)
                }
                "gb_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.gb_bank_transfer_payments,
                ),
                "giropay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.giropay_payments)
                }
                "grabpay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.grabpay_payments)
                }
                "ideal_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.ideal_payments)
                }
                "india_international_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.india_international_payments,
                ),
                "jcb_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.jcb_payments)
                }
                "jp_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.jp_bank_transfer_payments,
                ),
                "kakao_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.kakao_pay_payments)
                }
                "klarna_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna_payments)
                }
                "konbini_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.konbini_payments)
                }
                "kr_card_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.kr_card_payments)
                }
                "legacy_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.legacy_payments)
                }
                "link_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.link_payments)
                }
                "mb_way_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.mb_way_payments)
                }
                "mobilepay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.mobilepay_payments)
                }
                "multibanco_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.multibanco_payments)
                }
                "mx_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.mx_bank_transfer_payments,
                ),
                "naver_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.naver_pay_payments)
                }
                "nz_bank_account_becs_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.nz_bank_account_becs_debit_payments,
                    )
                }
                "oxxo_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.oxxo_payments)
                }
                "p24_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.p24_payments)
                }
                "pay_by_bank_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.pay_by_bank_payments)
                }
                "payco_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payco_payments)
                }
                "paynow_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.paynow_payments)
                }
                "payto_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payto_payments)
                }
                "pix_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.pix_payments)
                }
                "promptpay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.promptpay_payments)
                }
                "revolut_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.revolut_pay_payments)
                }
                "samsung_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.samsung_pay_payments)
                }
                "satispay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.satispay_payments)
                }
                "scalapay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.scalapay_payments)
                }
                "sepa_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.sepa_bank_transfer_payments,
                ),
                "sepa_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit_payments)
                }
                "sofort_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sofort_payments)
                }
                "sunbit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sunbit_payments)
                }
                "swish_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.swish_payments)
                }
                "tax_reporting_us_1099_k" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.tax_reporting_us_1099_k,
                ),
                "tax_reporting_us_1099_misc" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.tax_reporting_us_1099_misc,
                ),
                "transfers" => stripe_types::miniserde_helpers::begin_nullable(&mut self.transfers),
                "treasury" => stripe_types::miniserde_helpers::begin_nullable(&mut self.treasury),
                "twint_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.twint_payments)
                }
                "upi_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.upi_payments)
                }
                "us_bank_account_ach_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.us_bank_account_ach_payments,
                ),
                "us_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.us_bank_transfer_payments,
                ),
                "zip_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.zip_payments)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit_payments: None,
                affirm_payments: None,
                afterpay_clearpay_payments: None,
                alma_payments: None,
                amazon_pay_payments: None,
                app_distribution: None,
                au_becs_debit_payments: None,
                bacs_debit_payments: None,
                bancontact_payments: None,
                bank_transfer_payments: None,
                billie_payments: None,
                bizum_payments: None,
                blik_payments: None,
                boleto_payments: None,
                card_issuing: None,
                card_payments: None,
                cartes_bancaires_payments: None,
                cashapp_payments: None,
                crypto_payments: None,
                eps_payments: None,
                fpx_payments: None,
                gb_bank_transfer_payments: None,
                giropay_payments: None,
                grabpay_payments: None,
                ideal_payments: None,
                india_international_payments: None,
                jcb_payments: None,
                jp_bank_transfer_payments: None,
                kakao_pay_payments: None,
                klarna_payments: None,
                konbini_payments: None,
                kr_card_payments: None,
                legacy_payments: None,
                link_payments: None,
                mb_way_payments: None,
                mobilepay_payments: None,
                multibanco_payments: None,
                mx_bank_transfer_payments: None,
                naver_pay_payments: None,
                nz_bank_account_becs_debit_payments: None,
                oxxo_payments: None,
                p24_payments: None,
                pay_by_bank_payments: None,
                payco_payments: None,
                paynow_payments: None,
                payto_payments: None,
                pix_payments: None,
                promptpay_payments: None,
                revolut_pay_payments: None,
                samsung_pay_payments: None,
                satispay_payments: None,
                scalapay_payments: None,
                sepa_bank_transfer_payments: None,
                sepa_debit_payments: None,
                sofort_payments: None,
                sunbit_payments: None,
                swish_payments: None,
                tax_reporting_us_1099_k: None,
                tax_reporting_us_1099_misc: None,
                transfers: None,
                treasury: None,
                twint_payments: None,
                upi_payments: None,
                us_bank_account_ach_payments: None,
                us_bank_transfer_payments: None,
                zip_payments: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                acss_debit_payments,
                affirm_payments,
                afterpay_clearpay_payments,
                alma_payments,
                amazon_pay_payments,
                app_distribution,
                au_becs_debit_payments,
                bacs_debit_payments,
                bancontact_payments,
                bank_transfer_payments,
                billie_payments,
                bizum_payments,
                blik_payments,
                boleto_payments,
                card_issuing,
                card_payments,
                cartes_bancaires_payments,
                cashapp_payments,
                crypto_payments,
                eps_payments,
                fpx_payments,
                gb_bank_transfer_payments,
                giropay_payments,
                grabpay_payments,
                ideal_payments,
                india_international_payments,
                jcb_payments,
                jp_bank_transfer_payments,
                kakao_pay_payments,
                klarna_payments,
                konbini_payments,
                kr_card_payments,
                legacy_payments,
                link_payments,
                mb_way_payments,
                mobilepay_payments,
                multibanco_payments,
                mx_bank_transfer_payments,
                naver_pay_payments,
                nz_bank_account_becs_debit_payments,
                oxxo_payments,
                p24_payments,
                pay_by_bank_payments,
                payco_payments,
                paynow_payments,
                payto_payments,
                pix_payments,
                promptpay_payments,
                revolut_pay_payments,
                samsung_pay_payments,
                satispay_payments,
                scalapay_payments,
                sepa_bank_transfer_payments,
                sepa_debit_payments,
                sofort_payments,
                sunbit_payments,
                swish_payments,
                tax_reporting_us_1099_k,
                tax_reporting_us_1099_misc,
                transfers,
                treasury,
                twint_payments,
                upi_payments,
                us_bank_account_ach_payments,
                us_bank_transfer_payments,
                zip_payments,
            ) = (
                self.acss_debit_payments.take(),
                self.affirm_payments.take(),
                self.afterpay_clearpay_payments.take(),
                self.alma_payments.take(),
                self.amazon_pay_payments.take(),
                self.app_distribution.take(),
                self.au_becs_debit_payments.take(),
                self.bacs_debit_payments.take(),
                self.bancontact_payments.take(),
                self.bank_transfer_payments.take(),
                self.billie_payments.take(),
                self.bizum_payments.take(),
                self.blik_payments.take(),
                self.boleto_payments.take(),
                self.card_issuing.take(),
                self.card_payments.take(),
                self.cartes_bancaires_payments.take(),
                self.cashapp_payments.take(),
                self.crypto_payments.take(),
                self.eps_payments.take(),
                self.fpx_payments.take(),
                self.gb_bank_transfer_payments.take(),
                self.giropay_payments.take(),
                self.grabpay_payments.take(),
                self.ideal_payments.take(),
                self.india_international_payments.take(),
                self.jcb_payments.take(),
                self.jp_bank_transfer_payments.take(),
                self.kakao_pay_payments.take(),
                self.klarna_payments.take(),
                self.konbini_payments.take(),
                self.kr_card_payments.take(),
                self.legacy_payments.take(),
                self.link_payments.take(),
                self.mb_way_payments.take(),
                self.mobilepay_payments.take(),
                self.multibanco_payments.take(),
                self.mx_bank_transfer_payments.take(),
                self.naver_pay_payments.take(),
                self.nz_bank_account_becs_debit_payments.take(),
                self.oxxo_payments.take(),
                self.p24_payments.take(),
                self.pay_by_bank_payments.take(),
                self.payco_payments.take(),
                self.paynow_payments.take(),
                self.payto_payments.take(),
                self.pix_payments.take(),
                self.promptpay_payments.take(),
                self.revolut_pay_payments.take(),
                self.samsung_pay_payments.take(),
                self.satispay_payments.take(),
                self.scalapay_payments.take(),
                self.sepa_bank_transfer_payments.take(),
                self.sepa_debit_payments.take(),
                self.sofort_payments.take(),
                self.sunbit_payments.take(),
                self.swish_payments.take(),
                self.tax_reporting_us_1099_k.take(),
                self.tax_reporting_us_1099_misc.take(),
                self.transfers.take(),
                self.treasury.take(),
                self.twint_payments.take(),
                self.upi_payments.take(),
                self.us_bank_account_ach_payments.take(),
                self.us_bank_transfer_payments.take(),
                self.zip_payments.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                acss_debit_payments,
                affirm_payments,
                afterpay_clearpay_payments,
                alma_payments,
                amazon_pay_payments,
                app_distribution,
                au_becs_debit_payments,
                bacs_debit_payments,
                bancontact_payments,
                bank_transfer_payments,
                billie_payments,
                bizum_payments,
                blik_payments,
                boleto_payments,
                card_issuing,
                card_payments,
                cartes_bancaires_payments,
                cashapp_payments,
                crypto_payments,
                eps_payments,
                fpx_payments,
                gb_bank_transfer_payments,
                giropay_payments,
                grabpay_payments,
                ideal_payments,
                india_international_payments,
                jcb_payments,
                jp_bank_transfer_payments,
                kakao_pay_payments,
                klarna_payments,
                konbini_payments,
                kr_card_payments,
                legacy_payments,
                link_payments,
                mb_way_payments,
                mobilepay_payments,
                multibanco_payments,
                mx_bank_transfer_payments,
                naver_pay_payments,
                nz_bank_account_becs_debit_payments,
                oxxo_payments,
                p24_payments,
                pay_by_bank_payments,
                payco_payments,
                paynow_payments,
                payto_payments,
                pix_payments,
                promptpay_payments,
                revolut_pay_payments,
                samsung_pay_payments,
                satispay_payments,
                scalapay_payments,
                sepa_bank_transfer_payments,
                sepa_debit_payments,
                sofort_payments,
                sunbit_payments,
                swish_payments,
                tax_reporting_us_1099_k,
                tax_reporting_us_1099_misc,
                transfers,
                treasury,
                twint_payments,
                upi_payments,
                us_bank_account_ach_payments,
                us_bank_transfer_payments,
                zip_payments,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for AccountCapabilities {
        type Builder = AccountCapabilitiesBuilder;
    }

    impl FromValueOpt for AccountCapabilities {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountCapabilitiesStatus {
    Active,
    Inactive,
    Pending,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountCapabilitiesStatus {
    pub fn as_str(&self) -> &str {
        use AccountCapabilitiesStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AccountCapabilitiesStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountCapabilitiesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountCapabilitiesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCapabilitiesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountCapabilitiesStatus)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountCapabilitiesStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountCapabilitiesStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountCapabilitiesStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountCapabilitiesStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountCapabilitiesStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
