#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>,
    pub amazon_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>,
    pub boleto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>,
    pub card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>,
    pub card_present: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>,
    pub ideal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>,
    pub kakao_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKakaoPay>,
    pub klarna: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>,
    pub kr_card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKrCard>,
    pub link: Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>,
    pub naver_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsNaverPay>,
    pub nz_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsNzBankAccount>,
    pub paypal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>,
    pub payto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPayto>,
    pub revolut_pay: Option<stripe_shared::SetupAttemptPaymentMethodDetailsRevolutPay>,
    pub sepa_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>,
    pub sofort: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUpi>,
    pub us_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttemptPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupAttemptPaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsBuilder {
    acss_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>>,
    amazon_pay: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAmazonPay>>,
    au_becs_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>>,
    boleto: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>>,
    card: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>>,
    card_present: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>>,
    cashapp: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>>,
    ideal: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>>,
    kakao_pay: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsKakaoPay>>,
    klarna: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>>,
    kr_card: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsKrCard>>,
    link: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>>,
    naver_pay: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsNaverPay>>,
    nz_bank_account: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsNzBankAccount>>,
    paypal: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>>,
    payto: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsPayto>>,
    revolut_pay: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>>,
    sofort: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>>,
    type_: Option<String>,
    upi: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsUpi>>,
    us_bank_account: Option<Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for SetupAttemptPaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetails>,
        builder: SetupAttemptPaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupAttemptPaymentMethodDetailsBuilder {
                    acss_debit: Deserialize::default(),
                    amazon_pay: Deserialize::default(),
                    au_becs_debit: Deserialize::default(),
                    bacs_debit: Deserialize::default(),
                    bancontact: Deserialize::default(),
                    boleto: Deserialize::default(),
                    card: Deserialize::default(),
                    card_present: Deserialize::default(),
                    cashapp: Deserialize::default(),
                    ideal: Deserialize::default(),
                    kakao_pay: Deserialize::default(),
                    klarna: Deserialize::default(),
                    kr_card: Deserialize::default(),
                    link: Deserialize::default(),
                    naver_pay: Deserialize::default(),
                    nz_bank_account: Deserialize::default(),
                    paypal: Deserialize::default(),
                    payto: Deserialize::default(),
                    revolut_pay: Deserialize::default(),
                    sepa_debit: Deserialize::default(),
                    sofort: Deserialize::default(),
                    type_: Deserialize::default(),
                    upi: Deserialize::default(),
                    us_bank_account: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.builder.acss_debit),
                "amazon_pay" => Deserialize::begin(&mut self.builder.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.builder.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.builder.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.builder.bancontact),
                "boleto" => Deserialize::begin(&mut self.builder.boleto),
                "card" => Deserialize::begin(&mut self.builder.card),
                "card_present" => Deserialize::begin(&mut self.builder.card_present),
                "cashapp" => Deserialize::begin(&mut self.builder.cashapp),
                "ideal" => Deserialize::begin(&mut self.builder.ideal),
                "kakao_pay" => Deserialize::begin(&mut self.builder.kakao_pay),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "kr_card" => Deserialize::begin(&mut self.builder.kr_card),
                "link" => Deserialize::begin(&mut self.builder.link),
                "naver_pay" => Deserialize::begin(&mut self.builder.naver_pay),
                "nz_bank_account" => Deserialize::begin(&mut self.builder.nz_bank_account),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                "payto" => Deserialize::begin(&mut self.builder.payto),
                "revolut_pay" => Deserialize::begin(&mut self.builder.revolut_pay),
                "sepa_debit" => Deserialize::begin(&mut self.builder.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.builder.sofort),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "upi" => Deserialize::begin(&mut self.builder.upi),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(acss_debit),
                Some(amazon_pay),
                Some(au_becs_debit),
                Some(bacs_debit),
                Some(bancontact),
                Some(boleto),
                Some(card),
                Some(card_present),
                Some(cashapp),
                Some(ideal),
                Some(kakao_pay),
                Some(klarna),
                Some(kr_card),
                Some(link),
                Some(naver_pay),
                Some(nz_bank_account),
                Some(paypal),
                Some(payto),
                Some(revolut_pay),
                Some(sepa_debit),
                Some(sofort),
                Some(type_),
                Some(upi),
                Some(us_bank_account),
            ) = (
                self.builder.acss_debit,
                self.builder.amazon_pay,
                self.builder.au_becs_debit,
                self.builder.bacs_debit,
                self.builder.bancontact.take(),
                self.builder.boleto,
                self.builder.card.take(),
                self.builder.card_present.take(),
                self.builder.cashapp,
                self.builder.ideal.take(),
                self.builder.kakao_pay,
                self.builder.klarna,
                self.builder.kr_card,
                self.builder.link,
                self.builder.naver_pay.take(),
                self.builder.nz_bank_account,
                self.builder.paypal,
                self.builder.payto,
                self.builder.revolut_pay,
                self.builder.sepa_debit,
                self.builder.sofort.take(),
                self.builder.type_.take(),
                self.builder.upi,
                self.builder.us_bank_account,
            )
            else {
                return Ok(());
            };
            *self.out = Some(SetupAttemptPaymentMethodDetails {
                acss_debit,
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                boleto,
                card,
                card_present,
                cashapp,
                ideal,
                kakao_pay,
                klarna,
                kr_card,
                link,
                naver_pay,
                nz_bank_account,
                paypal,
                payto,
                revolut_pay,
                sepa_debit,
                sofort,
                type_,
                upi,
                us_bank_account,
            });
            Ok(())
        }
    }
};
