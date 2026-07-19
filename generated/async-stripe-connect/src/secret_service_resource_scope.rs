#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SecretServiceResourceScope {
    /// The secret scope type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SecretServiceResourceScopeType,
    /// The user ID, if type is set to "user"
    pub user: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SecretServiceResourceScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SecretServiceResourceScope").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SecretServiceResourceScopeBuilder {
    type_: Option<SecretServiceResourceScopeType>,
    user: Option<Option<String>>,
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
                builder: SecretServiceResourceScopeBuilder {
                    type_: Deserialize::default(),
                    user: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.builder.type_),
                "user" => Deserialize::begin(&mut self.builder.user),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(type_), Some(user)) = (self.builder.type_.take(), self.builder.user.take())
            else {
                return Ok(());
            };
            *self.out = Some(SecretServiceResourceScope { type_, user });
            Ok(())
        }
    }
};
/// The secret scope type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SecretServiceResourceScopeType {
    pub fn as_str(&self) -> &str {
        use SecretServiceResourceScopeType::*;
        match self {
            Account => "account",
            User => "user",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SecretServiceResourceScopeType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SecretServiceResourceScopeType::*;
        match s {
            "account" => Ok(Account),
            "user" => Ok(User),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SecretServiceResourceScopeType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SecretServiceResourceScopeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SecretServiceResourceScopeType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for SecretServiceResourceScopeType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SecretServiceResourceScopeType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SecretServiceResourceScopeType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SecretServiceResourceScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
