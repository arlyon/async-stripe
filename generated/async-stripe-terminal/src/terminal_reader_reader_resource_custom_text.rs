/// Represents custom text to be displayed when collecting the input using a reader
#[derive(Clone, Debug)]
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
                builder: TerminalReaderReaderResourceCustomTextBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceCustomTextBuilder {
        type Out = TerminalReaderReaderResourceCustomText;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.description),
                "skip_button" => Deserialize::begin(&mut self.skip_button),
                "submit_button" => Deserialize::begin(&mut self.submit_button),
                "title" => Deserialize::begin(&mut self.title),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                description: Deserialize::default(),
                skip_button: Deserialize::default(),
                submit_button: Deserialize::default(),
                title: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(description), Some(skip_button), Some(submit_button), Some(title)) = (
                self.description.take(),
                self.skip_button.take(),
                self.submit_button.take(),
                self.title.take(),
            ) else {
                return None;
            };
            Some(Self::Out { description, skip_button, submit_button, title })
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

    impl ObjectDeser for TerminalReaderReaderResourceCustomText {
        type Builder = TerminalReaderReaderResourceCustomTextBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceCustomText {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceCustomTextBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "description" => b.description = FromValueOpt::from_value(v),
                    "skip_button" => b.skip_button = FromValueOpt::from_value(v),
                    "submit_button" => b.submit_button = FromValueOpt::from_value(v),
                    "title" => b.title = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
