#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsRevolutPay {
    pub funding: Option<stripe_shared::RevolutPayUnderlyingPaymentMethodFundingDetails>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsRevolutPayBuilder {
    funding: Option<Option<stripe_shared::RevolutPayUnderlyingPaymentMethodFundingDetails>>,
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

    impl Deserialize for PaymentMethodDetailsRevolutPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsRevolutPay>,
        builder: PaymentMethodDetailsRevolutPayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsRevolutPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsRevolutPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsRevolutPayBuilder {
        type Out = PaymentMethodDetailsRevolutPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "funding" => Deserialize::begin(&mut self.funding),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { funding: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(funding),) = (self.funding.take(),) else {
                return None;
            };
            Some(Self::Out { funding })
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

    impl ObjectDeser for PaymentMethodDetailsRevolutPay {
        type Builder = PaymentMethodDetailsRevolutPayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsRevolutPay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsRevolutPayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "funding" => b.funding = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
