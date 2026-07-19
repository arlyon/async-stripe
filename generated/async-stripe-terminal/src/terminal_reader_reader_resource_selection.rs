/// Information about a selection being collected using a reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceSelection").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TerminalReaderReaderResourceSelectionBuilder {
                    choices: Deserialize::default(),
                    id: Deserialize::default(),
                    text: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "choices" => Deserialize::begin(&mut self.builder.choices),
                "id" => Deserialize::begin(&mut self.builder.id),
                "text" => Deserialize::begin(&mut self.builder.text),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(choices), Some(id), Some(text)) =
                (self.builder.choices.take(), self.builder.id.take(), self.builder.text.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceSelection { choices, id, text });
            Ok(())
        }
    }
};
