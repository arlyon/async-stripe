/// Represents custom text to be displayed when collecting the input using a reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceCustomText {
    /// Customize the default description for this input
    pub description: Option<String>,
    /// Customize the default label for this input's skip button
    pub skip_button: Option<String>,
    /// Customize the default label for this input's submit button
    pub submit_button: Option<String>,
    /// Customize the default title for this input
    pub title: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceCustomText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceCustomText").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceCustomTextBuilder {
    description: Option<Option<String>>,
    skip_button: Option<Option<String>>,
    submit_button: Option<Option<String>>,
    title: Option<Option<String>>,
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

    impl Deserialize for TerminalReaderReaderResourceCustomText {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceCustomText>,
        builder: TerminalReaderReaderResourceCustomTextBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceCustomText> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceCustomTextBuilder {
                    description: Deserialize::default(),
                    skip_button: Deserialize::default(),
                    submit_button: Deserialize::default(),
                    title: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.builder.description),
                "skip_button" => Deserialize::begin(&mut self.builder.skip_button),
                "submit_button" => Deserialize::begin(&mut self.builder.submit_button),
                "title" => Deserialize::begin(&mut self.builder.title),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(description), Some(skip_button), Some(submit_button), Some(title)) = (
                self.builder.description.take(),
                self.builder.skip_button.take(),
                self.builder.submit_button.take(),
                self.builder.title.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceCustomText {
                description,
                skip_button,
                submit_button,
                title,
            });
            Ok(())
        }
    }
};
