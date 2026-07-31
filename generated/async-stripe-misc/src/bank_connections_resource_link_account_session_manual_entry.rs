#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceLinkAccountSessionManualEntry {
    /// Controls how manual entry of bank account details is presented to the user.
    pub mode: Option<BankConnectionsResourceLinkAccountSessionManualEntryMode>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionManualEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceLinkAccountSessionManualEntry")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceLinkAccountSessionManualEntryBuilder {
    mode: Option<Option<BankConnectionsResourceLinkAccountSessionManualEntryMode>>,
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

    impl Deserialize for BankConnectionsResourceLinkAccountSessionManualEntry {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceLinkAccountSessionManualEntry>,
        builder: BankConnectionsResourceLinkAccountSessionManualEntryBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceLinkAccountSessionManualEntry> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceLinkAccountSessionManualEntryBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceLinkAccountSessionManualEntryBuilder {
        type Out = BankConnectionsResourceLinkAccountSessionManualEntry;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mode" => Deserialize::begin(&mut self.mode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { mode: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(mode),) = (self.mode.take(),) else {
                return None;
            };
            Some(Self::Out { mode })
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

    impl ObjectDeser for BankConnectionsResourceLinkAccountSessionManualEntry {
        type Builder = BankConnectionsResourceLinkAccountSessionManualEntryBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceLinkAccountSessionManualEntry {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BankConnectionsResourceLinkAccountSessionManualEntryBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "mode" => b.mode = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls how manual entry of bank account details is presented to the user.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceLinkAccountSessionManualEntryMode {
    Automatic,
    Disabled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceLinkAccountSessionManualEntryMode {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceLinkAccountSessionManualEntryMode::*;
        match self {
            Automatic => "automatic",
            Disabled => "disabled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceLinkAccountSessionManualEntryMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceLinkAccountSessionManualEntryMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "disabled" => Ok(Disabled),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceLinkAccountSessionManualEntryMode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceLinkAccountSessionManualEntryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionManualEntryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceLinkAccountSessionManualEntryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BankConnectionsResourceLinkAccountSessionManualEntryMode))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceLinkAccountSessionManualEntryMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for BankConnectionsResourceLinkAccountSessionManualEntryMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<BankConnectionsResourceLinkAccountSessionManualEntryMode>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceLinkAccountSessionManualEntryMode::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    BankConnectionsResourceLinkAccountSessionManualEntryMode
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceLinkAccountSessionManualEntryMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
