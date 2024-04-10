#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandatePaymentMethodDetails {
    pub acss_debit: Option<stripe_shared::MandateAcssDebit>,
    pub au_becs_debit: Option<stripe_shared::MandateAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::MandateBacsDebit>,
    pub card: Option<stripe_shared::CardMandatePaymentMethodDetails>,
    pub cashapp: Option<stripe_shared::MandateCashapp>,
    pub link: Option<stripe_shared::MandateLink>,
    pub paypal: Option<stripe_shared::MandatePaypal>,
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
    au_becs_debit: Option<Option<stripe_shared::MandateAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::MandateBacsDebit>>,
    card: Option<Option<stripe_shared::CardMandatePaymentMethodDetails>>,
    cashapp: Option<Option<stripe_shared::MandateCashapp>>,
    link: Option<Option<stripe_shared::MandateLink>>,
    paypal: Option<Option<stripe_shared::MandatePaypal>>,
    sepa_debit: Option<Option<stripe_shared::MandateSepaDebit>>,
    type_: Option<String>,
    us_bank_account: Option<Option<stripe_shared::MandateUsBankAccount>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "card" => Deserialize::begin(&mut self.card),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "link" => Deserialize::begin(&mut self.link),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                card: Deserialize::default(),
                cashapp: Deserialize::default(),
                link: Deserialize::default(),
                paypal: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                acss_debit: self.acss_debit.take()?,
                au_becs_debit: self.au_becs_debit.take()?,
                bacs_debit: self.bacs_debit.take()?,
                card: self.card?,
                cashapp: self.cashapp?,
                link: self.link?,
                paypal: self.paypal.take()?,
                sepa_debit: self.sepa_debit.take()?,
                type_: self.type_.take()?,
                us_bank_account: self.us_bank_account?,
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
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "au_becs_debit" => b.au_becs_debit = Some(FromValueOpt::from_value(v)?),
                    "bacs_debit" => b.bacs_debit = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "cashapp" => b.cashapp = Some(FromValueOpt::from_value(v)?),
                    "link" => b.link = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "us_bank_account" => b.us_bank_account = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
