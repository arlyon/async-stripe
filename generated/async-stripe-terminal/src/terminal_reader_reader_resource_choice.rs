/// Choice to be selected on a Reader
#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct TerminalReaderReaderResourceChoiceBuilder {
    id: Option<Option<stripe_terminal::TerminalReaderReaderResourceChoiceId>>,
    style: Option<Option<TerminalReaderReaderResourceChoiceStyle>>,
    text: Option<String>,
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
                builder: TerminalReaderReaderResourceChoiceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceChoiceBuilder {
        type Out = TerminalReaderReaderResourceChoice;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "style" => Deserialize::begin(&mut self.style),
                "text" => Deserialize::begin(&mut self.text),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                id: Deserialize::default(),
                style: Deserialize::default(),
                text: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(id), Some(style), Some(text)) =
                (self.id.take(), self.style, self.text.take())
            else {
                return None;
            };
            Some(Self::Out { id, style, text })
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

    impl ObjectDeser for TerminalReaderReaderResourceChoice {
        type Builder = TerminalReaderReaderResourceChoiceBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceChoice {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceChoiceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "style" => b.style = FromValueOpt::from_value(v),
                    "text" => b.text = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The button style for the choice. Can be `primary` or `secondary`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceChoiceStyle {
    Primary,
    Secondary,
}
impl TerminalReaderReaderResourceChoiceStyle {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceChoiceStyle::*;
        match self {
            Primary => "primary",
            Secondary => "secondary",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceChoiceStyle {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceChoiceStyle::*;
        match s {
            "primary" => Ok(Primary),
            "secondary" => Ok(Secondary),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceChoiceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceChoiceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TerminalReaderReaderResourceChoiceStyle {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceChoiceStyle> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalReaderReaderResourceChoiceStyle::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderReaderResourceChoiceStyle);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceChoiceStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for TerminalReaderReaderResourceChoiceStyle")
        })
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
