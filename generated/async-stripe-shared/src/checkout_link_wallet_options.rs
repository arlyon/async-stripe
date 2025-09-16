#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutLinkWalletOptions {
    /// Describes whether Checkout should display Link. Defaults to `auto`.
    pub display: Option<CheckoutLinkWalletOptionsDisplay>,
}
#[doc(hidden)]
pub struct CheckoutLinkWalletOptionsBuilder {
    display: Option<Option<CheckoutLinkWalletOptionsDisplay>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutLinkWalletOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutLinkWalletOptions>,
        builder: CheckoutLinkWalletOptionsBuilder,
    }

    impl Visitor for Place<CheckoutLinkWalletOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutLinkWalletOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutLinkWalletOptionsBuilder {
        type Out = CheckoutLinkWalletOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display" => Deserialize::begin(&mut self.display),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { display: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(display),) = (self.display,) else {
                return None;
            };
            Some(Self::Out { display })
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

    impl ObjectDeser for CheckoutLinkWalletOptions {
        type Builder = CheckoutLinkWalletOptionsBuilder;
    }

    impl FromValueOpt for CheckoutLinkWalletOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutLinkWalletOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "display" => b.display = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Describes whether Checkout should display Link. Defaults to `auto`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutLinkWalletOptionsDisplay {
    Auto,
    Never,
}
impl CheckoutLinkWalletOptionsDisplay {
    pub fn as_str(self) -> &'static str {
        use CheckoutLinkWalletOptionsDisplay::*;
        match self {
            Auto => "auto",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CheckoutLinkWalletOptionsDisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutLinkWalletOptionsDisplay::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutLinkWalletOptionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutLinkWalletOptionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutLinkWalletOptionsDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutLinkWalletOptionsDisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutLinkWalletOptionsDisplay> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(CheckoutLinkWalletOptionsDisplay::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutLinkWalletOptionsDisplay);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutLinkWalletOptionsDisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CheckoutLinkWalletOptionsDisplay")
        })
    }
}
