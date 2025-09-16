#[derive(Copy, Clone, Debug)]
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
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    pub us_bank_account_ach_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the US customer_balance payments (USD currency) capability of the account, or whether the account can directly process US customer_balance charges.
    pub us_bank_transfer_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    pub zip_payments: Option<stripe_shared::AccountCapabilitiesStatus>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                acss_debit_payments: Deserialize::default(),
                affirm_payments: Deserialize::default(),
                afterpay_clearpay_payments: Deserialize::default(),
                alma_payments: Deserialize::default(),
                amazon_pay_payments: Deserialize::default(),
                au_becs_debit_payments: Deserialize::default(),
                bacs_debit_payments: Deserialize::default(),
                bancontact_payments: Deserialize::default(),
                bank_transfer_payments: Deserialize::default(),
                billie_payments: Deserialize::default(),
                blik_payments: Deserialize::default(),
                boleto_payments: Deserialize::default(),
                card_issuing: Deserialize::default(),
                card_payments: Deserialize::default(),
                cartes_bancaires_payments: Deserialize::default(),
                cashapp_payments: Deserialize::default(),
                crypto_payments: Deserialize::default(),
                eps_payments: Deserialize::default(),
                fpx_payments: Deserialize::default(),
                gb_bank_transfer_payments: Deserialize::default(),
                giropay_payments: Deserialize::default(),
                grabpay_payments: Deserialize::default(),
                ideal_payments: Deserialize::default(),
                india_international_payments: Deserialize::default(),
                jcb_payments: Deserialize::default(),
                jp_bank_transfer_payments: Deserialize::default(),
                kakao_pay_payments: Deserialize::default(),
                klarna_payments: Deserialize::default(),
                konbini_payments: Deserialize::default(),
                kr_card_payments: Deserialize::default(),
                legacy_payments: Deserialize::default(),
                link_payments: Deserialize::default(),
                mobilepay_payments: Deserialize::default(),
                multibanco_payments: Deserialize::default(),
                mx_bank_transfer_payments: Deserialize::default(),
                naver_pay_payments: Deserialize::default(),
                nz_bank_account_becs_debit_payments: Deserialize::default(),
                oxxo_payments: Deserialize::default(),
                p24_payments: Deserialize::default(),
                pay_by_bank_payments: Deserialize::default(),
                payco_payments: Deserialize::default(),
                paynow_payments: Deserialize::default(),
                pix_payments: Deserialize::default(),
                promptpay_payments: Deserialize::default(),
                revolut_pay_payments: Deserialize::default(),
                samsung_pay_payments: Deserialize::default(),
                satispay_payments: Deserialize::default(),
                sepa_bank_transfer_payments: Deserialize::default(),
                sepa_debit_payments: Deserialize::default(),
                sofort_payments: Deserialize::default(),
                swish_payments: Deserialize::default(),
                tax_reporting_us_1099_k: Deserialize::default(),
                tax_reporting_us_1099_misc: Deserialize::default(),
                transfers: Deserialize::default(),
                treasury: Deserialize::default(),
                twint_payments: Deserialize::default(),
                us_bank_account_ach_payments: Deserialize::default(),
                us_bank_transfer_payments: Deserialize::default(),
                zip_payments: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acss_debit_payments),
                Some(affirm_payments),
                Some(afterpay_clearpay_payments),
                Some(alma_payments),
                Some(amazon_pay_payments),
                Some(au_becs_debit_payments),
                Some(bacs_debit_payments),
                Some(bancontact_payments),
                Some(bank_transfer_payments),
                Some(billie_payments),
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
                Some(pix_payments),
                Some(promptpay_payments),
                Some(revolut_pay_payments),
                Some(samsung_pay_payments),
                Some(satispay_payments),
                Some(sepa_bank_transfer_payments),
                Some(sepa_debit_payments),
                Some(sofort_payments),
                Some(swish_payments),
                Some(tax_reporting_us_1099_k),
                Some(tax_reporting_us_1099_misc),
                Some(transfers),
                Some(treasury),
                Some(twint_payments),
                Some(us_bank_account_ach_payments),
                Some(us_bank_transfer_payments),
                Some(zip_payments),
            ) = (
                self.acss_debit_payments,
                self.affirm_payments,
                self.afterpay_clearpay_payments,
                self.alma_payments,
                self.amazon_pay_payments,
                self.au_becs_debit_payments,
                self.bacs_debit_payments,
                self.bancontact_payments,
                self.bank_transfer_payments,
                self.billie_payments,
                self.blik_payments,
                self.boleto_payments,
                self.card_issuing,
                self.card_payments,
                self.cartes_bancaires_payments,
                self.cashapp_payments,
                self.crypto_payments,
                self.eps_payments,
                self.fpx_payments,
                self.gb_bank_transfer_payments,
                self.giropay_payments,
                self.grabpay_payments,
                self.ideal_payments,
                self.india_international_payments,
                self.jcb_payments,
                self.jp_bank_transfer_payments,
                self.kakao_pay_payments,
                self.klarna_payments,
                self.konbini_payments,
                self.kr_card_payments,
                self.legacy_payments,
                self.link_payments,
                self.mobilepay_payments,
                self.multibanco_payments,
                self.mx_bank_transfer_payments,
                self.naver_pay_payments,
                self.nz_bank_account_becs_debit_payments,
                self.oxxo_payments,
                self.p24_payments,
                self.pay_by_bank_payments,
                self.payco_payments,
                self.paynow_payments,
                self.pix_payments,
                self.promptpay_payments,
                self.revolut_pay_payments,
                self.samsung_pay_payments,
                self.satispay_payments,
                self.sepa_bank_transfer_payments,
                self.sepa_debit_payments,
                self.sofort_payments,
                self.swish_payments,
                self.tax_reporting_us_1099_k,
                self.tax_reporting_us_1099_misc,
                self.transfers,
                self.treasury,
                self.twint_payments,
                self.us_bank_account_ach_payments,
                self.us_bank_transfer_payments,
                self.zip_payments,
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
                au_becs_debit_payments,
                bacs_debit_payments,
                bancontact_payments,
                bank_transfer_payments,
                billie_payments,
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
                pix_payments,
                promptpay_payments,
                revolut_pay_payments,
                samsung_pay_payments,
                satispay_payments,
                sepa_bank_transfer_payments,
                sepa_debit_payments,
                sofort_payments,
                swish_payments,
                tax_reporting_us_1099_k,
                tax_reporting_us_1099_misc,
                transfers,
                treasury,
                twint_payments,
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesStatus {
    Active,
    Inactive,
    Pending,
}
impl AccountCapabilitiesStatus {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AccountCapabilitiesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        self.out = Some(AccountCapabilitiesStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountCapabilitiesStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountCapabilitiesStatus"))
    }
}
