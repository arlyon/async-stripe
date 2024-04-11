#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SecretServiceResourceScope {
    /// The secret scope type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SecretServiceResourceScopeType,
    /// The user ID, if type is set to "user"
    pub user: Option<String>,
}
#[doc(hidden)]
pub struct SecretServiceResourceScopeBuilder {
    type_: Option<SecretServiceResourceScopeType>,
    user: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SecretServiceResourceScope {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SecretServiceResourceScope>,
        builder: SecretServiceResourceScopeBuilder,
    }

    impl Visitor for Place<SecretServiceResourceScope> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SecretServiceResourceScopeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SecretServiceResourceScopeBuilder {
        type Out = SecretServiceResourceScope;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                "user" => Deserialize::begin(&mut self.user),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default(), user: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { type_: self.type_?, user: self.user.take()? })
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

    impl ObjectDeser for SecretServiceResourceScope {
        type Builder = SecretServiceResourceScopeBuilder;
    }

    impl FromValueOpt for SecretServiceResourceScope {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SecretServiceResourceScopeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "user" => b.user = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The secret scope type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
}
impl SecretServiceResourceScopeType {
    pub fn as_str(self) -> &'static str {
        use SecretServiceResourceScopeType::*;
        match self {
            Account => "account",
            User => "user",
        }
    }
}

impl std::str::FromStr for SecretServiceResourceScopeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SecretServiceResourceScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SecretServiceResourceScopeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SecretServiceResourceScopeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SecretServiceResourceScopeType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SecretServiceResourceScopeType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SecretServiceResourceScopeType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SecretServiceResourceScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SecretServiceResourceScopeType")
        })
    }
}
