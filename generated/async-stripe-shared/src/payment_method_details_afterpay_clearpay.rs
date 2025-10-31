#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAfterpayClearpay {
    /// The Afterpay order ID associated with this payment intent.
    pub order_id: Option<String>,
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsAfterpayClearpayBuilder {
    order_id: Option<Option<String>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsAfterpayClearpay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAfterpayClearpay>,
        builder: PaymentMethodDetailsAfterpayClearpayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAfterpayClearpay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsAfterpayClearpayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsAfterpayClearpayBuilder {
        type Out = PaymentMethodDetailsAfterpayClearpay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "order_id" => Deserialize::begin(&mut self.order_id),
                "reference" => Deserialize::begin(&mut self.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { order_id: Deserialize::default(), reference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(order_id), Some(reference)) = (self.order_id.take(), self.reference.take())
            else {
                return None;
            };
            Some(Self::Out { order_id, reference })
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

    impl ObjectDeser for PaymentMethodDetailsAfterpayClearpay {
        type Builder = PaymentMethodDetailsAfterpayClearpayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsAfterpayClearpay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsAfterpayClearpayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "order_id" => b.order_id = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
