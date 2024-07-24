#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
#[doc(hidden)]
pub struct InvoiceLineItemPeriodBuilder {
    end: Option<stripe_types::Timestamp>,
    start: Option<stripe_types::Timestamp>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoiceLineItemPeriod {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceLineItemPeriod>,
        builder: InvoiceLineItemPeriodBuilder,
    }

    impl Visitor for Place<InvoiceLineItemPeriod> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceLineItemPeriodBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceLineItemPeriodBuilder {
        type Out = InvoiceLineItemPeriod;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end" => Deserialize::begin(&mut self.end),
                "start" => Deserialize::begin(&mut self.start),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { end: Deserialize::default(), start: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { end: self.end?, start: self.start? })
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

    impl ObjectDeser for InvoiceLineItemPeriod {
        type Builder = InvoiceLineItemPeriodBuilder;
    }

    impl FromValueOpt for InvoiceLineItemPeriod {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceLineItemPeriodBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "end" => b.end = Some(FromValueOpt::from_value(v)?),
                    "start" => b.start = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
