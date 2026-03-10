/// Information about a phone number being collected using a reader
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourcePhone {
    /// The collected phone number
    pub value: Option<String>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourcePhoneBuilder {
    value: Option<Option<String>>,
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

    impl Deserialize for TerminalReaderReaderResourcePhone {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourcePhone>,
        builder: TerminalReaderReaderResourcePhoneBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourcePhone> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourcePhoneBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourcePhoneBuilder {
        type Out = TerminalReaderReaderResourcePhone;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "value" => Deserialize::begin(&mut self.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(value),) = (self.value.take(),) else {
                return None;
            };
            Some(Self::Out { value })
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

    impl ObjectDeser for TerminalReaderReaderResourcePhone {
        type Builder = TerminalReaderReaderResourcePhoneBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourcePhone {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourcePhoneBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "value" => b.value = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
