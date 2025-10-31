#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectAccountReference {
    /// The connected account being referenced when `type` is `account`.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Type of the account referenced.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ConnectAccountReferenceType,
}
#[doc(hidden)]
pub struct ConnectAccountReferenceBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    type_: Option<ConnectAccountReferenceType>,
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

    impl Deserialize for ConnectAccountReference {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectAccountReference>,
        builder: ConnectAccountReferenceBuilder,
    }

    impl Visitor for Place<ConnectAccountReference> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectAccountReferenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectAccountReferenceBuilder {
        type Out = ConnectAccountReference;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { account: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account), Some(type_)) = (self.account.take(), self.type_) else {
                return None;
            };
            Some(Self::Out { account, type_ })
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

    impl ObjectDeser for ConnectAccountReference {
        type Builder = ConnectAccountReferenceBuilder;
    }

    impl FromValueOpt for ConnectAccountReference {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectAccountReferenceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of the account referenced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConnectAccountReferenceType {
    Account,
    Self_,
}
impl ConnectAccountReferenceType {
    pub fn as_str(self) -> &'static str {
        use ConnectAccountReferenceType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for ConnectAccountReferenceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConnectAccountReferenceType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConnectAccountReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConnectAccountReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ConnectAccountReferenceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ConnectAccountReferenceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ConnectAccountReferenceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ConnectAccountReferenceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ConnectAccountReferenceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConnectAccountReferenceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ConnectAccountReferenceType"))
    }
}
