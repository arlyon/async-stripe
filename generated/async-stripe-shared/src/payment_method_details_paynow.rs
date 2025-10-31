#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaynow {
    /// ID of the [location](https://stripe.com/docs/api/terminal/locations) that this transaction's reader is assigned to.
    pub location: Option<String>,
    /// ID of the [reader](https://stripe.com/docs/api/terminal/readers) this transaction was made on.
    pub reader: Option<String>,
    /// Reference number associated with this PayNow payment
    pub reference: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaynowBuilder {
    location: Option<Option<String>>,
    reader: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsPaynow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaynow>,
        builder: PaymentMethodDetailsPaynowBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaynow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaynowBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaynowBuilder {
        type Out = PaymentMethodDetailsPaynow;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "location" => Deserialize::begin(&mut self.location),
                "reader" => Deserialize::begin(&mut self.reader),
                "reference" => Deserialize::begin(&mut self.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                location: Deserialize::default(),
                reader: Deserialize::default(),
                reference: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(location), Some(reader), Some(reference)) =
                (self.location.take(), self.reader.take(), self.reference.take())
            else {
                return None;
            };
            Some(Self::Out { location, reader, reference })
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

    impl ObjectDeser for PaymentMethodDetailsPaynow {
        type Builder = PaymentMethodDetailsPaynowBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsPaynow {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsPaynowBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "location" => b.location = FromValueOpt::from_value(v),
                    "reader" => b.reader = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
