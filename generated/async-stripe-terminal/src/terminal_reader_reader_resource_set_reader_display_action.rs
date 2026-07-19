/// Represents a reader action to set the reader display
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {
    /// Cart object to be displayed by the reader, including line items, amounts, and currency.
    pub cart: Option<stripe_terminal::TerminalReaderReaderResourceCart>,
    /// Type of information to be displayed by the reader. Only `cart` is currently supported.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TerminalReaderReaderResourceSetReaderDisplayActionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceSetReaderDisplayAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceSetReaderDisplayAction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceSetReaderDisplayActionBuilder {
    cart: Option<Option<stripe_terminal::TerminalReaderReaderResourceCart>>,
    type_: Option<TerminalReaderReaderResourceSetReaderDisplayActionType>,
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

    impl Deserialize for TerminalReaderReaderResourceSetReaderDisplayAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceSetReaderDisplayAction>,
        builder: TerminalReaderReaderResourceSetReaderDisplayActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceSetReaderDisplayAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceSetReaderDisplayActionBuilder {
                    cart: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cart" => Deserialize::begin(&mut self.builder.cart),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(cart), Some(type_)) = (self.builder.cart.take(), self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceSetReaderDisplayAction { cart, type_ });
            Ok(())
        }
    }
};
/// Type of information to be displayed by the reader. Only `cart` is currently supported.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderReaderResourceSetReaderDisplayActionType {
    Cart,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderReaderResourceSetReaderDisplayActionType {
    pub fn as_str(&self) -> &str {
        use TerminalReaderReaderResourceSetReaderDisplayActionType::*;
        match self {
            Cart => "cart",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceSetReaderDisplayActionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceSetReaderDisplayActionType::*;
        match s {
            "cart" => Ok(Cart),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalReaderReaderResourceSetReaderDisplayActionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderReaderResourceSetReaderDisplayActionType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TerminalReaderReaderResourceSetReaderDisplayActionType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalReaderReaderResourceSetReaderDisplayActionType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
