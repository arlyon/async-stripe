#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutLinkWalletOptions {
    /// Describes whether Checkout should display Link. Defaults to `auto`.
    pub display: Option<CheckoutLinkWalletOptionsDisplay>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutLinkWalletOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutLinkWalletOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutLinkWalletOptionsBuilder {
    display: Option<Option<CheckoutLinkWalletOptionsDisplay>>,
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
                builder: CheckoutLinkWalletOptionsBuilder { display: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "display" => Deserialize::begin(&mut self.builder.display),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(display),) = (self.builder.display.take(),) else {
                return Ok(());
            };
            *self.out = Some(CheckoutLinkWalletOptions { display });
            Ok(())
        }
    }
};
/// Describes whether Checkout should display Link. Defaults to `auto`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutLinkWalletOptionsDisplay {
    Auto,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutLinkWalletOptionsDisplay {
    pub fn as_str(&self) -> &str {
        use CheckoutLinkWalletOptionsDisplay::*;
        match self {
            Auto => "auto",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutLinkWalletOptionsDisplay {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutLinkWalletOptionsDisplay::*;
        match s {
            "auto" => Ok(Auto),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutLinkWalletOptionsDisplay"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutLinkWalletOptionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CheckoutLinkWalletOptionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutLinkWalletOptionsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CheckoutLinkWalletOptionsDisplay)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for CheckoutLinkWalletOptionsDisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CheckoutLinkWalletOptionsDisplay> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutLinkWalletOptionsDisplay::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutLinkWalletOptionsDisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
