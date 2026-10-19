/// Represents an input to be collected using the reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceInput {
    /// Default text of input being collected.
    pub custom_text: Option<stripe_terminal::TerminalReaderReaderResourceCustomText>,
    pub email: Option<stripe_terminal::TerminalReaderReaderResourceEmail>,
    pub numeric: Option<stripe_terminal::TerminalReaderReaderResourceNumeric>,
    pub phone: Option<stripe_terminal::TerminalReaderReaderResourcePhone>,
    /// Indicate that this input is required, disabling the skip button.
    pub required: Option<bool>,
    pub selection: Option<stripe_terminal::TerminalReaderReaderResourceSelection>,
    pub signature: Option<stripe_terminal::TerminalReaderReaderResourceSignature>,
    /// Indicate that this input was skipped by the user.
    pub skipped: Option<bool>,
    pub text: Option<stripe_terminal::TerminalReaderReaderResourceText>,
    /// List of toggles being collected. Values are present if collection is complete.
    pub toggles: Option<Vec<stripe_terminal::TerminalReaderReaderResourceToggle>>,
    /// Type of input being collected.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CollectInputsTerminalReaderInputsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceInput").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceInputBuilder {
    custom_text: Option<Option<stripe_terminal::TerminalReaderReaderResourceCustomText>>,
    email: Option<Option<stripe_terminal::TerminalReaderReaderResourceEmail>>,
    numeric: Option<Option<stripe_terminal::TerminalReaderReaderResourceNumeric>>,
    phone: Option<Option<stripe_terminal::TerminalReaderReaderResourcePhone>>,
    required: Option<Option<bool>>,
    selection: Option<Option<stripe_terminal::TerminalReaderReaderResourceSelection>>,
    signature: Option<Option<stripe_terminal::TerminalReaderReaderResourceSignature>>,
    skipped: Option<Option<bool>>,
    text: Option<Option<stripe_terminal::TerminalReaderReaderResourceText>>,
    toggles: Option<Option<Vec<stripe_terminal::TerminalReaderReaderResourceToggle>>>,
    type_: Option<stripe_shared::CollectInputsTerminalReaderInputsType>,
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

    impl Deserialize for TerminalReaderReaderResourceInput {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceInput>,
        builder: TerminalReaderReaderResourceInputBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceInput> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceInputBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceInputBuilder {
        type Out = TerminalReaderReaderResourceInput;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_text" => Deserialize::begin(&mut self.custom_text),
                "email" => Deserialize::begin(&mut self.email),
                "numeric" => Deserialize::begin(&mut self.numeric),
                "phone" => Deserialize::begin(&mut self.phone),
                "required" => Deserialize::begin(&mut self.required),
                "selection" => Deserialize::begin(&mut self.selection),
                "signature" => Deserialize::begin(&mut self.signature),
                "skipped" => Deserialize::begin(&mut self.skipped),
                "text" => Deserialize::begin(&mut self.text),
                "toggles" => Deserialize::begin(&mut self.toggles),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                custom_text: Some(None),
                email: Some(None),
                numeric: Some(None),
                phone: Some(None),
                required: Some(None),
                selection: Some(None),
                signature: Some(None),
                skipped: Some(None),
                text: Some(None),
                toggles: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(custom_text),
                Some(email),
                Some(numeric),
                Some(phone),
                Some(required),
                Some(selection),
                Some(signature),
                Some(skipped),
                Some(text),
                Some(toggles),
                Some(type_),
            ) = (
                self.custom_text.take(),
                self.email.take(),
                self.numeric.take(),
                self.phone.take(),
                self.required,
                self.selection.take(),
                self.signature.take(),
                self.skipped,
                self.text.take(),
                self.toggles.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                custom_text,
                email,
                numeric,
                phone,
                required,
                selection,
                signature,
                skipped,
                text,
                toggles,
                type_,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TerminalReaderReaderResourceInput {
        type Builder = TerminalReaderReaderResourceInputBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceInput {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceInputBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "custom_text" => b.custom_text = FromValueOpt::from_value(v),
                    "email" => b.email = FromValueOpt::from_value(v),
                    "numeric" => b.numeric = FromValueOpt::from_value(v),
                    "phone" => b.phone = FromValueOpt::from_value(v),
                    "required" => b.required = FromValueOpt::from_value(v),
                    "selection" => b.selection = FromValueOpt::from_value(v),
                    "signature" => b.signature = FromValueOpt::from_value(v),
                    "skipped" => b.skipped = FromValueOpt::from_value(v),
                    "text" => b.text = FromValueOpt::from_value(v),
                    "toggles" => b.toggles = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of input being collected.
pub type TerminalReaderReaderResourceInputType =
    stripe_shared::CollectInputsTerminalReaderInputsType;
