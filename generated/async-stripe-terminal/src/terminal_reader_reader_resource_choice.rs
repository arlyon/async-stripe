/// Choice to be selected on a Reader
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceChoice {
    /// The identifier for the selected choice. Maximum 50 characters.
    pub id: Option<stripe_terminal::TerminalReaderReaderResourceChoiceId>,
    /// The button style for the choice. Can be `primary` or `secondary`.
    pub style: Option<TerminalReaderReaderResourceChoiceStyle>,
    /// The text to be selected. Maximum 30 characters.
    pub text: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceChoice").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceChoiceBuilder {
    id: Option<Option<stripe_terminal::TerminalReaderReaderResourceChoiceId>>,
    style: Option<Option<TerminalReaderReaderResourceChoiceStyle>>,
    text: Option<String>,
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

    impl Deserialize for TerminalReaderReaderResourceChoice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceChoice>,
        builder: TerminalReaderReaderResourceChoiceBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceChoice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceChoiceBuilder {
                    id: Deserialize::default(),
                    style: Deserialize::default(),
                    text: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                "style" => Deserialize::begin(&mut self.builder.style),
                "text" => Deserialize::begin(&mut self.builder.text),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(id), Some(style), Some(text)) =
                (self.builder.id.take(), self.builder.style.take(), self.builder.text.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceChoice { id, style, text });
            Ok(())
        }
    }
};
/// The button style for the choice. Can be `primary` or `secondary`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderReaderResourceChoiceStyle {
    Primary,
    Secondary,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderReaderResourceChoiceStyle {
    pub fn as_str(&self) -> &str {
        use TerminalReaderReaderResourceChoiceStyle::*;
        match self {
            Primary => "primary",
            Secondary => "secondary",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceChoiceStyle {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceChoiceStyle::*;
        match s {
            "primary" => Ok(Primary),
            "secondary" => Ok(Secondary),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalReaderReaderResourceChoiceStyle"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceChoiceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderReaderResourceChoiceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceChoiceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderReaderResourceChoiceStyle)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalReaderReaderResourceChoiceStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TerminalReaderReaderResourceChoiceStyle {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceChoiceStyle> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TerminalReaderReaderResourceChoiceStyle::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceChoiceStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for TerminalReaderReaderResourceChoice {
    type Id = Option<stripe_terminal::TerminalReaderReaderResourceChoiceId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TerminalReaderReaderResourceChoiceId);
