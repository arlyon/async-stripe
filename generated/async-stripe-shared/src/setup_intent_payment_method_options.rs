#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>,
    pub amazon_pay: Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>,
    pub card: Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>,
    pub card_present: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>,
    pub link: Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>,
    pub paypal: Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>,
    pub sepa_debit: Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>,
    pub us_bank_account: Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>,
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsAcssDebit>>,
    amazon_pay: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsAmazonPay>>,
    card: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCard>>,
    card_present: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCardPresent>>,
    link: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsLink>>,
    paypal: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsPaypal>>,
    sepa_debit: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsSepaDebit>>,
    us_bank_account: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsUsBankAccount>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "link" => Deserialize::begin(&mut self.link),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                amazon_pay: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                link: Deserialize::default(),
                paypal: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                acss_debit: self.acss_debit.take()?,
                amazon_pay: self.amazon_pay?,
                card: self.card.take()?,
                card_present: self.card_present?,
                link: self.link.take()?,
                paypal: self.paypal.take()?,
                sepa_debit: self.sepa_debit?,
                us_bank_account: self.us_bank_account.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
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
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "amazon_pay" => b.amazon_pay = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "card_present" => b.card_present = Some(FromValueOpt::from_value(v)?),
                    "link" => b.link = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "us_bank_account" => b.us_bank_account = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
