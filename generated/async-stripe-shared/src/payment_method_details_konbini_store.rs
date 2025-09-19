#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKonbiniStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<PaymentMethodDetailsKonbiniStoreChain>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsKonbiniStoreBuilder {
    chain: Option<Option<PaymentMethodDetailsKonbiniStoreChain>>,
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

    impl Deserialize for PaymentMethodDetailsKonbiniStore {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKonbiniStore>,
        builder: PaymentMethodDetailsKonbiniStoreBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKonbiniStore> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsKonbiniStoreBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsKonbiniStoreBuilder {
        type Out = PaymentMethodDetailsKonbiniStore;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chain" => Deserialize::begin(&mut self.chain),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { chain: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(chain),) = (self.chain,) else {
                return None;
            };
            Some(Self::Out { chain })
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

    impl ObjectDeser for PaymentMethodDetailsKonbiniStore {
        type Builder = PaymentMethodDetailsKonbiniStoreBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsKonbiniStore {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsKonbiniStoreBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "chain" => b.chain = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The name of the convenience store chain where the payment was completed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsKonbiniStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}
impl PaymentMethodDetailsKonbiniStoreChain {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsKonbiniStoreChain::*;
        match self {
            Familymart => "familymart",
            Lawson => "lawson",
            Ministop => "ministop",
            Seicomart => "seicomart",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsKonbiniStoreChain {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsKonbiniStoreChain::*;
        match s {
            "familymart" => Ok(Familymart),
            "lawson" => Ok(Lawson),
            "ministop" => Ok(Ministop),
            "seicomart" => Ok(Seicomart),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsKonbiniStoreChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodDetailsKonbiniStoreChain {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsKonbiniStoreChain> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodDetailsKonbiniStoreChain::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsKonbiniStoreChain);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsKonbiniStoreChain {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsKonbiniStoreChain")
        })
    }
}
