#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceMandateNotificationAcssDebitData {
    /// The statement descriptor associate with the debit.
    pub statement_descriptor: Option<String>,
}
#[doc(hidden)]
pub struct SourceMandateNotificationAcssDebitDataBuilder {
    statement_descriptor: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceMandateNotificationAcssDebitData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceMandateNotificationAcssDebitData>,
        builder: SourceMandateNotificationAcssDebitDataBuilder,
    }

    impl Visitor for Place<SourceMandateNotificationAcssDebitData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceMandateNotificationAcssDebitDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceMandateNotificationAcssDebitDataBuilder {
        type Out = SourceMandateNotificationAcssDebitData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { statement_descriptor: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { statement_descriptor: self.statement_descriptor.take()? })
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

    impl ObjectDeser for SourceMandateNotificationAcssDebitData {
        type Builder = SourceMandateNotificationAcssDebitDataBuilder;
    }

    impl FromValueOpt for SourceMandateNotificationAcssDebitData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceMandateNotificationAcssDebitDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "statement_descriptor" => {
                        b.statement_descriptor = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
