#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputePaymentMethodDetailsPaypal {
    /// The ID of the dispute in PayPal.
    pub case_id: Option<String>,
    /// The reason for the dispute as defined by PayPal
    pub reason_code: Option<String>,
}
#[doc(hidden)]
pub struct DisputePaymentMethodDetailsPaypalBuilder {
    case_id: Option<Option<String>>,
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

    impl Deserialize for DisputePaymentMethodDetailsPaypal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputePaymentMethodDetailsPaypal>,
        builder: DisputePaymentMethodDetailsPaypalBuilder,
    }

    impl Visitor for Place<DisputePaymentMethodDetailsPaypal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputePaymentMethodDetailsPaypalBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputePaymentMethodDetailsPaypalBuilder {
        type Out = DisputePaymentMethodDetailsPaypal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "case_id" => Deserialize::begin(&mut self.case_id),
                "reason_code" => Deserialize::begin(&mut self.reason_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { case_id: Deserialize::default(), reason_code: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(case_id), Some(reason_code)) = (self.case_id.take(), self.reason_code.take())
            else {
                return None;
            };
            Some(Self::Out { case_id, reason_code })
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

    impl ObjectDeser for DisputePaymentMethodDetailsPaypal {
        type Builder = DisputePaymentMethodDetailsPaypalBuilder;
    }

    impl FromValueOpt for DisputePaymentMethodDetailsPaypal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputePaymentMethodDetailsPaypalBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "case_id" => b.case_id = FromValueOpt::from_value(v),
                    "reason_code" => b.reason_code = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
