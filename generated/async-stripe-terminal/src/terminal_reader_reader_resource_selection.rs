/// Information about a selection being collected using a reader
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceSelection {
    /// List of possible choices to be selected
    pub choices: Vec<stripe_terminal::TerminalReaderReaderResourceChoice>,
    /// The id of the selected choice
    pub id: Option<String>,
    /// The text of the selected choice
    pub text: Option<String>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceSelectionBuilder {
    choices: Option<Vec<stripe_terminal::TerminalReaderReaderResourceChoice>>,
    id: Option<Option<String>>,
    text: Option<Option<String>>,
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

    impl Deserialize for TerminalReaderReaderResourceSelection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceSelection>,
        builder: TerminalReaderReaderResourceSelectionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceSelection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceSelectionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceSelectionBuilder {
        type Out = TerminalReaderReaderResourceSelection;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "choices" => Deserialize::begin(&mut self.choices),
                "id" => Deserialize::begin(&mut self.id),
                "text" => Deserialize::begin(&mut self.text),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                choices: Deserialize::default(),
                id: Deserialize::default(),
                text: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(choices), Some(id), Some(text)) =
                (self.choices.take(), self.id.take(), self.text.take())
            else {
                return None;
            };
            Some(Self::Out { choices, id, text })
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

    impl ObjectDeser for TerminalReaderReaderResourceSelection {
        type Builder = TerminalReaderReaderResourceSelectionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceSelection {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceSelectionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "choices" => b.choices = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "text" => b.text = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
