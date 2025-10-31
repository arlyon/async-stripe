#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetailsKlarna {
    /// Chargeback loss reason mapped by Stripe from Klarna's chargeback loss reason
    pub chargeback_loss_reason_code: Option<String>,
    /// The reason for the dispute as defined by Klarna
    pub reason_code: Option<String>,
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsKlarnaBuilder {
    chargeback_loss_reason_code: Option<Option<String>>,
    reason_code: Option<Option<String>>,
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

    impl Deserialize for DisputePaymentMethodDetailsKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetailsKlarna>,
        builder: DisputePaymentMethodDetailsKlarnaBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetailsKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsKlarnaBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputePaymentMethodDetailsKlarnaBuilder {
        type Out = DisputePaymentMethodDetailsKlarna;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chargeback_loss_reason_code" => {
                    Deserialize::begin(&mut self.chargeback_loss_reason_code)
                }
                "reason_code" => Deserialize::begin(&mut self.reason_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                chargeback_loss_reason_code: Deserialize::default(),
                reason_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(chargeback_loss_reason_code), Some(reason_code)) =
                (self.chargeback_loss_reason_code.take(), self.reason_code.take())
            else {
                return None;
            };
            Some(Self::Out { chargeback_loss_reason_code, reason_code })
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

    impl ObjectDeser for DisputePaymentMethodDetailsKlarna {
        type Builder = DisputePaymentMethodDetailsKlarnaBuilder;
    }

    impl FromValueOpt for DisputePaymentMethodDetailsKlarna {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputePaymentMethodDetailsKlarnaBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "chargeback_loss_reason_code" => {
                        b.chargeback_loss_reason_code = FromValueOpt::from_value(v)
                    }
                    "reason_code" => b.reason_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
