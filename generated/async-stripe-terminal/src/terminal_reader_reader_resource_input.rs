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
    pub type_: TerminalReaderReaderResourceInputType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceInput").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceInputBuilder {
    custom_text: Option<stripe_terminal::TerminalReaderReaderResourceCustomText>,
    email: Option<stripe_terminal::TerminalReaderReaderResourceEmail>,
    numeric: Option<stripe_terminal::TerminalReaderReaderResourceNumeric>,
    phone: Option<stripe_terminal::TerminalReaderReaderResourcePhone>,
    required: Option<Option<bool>>,
    selection: Option<stripe_terminal::TerminalReaderReaderResourceSelection>,
    signature: Option<stripe_terminal::TerminalReaderReaderResourceSignature>,
    skipped: Option<Option<bool>>,
    text: Option<stripe_terminal::TerminalReaderReaderResourceText>,
    toggles: Option<Option<Vec<stripe_terminal::TerminalReaderReaderResourceToggle>>>,
    type_: Option<TerminalReaderReaderResourceInputType>,
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
                "custom_text" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.custom_text)
                }
                "email" => stripe_types::miniserde_helpers::begin_nullable(&mut self.email),
                "numeric" => stripe_types::miniserde_helpers::begin_nullable(&mut self.numeric),
                "phone" => stripe_types::miniserde_helpers::begin_nullable(&mut self.phone),
                "required" => Deserialize::begin(&mut self.required),
                "selection" => stripe_types::miniserde_helpers::begin_nullable(&mut self.selection),
                "signature" => stripe_types::miniserde_helpers::begin_nullable(&mut self.signature),
                "skipped" => Deserialize::begin(&mut self.skipped),
                "text" => stripe_types::miniserde_helpers::begin_nullable(&mut self.text),
                "toggles" => Deserialize::begin(&mut self.toggles),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                custom_text: None,
                email: None,
                numeric: None,
                phone: None,
                required: Some(None),
                selection: None,
                signature: None,
                skipped: Some(None),
                text: None,
                toggles: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                custom_text,
                email,
                numeric,
                phone,
                Some(required),
                selection,
                signature,
                Some(skipped),
                text,
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
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Type of input being collected.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderReaderResourceInputType {
    Email,
    Numeric,
    Phone,
    Selection,
    Signature,
    Text,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderReaderResourceInputType {
    pub fn as_str(&self) -> &str {
        use TerminalReaderReaderResourceInputType::*;
        match self {
            Email => "email",
            Numeric => "numeric",
            Phone => "phone",
            Selection => "selection",
            Signature => "signature",
            Text => "text",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceInputType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceInputType::*;
        match s {
            "email" => Ok(Email),
            "numeric" => Ok(Numeric),
            "phone" => Ok(Phone),
            "selection" => Ok(Selection),
            "signature" => Ok(Signature),
            "text" => Ok(Text),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalReaderReaderResourceInputType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderReaderResourceInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderReaderResourceInputType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalReaderReaderResourceInputType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TerminalReaderReaderResourceInputType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceInputType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderReaderResourceInputType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderReaderResourceInputType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceInputType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
