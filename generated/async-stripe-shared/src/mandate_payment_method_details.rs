#[derive(Clone, Debug)]
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
    pub revolut_pay: Option<stripe_shared::MandateRevolutPay>,
    pub sepa_debit: Option<stripe_shared::MandateSepaDebit>,
    /// This mandate corresponds with a specific payment method type.
    /// The `payment_method_details` includes an additional hash with the same name and contains mandate information that's specific to that payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_account: Option<stripe_shared::MandateUsBankAccount>,
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
    revolut_pay: Option<Option<stripe_shared::MandateRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::MandateSepaDebit>>,
    type_: Option<String>,
    us_bank_account: Option<Option<stripe_shared::MandateUsBankAccount>>,
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
                builder: MandatePaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandatePaymentMethodDetailsBuilder {
        type Out = MandatePaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "card" => Deserialize::begin(&mut self.card),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "kakao_pay" => Deserialize::begin(&mut self.kakao_pay),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "kr_card" => Deserialize::begin(&mut self.kr_card),
                "link" => Deserialize::begin(&mut self.link),
                "naver_pay" => Deserialize::begin(&mut self.naver_pay),
                "nz_bank_account" => Deserialize::begin(&mut self.nz_bank_account),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
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
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                Some(revolut_pay),
                Some(sepa_debit),
                Some(type_),
                Some(us_bank_account),
            ) = (
                self.acss_debit.take(),
                self.amazon_pay,
                self.au_becs_debit.take(),
                self.bacs_debit.take(),
                self.card,
                self.cashapp,
                self.kakao_pay,
                self.klarna,
                self.kr_card,
                self.link,
                self.naver_pay,
                self.nz_bank_account,
                self.paypal.take(),
                self.revolut_pay,
                self.sepa_debit.take(),
                self.type_.take(),
                self.us_bank_account.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
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
                revolut_pay,
                sepa_debit,
                type_,
                us_bank_account,
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

    impl ObjectDeser for MandatePaymentMethodDetails {
        type Builder = MandatePaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for MandatePaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandatePaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = FromValueOpt::from_value(v),
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "au_becs_debit" => b.au_becs_debit = FromValueOpt::from_value(v),
                    "bacs_debit" => b.bacs_debit = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "cashapp" => b.cashapp = FromValueOpt::from_value(v),
                    "kakao_pay" => b.kakao_pay = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "kr_card" => b.kr_card = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "naver_pay" => b.naver_pay = FromValueOpt::from_value(v),
                    "nz_bank_account" => b.nz_bank_account = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),
                    "revolut_pay" => b.revolut_pay = FromValueOpt::from_value(v),
                    "sepa_debit" => b.sepa_debit = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "us_bank_account" => b.us_bank_account = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
