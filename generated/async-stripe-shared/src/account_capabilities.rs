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
    /// The status of the SEPA customer_balance payments (EUR currency) capability of the account, or whether the account can directly process SEPA customer_balance charges.
    pub sepa_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    pub sepa_debit_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    pub sofort_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
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
    acss_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    affirm_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    afterpay_clearpay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    alma_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    amazon_pay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    au_becs_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    bacs_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    bancontact_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    bank_transfer_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    billie_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    blik_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    boleto_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    card_issuing: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    card_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    cartes_bancaires_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    cashapp_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    crypto_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    eps_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    fpx_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    gb_bank_transfer_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    giropay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    grabpay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    ideal_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    india_international_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    jcb_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    jp_bank_transfer_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    kakao_pay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    klarna_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    konbini_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    kr_card_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    legacy_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    link_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    mb_way_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    mobilepay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    multibanco_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    mx_bank_transfer_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    naver_pay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    nz_bank_account_becs_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    oxxo_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    p24_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    pay_by_bank_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    payco_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    paynow_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    payto_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    pix_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    promptpay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    revolut_pay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    samsung_pay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    satispay_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    sepa_bank_transfer_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    sepa_debit_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    sofort_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    swish_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    tax_reporting_us_1099_k: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    tax_reporting_us_1099_misc: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    transfers: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    treasury: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    twint_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    upi_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    us_bank_account_ach_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    us_bank_transfer_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
    zip_payments: Option<Option<stripe_shared::AccountCapabilitiesStatus>>,
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
                "au_becs_debit_payments" => Deserialize::begin(&mut self.au_becs_debit_payments),
                "bacs_debit_payments" => Deserialize::begin(&mut self.bacs_debit_payments),
                "bancontact_payments" => Deserialize::begin(&mut self.bancontact_payments),
                "bank_transfer_payments" => Deserialize::begin(&mut self.bank_transfer_payments),
                "billie_payments" => Deserialize::begin(&mut self.billie_payments),
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
                "sepa_bank_transfer_payments" => {
                    Deserialize::begin(&mut self.sepa_bank_transfer_payments)
                }
                "sepa_debit_payments" => Deserialize::begin(&mut self.sepa_debit_payments),
                "sofort_payments" => Deserialize::begin(&mut self.sofort_payments),
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
                au_becs_debit_payments: Some(None),
                bacs_debit_payments: Some(None),
                bancontact_payments: Some(None),
                bank_transfer_payments: Some(None),
                billie_payments: Some(None),
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
                sepa_bank_transfer_payments: Some(None),
                sepa_debit_payments: Some(None),
                sofort_payments: Some(None),
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
            Some(Self::Out {
                acss_debit_payments: self.acss_debit_payments.take().flatten(),
                affirm_payments: self.affirm_payments.take().flatten(),
                afterpay_clearpay_payments: self.afterpay_clearpay_payments.take().flatten(),
                alma_payments: self.alma_payments.take().flatten(),
                amazon_pay_payments: self.amazon_pay_payments.take().flatten(),
                au_becs_debit_payments: self.au_becs_debit_payments.take().flatten(),
                bacs_debit_payments: self.bacs_debit_payments.take().flatten(),
                bancontact_payments: self.bancontact_payments.take().flatten(),
                bank_transfer_payments: self.bank_transfer_payments.take().flatten(),
                billie_payments: self.billie_payments.take().flatten(),
                blik_payments: self.blik_payments.take().flatten(),
                boleto_payments: self.boleto_payments.take().flatten(),
                card_issuing: self.card_issuing.take().flatten(),
                card_payments: self.card_payments.take().flatten(),
                cartes_bancaires_payments: self.cartes_bancaires_payments.take().flatten(),
                cashapp_payments: self.cashapp_payments.take().flatten(),
                crypto_payments: self.crypto_payments.take().flatten(),
                eps_payments: self.eps_payments.take().flatten(),
                fpx_payments: self.fpx_payments.take().flatten(),
                gb_bank_transfer_payments: self.gb_bank_transfer_payments.take().flatten(),
                giropay_payments: self.giropay_payments.take().flatten(),
                grabpay_payments: self.grabpay_payments.take().flatten(),
                ideal_payments: self.ideal_payments.take().flatten(),
                india_international_payments: self.india_international_payments.take().flatten(),
                jcb_payments: self.jcb_payments.take().flatten(),
                jp_bank_transfer_payments: self.jp_bank_transfer_payments.take().flatten(),
                kakao_pay_payments: self.kakao_pay_payments.take().flatten(),
                klarna_payments: self.klarna_payments.take().flatten(),
                konbini_payments: self.konbini_payments.take().flatten(),
                kr_card_payments: self.kr_card_payments.take().flatten(),
                legacy_payments: self.legacy_payments.take().flatten(),
                link_payments: self.link_payments.take().flatten(),
                mb_way_payments: self.mb_way_payments.take().flatten(),
                mobilepay_payments: self.mobilepay_payments.take().flatten(),
                multibanco_payments: self.multibanco_payments.take().flatten(),
                mx_bank_transfer_payments: self.mx_bank_transfer_payments.take().flatten(),
                naver_pay_payments: self.naver_pay_payments.take().flatten(),
                nz_bank_account_becs_debit_payments: self
                    .nz_bank_account_becs_debit_payments
                    .take()
                    .flatten(),
                oxxo_payments: self.oxxo_payments.take().flatten(),
                p24_payments: self.p24_payments.take().flatten(),
                pay_by_bank_payments: self.pay_by_bank_payments.take().flatten(),
                payco_payments: self.payco_payments.take().flatten(),
                paynow_payments: self.paynow_payments.take().flatten(),
                payto_payments: self.payto_payments.take().flatten(),
                pix_payments: self.pix_payments.take().flatten(),
                promptpay_payments: self.promptpay_payments.take().flatten(),
                revolut_pay_payments: self.revolut_pay_payments.take().flatten(),
                samsung_pay_payments: self.samsung_pay_payments.take().flatten(),
                satispay_payments: self.satispay_payments.take().flatten(),
                sepa_bank_transfer_payments: self.sepa_bank_transfer_payments.take().flatten(),
                sepa_debit_payments: self.sepa_debit_payments.take().flatten(),
                sofort_payments: self.sofort_payments.take().flatten(),
                swish_payments: self.swish_payments.take().flatten(),
                tax_reporting_us_1099_k: self.tax_reporting_us_1099_k.take().flatten(),
                tax_reporting_us_1099_misc: self.tax_reporting_us_1099_misc.take().flatten(),
                transfers: self.transfers.take().flatten(),
                treasury: self.treasury.take().flatten(),
                twint_payments: self.twint_payments.take().flatten(),
                upi_payments: self.upi_payments.take().flatten(),
                us_bank_account_ach_payments: self.us_bank_account_ach_payments.take().flatten(),
                us_bank_transfer_payments: self.us_bank_transfer_payments.take().flatten(),
                zip_payments: self.zip_payments.take().flatten(),
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
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
                    "au_becs_debit_payments" => {
                        b.au_becs_debit_payments = FromValueOpt::from_value(v)
                    }
                    "bacs_debit_payments" => b.bacs_debit_payments = FromValueOpt::from_value(v),
                    "bancontact_payments" => b.bancontact_payments = FromValueOpt::from_value(v),
                    "bank_transfer_payments" => {
                        b.bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "billie_payments" => b.billie_payments = FromValueOpt::from_value(v),
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
                    "sepa_bank_transfer_payments" => {
                        b.sepa_bank_transfer_payments = FromValueOpt::from_value(v)
                    }
                    "sepa_debit_payments" => b.sepa_debit_payments = FromValueOpt::from_value(v),
                    "sofort_payments" => b.sofort_payments = FromValueOpt::from_value(v),
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
