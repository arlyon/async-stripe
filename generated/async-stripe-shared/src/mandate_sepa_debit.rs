#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandateSepaDebit {
    /// The unique reference of the mandate.
    pub reference: String,
    /// The URL of the mandate.
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}
#[doc(hidden)]
pub struct MandateSepaDebitBuilder {
    reference: Option<String>,
    url: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for MandateSepaDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateSepaDebit>,
        builder: MandateSepaDebitBuilder,
    }

    impl Visitor for Place<MandateSepaDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandateSepaDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandateSepaDebitBuilder {
        type Out = MandateSepaDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.reference),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { reference: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(reference), Some(url)) = (self.reference.take(), self.url.take()) else {
                return None;
            };
            Some(Self::Out { reference, url })
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

    impl ObjectDeser for MandateSepaDebit {
        type Builder = MandateSepaDebitBuilder;
    }

    impl FromValueOpt for MandateSepaDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandateSepaDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
