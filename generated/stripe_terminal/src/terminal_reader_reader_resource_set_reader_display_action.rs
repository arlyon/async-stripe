/// Represents a reader action to set the reader display
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {
    /// Cart object to be displayed by the reader.
    pub cart: Option<stripe_terminal::TerminalReaderReaderResourceCart>,
    /// Type of information to be displayed by the reader.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TerminalReaderReaderResourceSetReaderDisplayActionType,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceSetReaderDisplayActionBuilder {
    cart: Option<Option<stripe_terminal::TerminalReaderReaderResourceCart>>,
    type_: Option<TerminalReaderReaderResourceSetReaderDisplayActionType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: TerminalReaderReaderResourceSetReaderDisplayActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceSetReaderDisplayActionBuilder {
        type Out = TerminalReaderReaderResourceSetReaderDisplayAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cart" => Deserialize::begin(&mut self.cart),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { cart: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { cart: self.cart.take()?, type_: self.type_? })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TerminalReaderReaderResourceSetReaderDisplayAction {
        type Builder = TerminalReaderReaderResourceSetReaderDisplayActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceSetReaderDisplayAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceSetReaderDisplayActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "cart" => b.cart = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of information to be displayed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceSetReaderDisplayActionType {
    Cart,
}
impl TerminalReaderReaderResourceSetReaderDisplayActionType {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceSetReaderDisplayActionType::*;
        match self {
            Cart => "cart",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceSetReaderDisplayActionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceSetReaderDisplayActionType::*;
        match s {
            "cart" => Ok(Cart),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TerminalReaderReaderResourceSetReaderDisplayActionType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalReaderReaderResourceSetReaderDisplayActionType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderReaderResourceSetReaderDisplayActionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalReaderReaderResourceSetReaderDisplayActionType",
            )
        })
    }
}
