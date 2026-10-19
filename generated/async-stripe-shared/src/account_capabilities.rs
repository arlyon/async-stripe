#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    pub acss_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    pub affirm_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    pub afterpay_clearpay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Alma capability of the account, or whether the account can directly process Alma payments.
    pub alma_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the AmazonPay capability of the account, or whether the account can directly process AmazonPay payments.
    pub amazon_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the `app_distribution` capability of the account, or whether the platform can distribute apps to other accounts.
    pub app_distribution: Option<stripe_shared::MandateStatus>,
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    pub au_becs_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    pub bacs_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    pub bancontact_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    pub bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Billie capability of the account, or whether the account can directly process Billie payments.
    pub billie_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Bizum capability of the account, or whether the account can directly process Bizum payments.
    pub bizum_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    pub blik_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    pub boleto_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    pub card_issuing: Option<stripe_shared::MandateStatus>,
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    pub card_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    pub cartes_bancaires_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
    pub cashapp_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Crypto capability of the account, or whether the account can directly process Crypto payments.
    pub crypto_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    pub eps_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    pub fpx_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the GB customer_balance payments (GBP currency) capability of the account, or whether the account can directly process GB customer_balance charges.
    pub gb_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    pub giropay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    pub grabpay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    pub ideal_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
    pub india_international_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    pub jcb_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Japanese customer_balance payments (JPY currency) capability of the account, or whether the account can directly process Japanese customer_balance charges.
    pub jp_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the KakaoPay capability of the account, or whether the account can directly process KakaoPay payments.
    pub kakao_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    pub klarna_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    pub konbini_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the KrCard capability of the account, or whether the account can directly process KrCard payments.
    pub kr_card_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the legacy payments capability of the account.
    pub legacy_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    pub link_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the MB WAY payments capability of the account, or whether the account can directly process MB WAY charges.
    pub mb_way_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the MobilePay capability of the account, or whether the account can directly process MobilePay charges.
    pub mobilepay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Multibanco payments capability of the account, or whether the account can directly process Multibanco charges.
    pub multibanco_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Mexican customer_balance payments (MXN currency) capability of the account, or whether the account can directly process Mexican customer_balance charges.
    pub mx_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the NaverPay capability of the account, or whether the account can directly process NaverPay payments.
    pub naver_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the New Zealand BECS Direct Debit payments capability of the account, or whether the account can directly process New Zealand BECS Direct Debit charges.
    pub nz_bank_account_becs_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    pub oxxo_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    pub p24_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the pay_by_bank payments capability of the account, or whether the account can directly process pay_by_bank charges.
    pub pay_by_bank_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Payco capability of the account, or whether the account can directly process Payco payments.
    pub payco_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    pub paynow_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the PayTo capability of the account, or whether the account can directly process PayTo charges.
    pub payto_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the pix payments capability of the account, or whether the account can directly process pix charges.
    pub pix_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    pub promptpay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
    pub revolut_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the SamsungPay capability of the account, or whether the account can directly process SamsungPay payments.
    pub samsung_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Satispay capability of the account, or whether the account can directly process Satispay payments.
    pub satispay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Scalapay capability of the account, or whether the account can directly process Scalapay payments.
    pub scalapay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the SEPA customer_balance payments (EUR currency) capability of the account, or whether the account can directly process SEPA customer_balance charges.
    pub sepa_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    pub sepa_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    pub sofort_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Sunbit capability of the account, or whether the account can directly process Sunbit payments.
    pub sunbit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Swish capability of the account, or whether the account can directly process Swish payments.
    pub swish_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the tax reporting 1099-K (US) capability of the account.
    pub tax_reporting_us_1099_k: Option<stripe_shared::MandateStatus>,
    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    pub tax_reporting_us_1099_misc: Option<stripe_shared::MandateStatus>,
    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    pub transfers: Option<stripe_shared::MandateStatus>,
    /// The status of the banking capability, or whether the account can have bank accounts.
    pub treasury: Option<stripe_shared::MandateStatus>,
    /// The status of the TWINT capability of the account, or whether the account can directly process TWINT charges.
    pub twint_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the upi payments capability of the account, or whether the account can directly process upi charges.
    pub upi_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    pub us_bank_account_ach_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the US customer_balance payments (USD currency) capability of the account, or whether the account can directly process US customer_balance charges.
    pub us_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    pub zip_payments: Option<stripe_shared::MandateStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountCapabilities").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountCapabilitiesBuilder {
    acss_debit_payments: Option<Option<stripe_shared::MandateStatus>>,
    affirm_payments: Option<Option<stripe_shared::MandateStatus>>,
    afterpay_clearpay_payments: Option<Option<stripe_shared::MandateStatus>>,
    alma_payments: Option<Option<stripe_shared::MandateStatus>>,
    amazon_pay_payments: Option<Option<stripe_shared::MandateStatus>>,
    app_distribution: Option<Option<stripe_shared::MandateStatus>>,
    au_becs_debit_payments: Option<Option<stripe_shared::MandateStatus>>,
    bacs_debit_payments: Option<Option<stripe_shared::MandateStatus>>,
    bancontact_payments: Option<Option<stripe_shared::MandateStatus>>,
    bank_transfer_payments: Option<Option<stripe_shared::MandateStatus>>,
    billie_payments: Option<Option<stripe_shared::MandateStatus>>,
    bizum_payments: Option<Option<stripe_shared::MandateStatus>>,
    blik_payments: Option<Option<stripe_shared::MandateStatus>>,
    boleto_payments: Option<Option<stripe_shared::MandateStatus>>,
    card_issuing: Option<Option<stripe_shared::MandateStatus>>,
    card_payments: Option<Option<stripe_shared::MandateStatus>>,
    cartes_bancaires_payments: Option<Option<stripe_shared::MandateStatus>>,
    cashapp_payments: Option<Option<stripe_shared::MandateStatus>>,
    crypto_payments: Option<Option<stripe_shared::MandateStatus>>,
    eps_payments: Option<Option<stripe_shared::MandateStatus>>,
    fpx_payments: Option<Option<stripe_shared::MandateStatus>>,
    gb_bank_transfer_payments: Option<Option<stripe_shared::MandateStatus>>,
    giropay_payments: Option<Option<stripe_shared::MandateStatus>>,
    grabpay_payments: Option<Option<stripe_shared::MandateStatus>>,
    ideal_payments: Option<Option<stripe_shared::MandateStatus>>,
    india_international_payments: Option<Option<stripe_shared::MandateStatus>>,
    jcb_payments: Option<Option<stripe_shared::MandateStatus>>,
    jp_bank_transfer_payments: Option<Option<stripe_shared::MandateStatus>>,
    kakao_pay_payments: Option<Option<stripe_shared::MandateStatus>>,
    klarna_payments: Option<Option<stripe_shared::MandateStatus>>,
    konbini_payments: Option<Option<stripe_shared::MandateStatus>>,
    kr_card_payments: Option<Option<stripe_shared::MandateStatus>>,
    legacy_payments: Option<Option<stripe_shared::MandateStatus>>,
    link_payments: Option<Option<stripe_shared::MandateStatus>>,
    mb_way_payments: Option<Option<stripe_shared::MandateStatus>>,
    mobilepay_payments: Option<Option<stripe_shared::MandateStatus>>,
    multibanco_payments: Option<Option<stripe_shared::MandateStatus>>,
    mx_bank_transfer_payments: Option<Option<stripe_shared::MandateStatus>>,
    naver_pay_payments: Option<Option<stripe_shared::MandateStatus>>,
    nz_bank_account_becs_debit_payments: Option<Option<stripe_shared::MandateStatus>>,
    oxxo_payments: Option<Option<stripe_shared::MandateStatus>>,
    p24_payments: Option<Option<stripe_shared::MandateStatus>>,
    pay_by_bank_payments: Option<Option<stripe_shared::MandateStatus>>,
    payco_payments: Option<Option<stripe_shared::MandateStatus>>,
    paynow_payments: Option<Option<stripe_shared::MandateStatus>>,
    payto_payments: Option<Option<stripe_shared::MandateStatus>>,
    pix_payments: Option<Option<stripe_shared::MandateStatus>>,
    promptpay_payments: Option<Option<stripe_shared::MandateStatus>>,
    revolut_pay_payments: Option<Option<stripe_shared::MandateStatus>>,
    samsung_pay_payments: Option<Option<stripe_shared::MandateStatus>>,
    satispay_payments: Option<Option<stripe_shared::MandateStatus>>,
    scalapay_payments: Option<Option<stripe_shared::MandateStatus>>,
    sepa_bank_transfer_payments: Option<Option<stripe_shared::MandateStatus>>,
    sepa_debit_payments: Option<Option<stripe_shared::MandateStatus>>,
    sofort_payments: Option<Option<stripe_shared::MandateStatus>>,
    sunbit_payments: Option<Option<stripe_shared::MandateStatus>>,
    swish_payments: Option<Option<stripe_shared::MandateStatus>>,
    tax_reporting_us_1099_k: Option<Option<stripe_shared::MandateStatus>>,
    tax_reporting_us_1099_misc: Option<Option<stripe_shared::MandateStatus>>,
    transfers: Option<Option<stripe_shared::MandateStatus>>,
    treasury: Option<Option<stripe_shared::MandateStatus>>,
    twint_payments: Option<Option<stripe_shared::MandateStatus>>,
    upi_payments: Option<Option<stripe_shared::MandateStatus>>,
    us_bank_account_ach_payments: Option<Option<stripe_shared::MandateStatus>>,
    us_bank_transfer_payments: Option<Option<stripe_shared::MandateStatus>>,
    zip_payments: Option<Option<stripe_shared::MandateStatus>>,
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
                "acss_debit_payments" => Deserialize::begin(&mut self.acss_debit_payments),
                "affirm_payments" => Deserialize::begin(&mut self.affirm_payments),
                "afterpay_clearpay_payments" => {
                    Deserialize::begin(&mut self.afterpay_clearpay_payments)
                }
                "alma_payments" => Deserialize::begin(&mut self.alma_payments),
                "amazon_pay_payments" => Deserialize::begin(&mut self.amazon_pay_payments),
                "app_distribution" => Deserialize::begin(&mut self.app_distribution),
                "au_becs_debit_payments" => Deserialize::begin(&mut self.au_becs_debit_payments),
                "bacs_debit_payments" => Deserialize::begin(&mut self.bacs_debit_payments),
                "bancontact_payments" => Deserialize::begin(&mut self.bancontact_payments),
                "bank_transfer_payments" => Deserialize::begin(&mut self.bank_transfer_payments),
                "billie_payments" => Deserialize::begin(&mut self.billie_payments),
                "bizum_payments" => Deserialize::begin(&mut self.bizum_payments),
                "blik_payments" => Deserialize::begin(&mut self.blik_payments),
                "boleto_payments" => Deserialize::begin(&mut self.boleto_payments),
                "card_issuing" => Deserialize::begin(&mut self.card_issuing),
                "card_payments" => Deserialize::begin(&mut self.card_payments),
                "cartes_bancaires_payments" => {
                    Deserialize::begin(&mut self.cartes_bancaires_payments)
                }
                "cashapp_payments" => Deserialize::begin(&mut self.cashapp_payments),
                "crypto_payments" => Deserialize::begin(&mut self.crypto_payments),
                "eps_payments" => Deserialize::begin(&mut self.eps_payments),
                "fpx_payments" => Deserialize::begin(&mut self.fpx_payments),
                "gb_bank_transfer_payments" => {
                    Deserialize::begin(&mut self.gb_bank_transfer_payments)
                }
                "giropay_payments" => Deserialize::begin(&mut self.giropay_payments),
                "grabpay_payments" => Deserialize::begin(&mut self.grabpay_payments),
                "ideal_payments" => Deserialize::begin(&mut self.ideal_payments),
                "india_international_payments" => {
                    Deserialize::begin(&mut self.india_international_payments)
                }
                "jcb_payments" => Deserialize::begin(&mut self.jcb_payments),
                "jp_bank_transfer_payments" => {
                    Deserialize::begin(&mut self.jp_bank_transfer_payments)
                }
                "kakao_pay_payments" => Deserialize::begin(&mut self.kakao_pay_payments),
                "klarna_payments" => Deserialize::begin(&mut self.klarna_payments),
                "konbini_payments" => Deserialize::begin(&mut self.konbini_payments),
                "kr_card_payments" => Deserialize::begin(&mut self.kr_card_payments),
                "legacy_payments" => Deserialize::begin(&mut self.legacy_payments),
                "link_payments" => Deserialize::begin(&mut self.link_payments),
                "mb_way_payments" => Deserialize::begin(&mut self.mb_way_payments),
                "mobilepay_payments" => Deserialize::begin(&mut self.mobilepay_payments),
                "multibanco_payments" => Deserialize::begin(&mut self.multibanco_payments),
                "mx_bank_transfer_payments" => {
                    Deserialize::begin(&mut self.mx_bank_transfer_payments)
                }
                "naver_pay_payments" => Deserialize::begin(&mut self.naver_pay_payments),
                "nz_bank_account_becs_debit_payments" => {
                    Deserialize::begin(&mut self.nz_bank_account_becs_debit_payments)
                }
                "oxxo_payments" => Deserialize::begin(&mut self.oxxo_payments),
                "p24_payments" => Deserialize::begin(&mut self.p24_payments),
                "pay_by_bank_payments" => Deserialize::begin(&mut self.pay_by_bank_payments),
                "payco_payments" => Deserialize::begin(&mut self.payco_payments),
                "paynow_payments" => Deserialize::begin(&mut self.paynow_payments),
                "payto_payments" => Deserialize::begin(&mut self.payto_payments),
                "pix_payments" => Deserialize::begin(&mut self.pix_payments),
                "promptpay_payments" => Deserialize::begin(&mut self.promptpay_payments),
                "revolut_pay_payments" => Deserialize::begin(&mut self.revolut_pay_payments),
                "samsung_pay_payments" => Deserialize::begin(&mut self.samsung_pay_payments),
                "satispay_payments" => Deserialize::begin(&mut self.satispay_payments),
                "scalapay_payments" => Deserialize::begin(&mut self.scalapay_payments),
                "sepa_bank_transfer_payments" => {
                    Deserialize::begin(&mut self.sepa_bank_transfer_payments)
                }
                "sepa_debit_payments" => Deserialize::begin(&mut self.sepa_debit_payments),
                "sofort_payments" => Deserialize::begin(&mut self.sofort_payments),
                "sunbit_payments" => Deserialize::begin(&mut self.sunbit_payments),
                "swish_payments" => Deserialize::begin(&mut self.swish_payments),
                "tax_reporting_us_1099_k" => Deserialize::begin(&mut self.tax_reporting_us_1099_k),
                "tax_reporting_us_1099_misc" => {
                    Deserialize::begin(&mut self.tax_reporting_us_1099_misc)
                }
                "transfers" => Deserialize::begin(&mut self.transfers),
                "treasury" => Deserialize::begin(&mut self.treasury),
                "twint_payments" => Deserialize::begin(&mut self.twint_payments),
                "upi_payments" => Deserialize::begin(&mut self.upi_payments),
                "us_bank_account_ach_payments" => {
                    Deserialize::begin(&mut self.us_bank_account_ach_payments)
                }
                "us_bank_transfer_payments" => {
                    Deserialize::begin(&mut self.us_bank_transfer_payments)
                }
                "zip_payments" => Deserialize::begin(&mut self.zip_payments),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit_payments: Some(None),
                affirm_payments: Some(None),
                afterpay_clearpay_payments: Some(None),
                alma_payments: Some(None),
                amazon_pay_payments: Some(None),
                app_distribution: Some(None),
                au_becs_debit_payments: Some(None),
                bacs_debit_payments: Some(None),
                bancontact_payments: Some(None),
                bank_transfer_payments: Some(None),
                billie_payments: Some(None),
                bizum_payments: Some(None),
                blik_payments: Some(None),
                boleto_payments: Some(None),
                card_issuing: Some(None),
                card_payments: Some(None),
                cartes_bancaires_payments: Some(None),
                cashapp_payments: Some(None),
                crypto_payments: Some(None),
                eps_payments: Some(None),
                fpx_payments: Some(None),
                gb_bank_transfer_payments: Some(None),
                giropay_payments: Some(None),
                grabpay_payments: Some(None),
                ideal_payments: Some(None),
                india_international_payments: Some(None),
                jcb_payments: Some(None),
                jp_bank_transfer_payments: Some(None),
                kakao_pay_payments: Some(None),
                klarna_payments: Some(None),
                konbini_payments: Some(None),
                kr_card_payments: Some(None),
                legacy_payments: Some(None),
                link_payments: Some(None),
                mb_way_payments: Some(None),
                mobilepay_payments: Some(None),
                multibanco_payments: Some(None),
                mx_bank_transfer_payments: Some(None),
                naver_pay_payments: Some(None),
                nz_bank_account_becs_debit_payments: Some(None),
                oxxo_payments: Some(None),
                p24_payments: Some(None),
                pay_by_bank_payments: Some(None),
                payco_payments: Some(None),
                paynow_payments: Some(None),
                payto_payments: Some(None),
                pix_payments: Some(None),
                promptpay_payments: Some(None),
                revolut_pay_payments: Some(None),
                samsung_pay_payments: Some(None),
                satispay_payments: Some(None),
                scalapay_payments: Some(None),
                sepa_bank_transfer_payments: Some(None),
                sepa_debit_payments: Some(None),
                sofort_payments: Some(None),
                sunbit_payments: Some(None),
                swish_payments: Some(None),
                tax_reporting_us_1099_k: Some(None),
                tax_reporting_us_1099_misc: Some(None),
                transfers: Some(None),
                treasury: Some(None),
                twint_payments: Some(None),
                upi_payments: Some(None),
                us_bank_account_ach_payments: Some(None),
                us_bank_transfer_payments: Some(None),
                zip_payments: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acss_debit_payments),
                Some(affirm_payments),
                Some(afterpay_clearpay_payments),
                Some(alma_payments),
                Some(amazon_pay_payments),
                Some(app_distribution),
                Some(au_becs_debit_payments),
                Some(bacs_debit_payments),
                Some(bancontact_payments),
                Some(bank_transfer_payments),
                Some(billie_payments),
                Some(bizum_payments),
                Some(blik_payments),
                Some(boleto_payments),
                Some(card_issuing),
                Some(card_payments),
                Some(cartes_bancaires_payments),
                Some(cashapp_payments),
                Some(crypto_payments),
                Some(eps_payments),
                Some(fpx_payments),
                Some(gb_bank_transfer_payments),
                Some(giropay_payments),
                Some(grabpay_payments),
                Some(ideal_payments),
                Some(india_international_payments),
                Some(jcb_payments),
                Some(jp_bank_transfer_payments),
                Some(kakao_pay_payments),
                Some(klarna_payments),
                Some(konbini_payments),
                Some(kr_card_payments),
                Some(legacy_payments),
                Some(link_payments),
                Some(mb_way_payments),
                Some(mobilepay_payments),
                Some(multibanco_payments),
                Some(mx_bank_transfer_payments),
                Some(naver_pay_payments),
                Some(nz_bank_account_becs_debit_payments),
                Some(oxxo_payments),
                Some(p24_payments),
                Some(pay_by_bank_payments),
                Some(payco_payments),
                Some(paynow_payments),
                Some(payto_payments),
                Some(pix_payments),
                Some(promptpay_payments),
                Some(revolut_pay_payments),
                Some(samsung_pay_payments),
                Some(satispay_payments),
                Some(scalapay_payments),
                Some(sepa_bank_transfer_payments),
                Some(sepa_debit_payments),
                Some(sofort_payments),
                Some(sunbit_payments),
                Some(swish_payments),
                Some(tax_reporting_us_1099_k),
                Some(tax_reporting_us_1099_misc),
                Some(transfers),
                Some(treasury),
                Some(twint_payments),
                Some(upi_payments),
                Some(us_bank_account_ach_payments),
                Some(us_bank_transfer_payments),
                Some(zip_payments),
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
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountCapabilitiesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit_payments" => b.acss_debit_payments = FromValueOpt::from_value(v),
                    "affirm_payments" => b.affirm_payments = FromValueOpt::from_value(v),
                    "afterpay_clearpay_payments" => {
                        b.afterpay_clearpay_payments = FromValueOpt::from_value(v)
                    }
                    "alma_payments" => b.alma_payments = FromValueOpt::from_value(v),
                    "amazon_pay_payments" => b.amazon_pay_payments = FromValueOpt::from_value(v),
                    "app_distribution" => b.app_distribution = FromValueOpt::from_value(v),
                    "au_becs_debit_payments" => {
                        b.au_becs_debit_payments = FromValueOpt::from_value(v)
                    }
                    "bacs_debit_payments" => b.bacs_debit_payments = FromValueOpt::from_value(v),
                    "bancontact_payments" => b.bancontact_payments = FromValueOpt::from_value(v),
                    "bank_transfer_payments" => {
                        b.bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "billie_payments" => b.billie_payments = FromValueOpt::from_value(v),
                    "bizum_payments" => b.bizum_payments = FromValueOpt::from_value(v),
                    "blik_payments" => b.blik_payments = FromValueOpt::from_value(v),
                    "boleto_payments" => b.boleto_payments = FromValueOpt::from_value(v),
                    "card_issuing" => b.card_issuing = FromValueOpt::from_value(v),
                    "card_payments" => b.card_payments = FromValueOpt::from_value(v),
                    "cartes_bancaires_payments" => {
                        b.cartes_bancaires_payments = FromValueOpt::from_value(v)
                    }
                    "cashapp_payments" => b.cashapp_payments = FromValueOpt::from_value(v),
                    "crypto_payments" => b.crypto_payments = FromValueOpt::from_value(v),
                    "eps_payments" => b.eps_payments = FromValueOpt::from_value(v),
                    "fpx_payments" => b.fpx_payments = FromValueOpt::from_value(v),
                    "gb_bank_transfer_payments" => {
                        b.gb_bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "giropay_payments" => b.giropay_payments = FromValueOpt::from_value(v),
                    "grabpay_payments" => b.grabpay_payments = FromValueOpt::from_value(v),
                    "ideal_payments" => b.ideal_payments = FromValueOpt::from_value(v),
                    "india_international_payments" => {
                        b.india_international_payments = FromValueOpt::from_value(v)
                    }
                    "jcb_payments" => b.jcb_payments = FromValueOpt::from_value(v),
                    "jp_bank_transfer_payments" => {
                        b.jp_bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "kakao_pay_payments" => b.kakao_pay_payments = FromValueOpt::from_value(v),
                    "klarna_payments" => b.klarna_payments = FromValueOpt::from_value(v),
                    "konbini_payments" => b.konbini_payments = FromValueOpt::from_value(v),
                    "kr_card_payments" => b.kr_card_payments = FromValueOpt::from_value(v),
                    "legacy_payments" => b.legacy_payments = FromValueOpt::from_value(v),
                    "link_payments" => b.link_payments = FromValueOpt::from_value(v),
                    "mb_way_payments" => b.mb_way_payments = FromValueOpt::from_value(v),
                    "mobilepay_payments" => b.mobilepay_payments = FromValueOpt::from_value(v),
                    "multibanco_payments" => b.multibanco_payments = FromValueOpt::from_value(v),
                    "mx_bank_transfer_payments" => {
                        b.mx_bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "naver_pay_payments" => b.naver_pay_payments = FromValueOpt::from_value(v),
                    "nz_bank_account_becs_debit_payments" => {
                        b.nz_bank_account_becs_debit_payments = FromValueOpt::from_value(v)
                    }
                    "oxxo_payments" => b.oxxo_payments = FromValueOpt::from_value(v),
                    "p24_payments" => b.p24_payments = FromValueOpt::from_value(v),
                    "pay_by_bank_payments" => b.pay_by_bank_payments = FromValueOpt::from_value(v),
                    "payco_payments" => b.payco_payments = FromValueOpt::from_value(v),
                    "paynow_payments" => b.paynow_payments = FromValueOpt::from_value(v),
                    "payto_payments" => b.payto_payments = FromValueOpt::from_value(v),
                    "pix_payments" => b.pix_payments = FromValueOpt::from_value(v),
                    "promptpay_payments" => b.promptpay_payments = FromValueOpt::from_value(v),
                    "revolut_pay_payments" => b.revolut_pay_payments = FromValueOpt::from_value(v),
                    "samsung_pay_payments" => b.samsung_pay_payments = FromValueOpt::from_value(v),
                    "satispay_payments" => b.satispay_payments = FromValueOpt::from_value(v),
                    "scalapay_payments" => b.scalapay_payments = FromValueOpt::from_value(v),
                    "sepa_bank_transfer_payments" => {
                        b.sepa_bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "sepa_debit_payments" => b.sepa_debit_payments = FromValueOpt::from_value(v),
                    "sofort_payments" => b.sofort_payments = FromValueOpt::from_value(v),
                    "sunbit_payments" => b.sunbit_payments = FromValueOpt::from_value(v),
                    "swish_payments" => b.swish_payments = FromValueOpt::from_value(v),
                    "tax_reporting_us_1099_k" => {
                        b.tax_reporting_us_1099_k = FromValueOpt::from_value(v)
                    }
                    "tax_reporting_us_1099_misc" => {
                        b.tax_reporting_us_1099_misc = FromValueOpt::from_value(v)
                    }
                    "transfers" => b.transfers = FromValueOpt::from_value(v),
                    "treasury" => b.treasury = FromValueOpt::from_value(v),
                    "twint_payments" => b.twint_payments = FromValueOpt::from_value(v),
                    "upi_payments" => b.upi_payments = FromValueOpt::from_value(v),
                    "us_bank_account_ach_payments" => {
                        b.us_bank_account_ach_payments = FromValueOpt::from_value(v)
                    }
                    "us_bank_transfer_payments" => {
                        b.us_bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "zip_payments" => b.zip_payments = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
pub type AccountCapabilitiesAcssDebitPayments = stripe_shared::MandateStatus;
/// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
pub type AccountCapabilitiesAffirmPayments = stripe_shared::MandateStatus;
/// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
pub type AccountCapabilitiesAfterpayClearpayPayments = stripe_shared::MandateStatus;
/// The status of the Alma capability of the account, or whether the account can directly process Alma payments.
pub type AccountCapabilitiesAlmaPayments = stripe_shared::MandateStatus;
/// The status of the AmazonPay capability of the account, or whether the account can directly process AmazonPay payments.
pub type AccountCapabilitiesAmazonPayPayments = stripe_shared::MandateStatus;
/// The status of the `app_distribution` capability of the account, or whether the platform can distribute apps to other accounts.
pub type AccountCapabilitiesAppDistribution = stripe_shared::MandateStatus;
/// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
pub type AccountCapabilitiesAuBecsDebitPayments = stripe_shared::MandateStatus;
/// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
pub type AccountCapabilitiesBacsDebitPayments = stripe_shared::MandateStatus;
/// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
pub type AccountCapabilitiesBancontactPayments = stripe_shared::MandateStatus;
/// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
pub type AccountCapabilitiesBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the Billie capability of the account, or whether the account can directly process Billie payments.
pub type AccountCapabilitiesBilliePayments = stripe_shared::MandateStatus;
/// The status of the Bizum capability of the account, or whether the account can directly process Bizum payments.
pub type AccountCapabilitiesBizumPayments = stripe_shared::MandateStatus;
/// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
pub type AccountCapabilitiesBlikPayments = stripe_shared::MandateStatus;
/// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
pub type AccountCapabilitiesBoletoPayments = stripe_shared::MandateStatus;
/// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
pub type AccountCapabilitiesCardIssuing = stripe_shared::MandateStatus;
/// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
pub type AccountCapabilitiesCardPayments = stripe_shared::MandateStatus;
/// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
pub type AccountCapabilitiesCartesBancairesPayments = stripe_shared::MandateStatus;
/// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
pub type AccountCapabilitiesCashappPayments = stripe_shared::MandateStatus;
/// The status of the Crypto capability of the account, or whether the account can directly process Crypto payments.
pub type AccountCapabilitiesCryptoPayments = stripe_shared::MandateStatus;
/// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
pub type AccountCapabilitiesEpsPayments = stripe_shared::MandateStatus;
/// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
pub type AccountCapabilitiesFpxPayments = stripe_shared::MandateStatus;
/// The status of the GB customer_balance payments (GBP currency) capability of the account, or whether the account can directly process GB customer_balance charges.
pub type AccountCapabilitiesGbBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
pub type AccountCapabilitiesGiropayPayments = stripe_shared::MandateStatus;
/// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
pub type AccountCapabilitiesGrabpayPayments = stripe_shared::MandateStatus;
/// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
pub type AccountCapabilitiesIdealPayments = stripe_shared::MandateStatus;
/// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
pub type AccountCapabilitiesIndiaInternationalPayments = stripe_shared::MandateStatus;
/// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
pub type AccountCapabilitiesJcbPayments = stripe_shared::MandateStatus;
/// The status of the Japanese customer_balance payments (JPY currency) capability of the account, or whether the account can directly process Japanese customer_balance charges.
pub type AccountCapabilitiesJpBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the KakaoPay capability of the account, or whether the account can directly process KakaoPay payments.
pub type AccountCapabilitiesKakaoPayPayments = stripe_shared::MandateStatus;
/// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
pub type AccountCapabilitiesKlarnaPayments = stripe_shared::MandateStatus;
/// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
pub type AccountCapabilitiesKonbiniPayments = stripe_shared::MandateStatus;
/// The status of the KrCard capability of the account, or whether the account can directly process KrCard payments.
pub type AccountCapabilitiesKrCardPayments = stripe_shared::MandateStatus;
/// The status of the legacy payments capability of the account.
pub type AccountCapabilitiesLegacyPayments = stripe_shared::MandateStatus;
/// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
pub type AccountCapabilitiesLinkPayments = stripe_shared::MandateStatus;
/// The status of the MB WAY payments capability of the account, or whether the account can directly process MB WAY charges.
pub type AccountCapabilitiesMbWayPayments = stripe_shared::MandateStatus;
/// The status of the MobilePay capability of the account, or whether the account can directly process MobilePay charges.
pub type AccountCapabilitiesMobilepayPayments = stripe_shared::MandateStatus;
/// The status of the Multibanco payments capability of the account, or whether the account can directly process Multibanco charges.
pub type AccountCapabilitiesMultibancoPayments = stripe_shared::MandateStatus;
/// The status of the Mexican customer_balance payments (MXN currency) capability of the account, or whether the account can directly process Mexican customer_balance charges.
pub type AccountCapabilitiesMxBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the NaverPay capability of the account, or whether the account can directly process NaverPay payments.
pub type AccountCapabilitiesNaverPayPayments = stripe_shared::MandateStatus;
/// The status of the New Zealand BECS Direct Debit payments capability of the account, or whether the account can directly process New Zealand BECS Direct Debit charges.
pub type AccountCapabilitiesNzBankAccountBecsDebitPayments = stripe_shared::MandateStatus;
/// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
pub type AccountCapabilitiesOxxoPayments = stripe_shared::MandateStatus;
/// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
pub type AccountCapabilitiesP24Payments = stripe_shared::MandateStatus;
/// The status of the pay_by_bank payments capability of the account, or whether the account can directly process pay_by_bank charges.
pub type AccountCapabilitiesPayByBankPayments = stripe_shared::MandateStatus;
/// The status of the Payco capability of the account, or whether the account can directly process Payco payments.
pub type AccountCapabilitiesPaycoPayments = stripe_shared::MandateStatus;
/// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
pub type AccountCapabilitiesPaynowPayments = stripe_shared::MandateStatus;
/// The status of the PayTo capability of the account, or whether the account can directly process PayTo charges.
pub type AccountCapabilitiesPaytoPayments = stripe_shared::MandateStatus;
/// The status of the pix payments capability of the account, or whether the account can directly process pix charges.
pub type AccountCapabilitiesPixPayments = stripe_shared::MandateStatus;
/// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
pub type AccountCapabilitiesPromptpayPayments = stripe_shared::MandateStatus;
/// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
pub type AccountCapabilitiesRevolutPayPayments = stripe_shared::MandateStatus;
/// The status of the SamsungPay capability of the account, or whether the account can directly process SamsungPay payments.
pub type AccountCapabilitiesSamsungPayPayments = stripe_shared::MandateStatus;
/// The status of the Satispay capability of the account, or whether the account can directly process Satispay payments.
pub type AccountCapabilitiesSatispayPayments = stripe_shared::MandateStatus;
/// The status of the Scalapay capability of the account, or whether the account can directly process Scalapay payments.
pub type AccountCapabilitiesScalapayPayments = stripe_shared::MandateStatus;
/// The status of the SEPA customer_balance payments (EUR currency) capability of the account, or whether the account can directly process SEPA customer_balance charges.
pub type AccountCapabilitiesSepaBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
pub type AccountCapabilitiesSepaDebitPayments = stripe_shared::MandateStatus;
/// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
pub type AccountCapabilitiesSofortPayments = stripe_shared::MandateStatus;
/// The status of the Sunbit capability of the account, or whether the account can directly process Sunbit payments.
pub type AccountCapabilitiesSunbitPayments = stripe_shared::MandateStatus;
/// The status of the Swish capability of the account, or whether the account can directly process Swish payments.
pub type AccountCapabilitiesSwishPayments = stripe_shared::MandateStatus;
/// The status of the tax reporting 1099-K (US) capability of the account.
pub type AccountCapabilitiesTaxReportingUs1099K = stripe_shared::MandateStatus;
/// The status of the tax reporting 1099-MISC (US) capability of the account.
pub type AccountCapabilitiesTaxReportingUs1099Misc = stripe_shared::MandateStatus;
/// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
pub type AccountCapabilitiesTransfers = stripe_shared::MandateStatus;
/// The status of the banking capability, or whether the account can have bank accounts.
pub type AccountCapabilitiesTreasury = stripe_shared::MandateStatus;
/// The status of the TWINT capability of the account, or whether the account can directly process TWINT charges.
pub type AccountCapabilitiesTwintPayments = stripe_shared::MandateStatus;
/// The status of the upi payments capability of the account, or whether the account can directly process upi charges.
pub type AccountCapabilitiesUpiPayments = stripe_shared::MandateStatus;
/// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
pub type AccountCapabilitiesUsBankAccountAchPayments = stripe_shared::MandateStatus;
/// The status of the US customer_balance payments (USD currency) capability of the account, or whether the account can directly process US customer_balance charges.
pub type AccountCapabilitiesUsBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
pub type AccountCapabilitiesZipPayments = stripe_shared::MandateStatus;
pub type AccountCapabilitiesStatus = stripe_shared::MandateStatus;
