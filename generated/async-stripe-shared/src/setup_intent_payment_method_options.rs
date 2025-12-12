#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>,
    pub amazon_pay: Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>,
    pub bacs_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsBacsDebit>,
    pub card: Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>,
    pub card_present: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>,
    pub klarna: Option<stripe_shared::SetupIntentPaymentMethodOptionsKlarna>,
    pub link: Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>,
    pub paypal: Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>,
    pub payto: Option<stripe_shared::SetupIntentPaymentMethodOptionsPayto>,
    pub sepa_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>,
    pub us_bank_account: Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>,
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>>,
    amazon_pay: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>>,
    bacs_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsBacsDebit>>,
    card: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>>,
    card_present: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>>,
    klarna: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsKlarna>>,
    link: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>>,
    paypal: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>>,
    payto: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsPayto>>,
    sepa_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>>,
    us_bank_account: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptions>,
        builder: SetupIntentPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsBuilder {
        type Out = SetupIntentPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "link" => Deserialize::begin(&mut self.link),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "payto" => Deserialize::begin(&mut self.payto),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                amazon_pay: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                klarna: Deserialize::default(),
                link: Deserialize::default(),
                paypal: Deserialize::default(),
                payto: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acss_debit),
                Some(amazon_pay),
                Some(bacs_debit),
                Some(card),
                Some(card_present),
                Some(klarna),
                Some(link),
                Some(paypal),
                Some(payto),
                Some(sepa_debit),
                Some(us_bank_account),
            ) = (
                self.acss_debit.take(),
                self.amazon_pay,
                self.bacs_debit.take(),
                self.card.take(),
                self.card_present,
                self.klarna.take(),
                self.link.take(),
                self.paypal.take(),
                self.payto.take(),
                self.sepa_debit.take(),
                self.us_bank_account.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                acss_debit,
                amazon_pay,
                bacs_debit,
                card,
                card_present,
                klarna,
                link,
                paypal,
                payto,
                sepa_debit,
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

    impl ObjectDeser for SetupIntentPaymentMethodOptions {
        type Builder = SetupIntentPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = FromValueOpt::from_value(v),
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "bacs_debit" => b.bacs_debit = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "card_present" => b.card_present = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),
                    "payto" => b.payto = FromValueOpt::from_value(v),
                    "sepa_debit" => b.sepa_debit = FromValueOpt::from_value(v),
                    "us_bank_account" => b.us_bank_account = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
