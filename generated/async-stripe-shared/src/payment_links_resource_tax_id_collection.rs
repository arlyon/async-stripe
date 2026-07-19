#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceTaxIdCollection {
    /// Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
    pub required: PaymentLinksResourceTaxIdCollectionRequired,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceTaxIdCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceTaxIdCollection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceTaxIdCollectionBuilder {
    enabled: Option<bool>,
    required: Option<PaymentLinksResourceTaxIdCollectionRequired>,
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

    impl Deserialize for PaymentLinksResourceTaxIdCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceTaxIdCollection>,
        builder: PaymentLinksResourceTaxIdCollectionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceTaxIdCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceTaxIdCollectionBuilder {
                    enabled: Deserialize::default(),
                    required: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "required" => Deserialize::begin(&mut self.builder.required),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled), Some(required)) =
                (self.builder.enabled, self.builder.required.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceTaxIdCollection { enabled, required });
            Ok(())
        }
    }
};
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinksResourceTaxIdCollectionRequired {
    IfSupported,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinksResourceTaxIdCollectionRequired {
    pub fn as_str(&self) -> &str {
        use PaymentLinksResourceTaxIdCollectionRequired::*;
        match self {
            IfSupported => "if_supported",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinksResourceTaxIdCollectionRequired {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourceTaxIdCollectionRequired::*;
        match s {
            "if_supported" => Ok(IfSupported),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinksResourceTaxIdCollectionRequired"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentLinksResourceTaxIdCollectionRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentLinksResourceTaxIdCollectionRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceTaxIdCollectionRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentLinksResourceTaxIdCollectionRequired))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourceTaxIdCollectionRequired {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentLinksResourceTaxIdCollectionRequired {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentLinksResourceTaxIdCollectionRequired> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentLinksResourceTaxIdCollectionRequired::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourceTaxIdCollectionRequired {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
