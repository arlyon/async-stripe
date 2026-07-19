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
    type_: Option<TerminalReaderReaderResourceInputType>,
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
                builder: TerminalReaderReaderResourceInputBuilder {
                    custom_text: Deserialize::default(),
                    email: Deserialize::default(),
                    numeric: Deserialize::default(),
                    phone: Deserialize::default(),
                    required: Deserialize::default(),
                    selection: Deserialize::default(),
                    signature: Deserialize::default(),
                    skipped: Deserialize::default(),
                    text: Deserialize::default(),
                    toggles: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "custom_text" => Deserialize::begin(&mut self.builder.custom_text),
                "email" => Deserialize::begin(&mut self.builder.email),
                "numeric" => Deserialize::begin(&mut self.builder.numeric),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                "required" => Deserialize::begin(&mut self.builder.required),
                "selection" => Deserialize::begin(&mut self.builder.selection),
                "signature" => Deserialize::begin(&mut self.builder.signature),
                "skipped" => Deserialize::begin(&mut self.builder.skipped),
                "text" => Deserialize::begin(&mut self.builder.text),
                "toggles" => Deserialize::begin(&mut self.builder.toggles),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.custom_text.take(),
                self.builder.email.take(),
                self.builder.numeric.take(),
                self.builder.phone.take(),
                self.builder.required,
                self.builder.selection.take(),
                self.builder.signature.take(),
                self.builder.skipped,
                self.builder.text.take(),
                self.builder.toggles.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceInput {
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
            });
            Ok(())
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
impl stripe_miniserde::Deserialize for TerminalReaderReaderResourceInputType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceInputType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderReaderResourceInputType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceInputType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
