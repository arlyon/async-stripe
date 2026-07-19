#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandatePaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::MandateAcssDebit>,
    pub amazon_pay: Option<stripe_shared::MandateAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::MandateAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::MandateBacsDebit>,
    pub card: Option<stripe_shared::CardMandatePaymentMethodDetails>,
    pub cashapp: Option<stripe_shared::MandateCashapp>,
    pub kakao_pay: Option<stripe_shared::MandateKakaoPay>,
    pub klarna: Option<stripe_shared::MandateKlarna>,
    pub kr_card: Option<stripe_shared::MandateKrCard>,
    pub link: Option<stripe_shared::MandateLink>,
    pub naver_pay: Option<stripe_shared::MandateNaverPay>,
    pub nz_bank_account: Option<stripe_shared::MandateNzBankAccount>,
    pub paypal: Option<stripe_shared::MandatePaypal>,
    pub payto: Option<stripe_shared::MandatePayto>,
    pub revolut_pay: Option<stripe_shared::MandateRevolutPay>,
    pub sepa_debit: Option<stripe_shared::MandateSepaDebit>,
    /// This mandate corresponds with a specific payment method type.
    /// The `payment_method_details` includes an additional hash with the same name and contains mandate information that's specific to that payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi: Option<stripe_shared::MandateUpi>,
    pub us_bank_account: Option<stripe_shared::MandateUsBankAccount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MandatePaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct MandatePaymentMethodDetailsBuilder {
    acss_debit: Option<Option<stripe_shared::MandateAcssDebit>>,
    amazon_pay: Option<Option<stripe_shared::MandateAmazonPay>>,
    au_becs_debit: Option<Option<stripe_shared::MandateAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::MandateBacsDebit>>,
    card: Option<Option<stripe_shared::CardMandatePaymentMethodDetails>>,
    cashapp: Option<Option<stripe_shared::MandateCashapp>>,
    kakao_pay: Option<Option<stripe_shared::MandateKakaoPay>>,
    klarna: Option<Option<stripe_shared::MandateKlarna>>,
    kr_card: Option<Option<stripe_shared::MandateKrCard>>,
    link: Option<Option<stripe_shared::MandateLink>>,
    naver_pay: Option<Option<stripe_shared::MandateNaverPay>>,
    nz_bank_account: Option<Option<stripe_shared::MandateNzBankAccount>>,
    paypal: Option<Option<stripe_shared::MandatePaypal>>,
    payto: Option<Option<stripe_shared::MandatePayto>>,
    revolut_pay: Option<Option<stripe_shared::MandateRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::MandateSepaDebit>>,
    type_: Option<String>,
    upi: Option<Option<stripe_shared::MandateUpi>>,
    us_bank_account: Option<Option<stripe_shared::MandateUsBankAccount>>,
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

    impl Deserialize for MandatePaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandatePaymentMethodDetails>,
        builder: MandatePaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<MandatePaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandatePaymentMethodDetailsBuilder {
                    acss_debit: Deserialize::default(),
                    amazon_pay: Deserialize::default(),
                    au_becs_debit: Deserialize::default(),
                    bacs_debit: Deserialize::default(),
                    card: Deserialize::default(),
                    cashapp: Deserialize::default(),
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
                "card" => Deserialize::begin(&mut self.builder.card),
                "cashapp" => Deserialize::begin(&mut self.builder.cashapp),
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
                Some(card),
                Some(cashapp),
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
                Some(type_),
                Some(upi),
                Some(us_bank_account),
            ) = (
                self.builder.acss_debit.take(),
                self.builder.amazon_pay,
                self.builder.au_becs_debit.take(),
                self.builder.bacs_debit.take(),
                self.builder.card,
                self.builder.cashapp,
                self.builder.kakao_pay,
                self.builder.klarna,
                self.builder.kr_card,
                self.builder.link,
                self.builder.naver_pay,
                self.builder.nz_bank_account,
                self.builder.paypal.take(),
                self.builder.payto.take(),
                self.builder.revolut_pay,
                self.builder.sepa_debit.take(),
                self.builder.type_.take(),
                self.builder.upi.take(),
                self.builder.us_bank_account.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(MandatePaymentMethodDetails {
                acss_debit,
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                card,
                cashapp,
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
                type_,
                upi,
                us_bank_account,
            });
            Ok(())
        }
    }
};
