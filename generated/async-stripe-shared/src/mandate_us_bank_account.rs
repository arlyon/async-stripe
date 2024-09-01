#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandateUsBankAccount {
    /// Mandate collection method
    pub collection_method: Option<MandateUsBankAccountCollectionMethod>,
}
#[doc(hidden)]
pub struct MandateUsBankAccountBuilder {
    collection_method: Option<Option<MandateUsBankAccountCollectionMethod>>,
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

    impl Deserialize for MandateUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateUsBankAccount>,
        builder: MandateUsBankAccountBuilder,
    }

    impl Visitor for Place<MandateUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandateUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandateUsBankAccountBuilder {
        type Out = MandateUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "collection_method" => Deserialize::begin(&mut self.collection_method),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { collection_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(collection_method),) = (self.collection_method,) else {
                return None;
            };
            Some(Self::Out { collection_method })
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

    impl ObjectDeser for MandateUsBankAccount {
        type Builder = MandateUsBankAccountBuilder;
    }

    impl FromValueOpt for MandateUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandateUsBankAccountBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "collection_method" => b.collection_method = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Mandate collection method
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateUsBankAccountCollectionMethod {
    Paper,
}
impl MandateUsBankAccountCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use MandateUsBankAccountCollectionMethod::*;
        match self {
            Paper => "paper",
        }
    }
}

impl std::str::FromStr for MandateUsBankAccountCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateUsBankAccountCollectionMethod::*;
        match s {
            "paper" => Ok(Paper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for MandateUsBankAccountCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateUsBankAccountCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateUsBankAccountCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandateUsBankAccountCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateUsBankAccountCollectionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(MandateUsBankAccountCollectionMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateUsBankAccountCollectionMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateUsBankAccountCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for MandateUsBankAccountCollectionMethod")
        })
    }
}
