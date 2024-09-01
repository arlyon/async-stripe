#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundDestinationDetailsGeneric {
    /// The reference assigned to the refund.
    pub reference: Option<String>,
    /// Status of the reference on the refund. This can be `pending`, `available` or `unavailable`.
    pub reference_status: Option<String>,
}
#[doc(hidden)]
pub struct RefundDestinationDetailsGenericBuilder {
    reference: Option<Option<String>>,
    reference_status: Option<Option<String>>,
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

    impl Deserialize for RefundDestinationDetailsGeneric {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundDestinationDetailsGeneric>,
        builder: RefundDestinationDetailsGenericBuilder,
    }

    impl Visitor for Place<RefundDestinationDetailsGeneric> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RefundDestinationDetailsGenericBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RefundDestinationDetailsGenericBuilder {
        type Out = RefundDestinationDetailsGeneric;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.reference),
                "reference_status" => Deserialize::begin(&mut self.reference_status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reference: Deserialize::default(), reference_status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(reference), Some(reference_status)) =
                (self.reference.take(), self.reference_status.take())
            else {
                return None;
            };
            Some(Self::Out { reference, reference_status })
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

    impl ObjectDeser for RefundDestinationDetailsGeneric {
        type Builder = RefundDestinationDetailsGenericBuilder;
    }

    impl FromValueOpt for RefundDestinationDetailsGeneric {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RefundDestinationDetailsGenericBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "reference_status" => b.reference_status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
