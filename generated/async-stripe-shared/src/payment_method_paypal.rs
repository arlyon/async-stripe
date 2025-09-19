#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodPaypal {
    /// Two-letter ISO code representing the buyer's country.
    /// Values are provided by PayPal directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub country: Option<String>,
    /// Owner's email. Values are provided by PayPal directly
    /// (if supported) at the time of authorization or settlement. They cannot be set or mutated.
    pub payer_email: Option<String>,
    /// PayPal account PayerID. This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodPaypalBuilder {
    country: Option<Option<String>>,
    payer_email: Option<Option<String>>,
    payer_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodPaypal>,
        builder: PaymentMethodPaypalBuilder,
    }

    impl Visitor for Place<PaymentMethodPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodPaypalBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodPaypalBuilder {
        type Out = PaymentMethodPaypal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                "payer_email" => Deserialize::begin(&mut self.payer_email),
                "payer_id" => Deserialize::begin(&mut self.payer_id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                country: Deserialize::default(),
                payer_email: Deserialize::default(),
                payer_id: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(country), Some(payer_email), Some(payer_id)) =
                (self.country.take(), self.payer_email.take(), self.payer_id.take())
            else {
                return None;
            };
            Some(Self::Out { country, payer_email, payer_id })
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

    impl ObjectDeser for PaymentMethodPaypal {
        type Builder = PaymentMethodPaypalBuilder;
    }

    impl FromValueOpt for PaymentMethodPaypal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodPaypalBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    "payer_email" => b.payer_email = FromValueOpt::from_value(v),
                    "payer_id" => b.payer_id = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
