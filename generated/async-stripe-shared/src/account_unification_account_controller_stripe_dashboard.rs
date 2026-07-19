#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountUnificationAccountControllerStripeDashboard {
    /// A value indicating the Stripe dashboard this account has access to independent of the Connect application.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: AccountUnificationAccountControllerStripeDashboardType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountUnificationAccountControllerStripeDashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountUnificationAccountControllerStripeDashboard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountUnificationAccountControllerStripeDashboardBuilder {
    type_: Option<AccountUnificationAccountControllerStripeDashboardType>,
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

    impl Deserialize for AccountUnificationAccountControllerStripeDashboard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountUnificationAccountControllerStripeDashboard>,
        builder: AccountUnificationAccountControllerStripeDashboardBuilder,
    }

    impl Visitor for Place<AccountUnificationAccountControllerStripeDashboard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountUnificationAccountControllerStripeDashboardBuilder {
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(type_),) = (self.builder.type_.take(),) else {
                return Ok(());
            };
            *self.out = Some(AccountUnificationAccountControllerStripeDashboard { type_ });
            Ok(())
        }
    }
};
/// A value indicating the Stripe dashboard this account has access to independent of the Connect application.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountUnificationAccountControllerStripeDashboardType {
    Express,
    Full,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountUnificationAccountControllerStripeDashboardType {
    pub fn as_str(&self) -> &str {
        use AccountUnificationAccountControllerStripeDashboardType::*;
        match self {
            Express => "express",
            Full => "full",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerStripeDashboardType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerStripeDashboardType::*;
        match s {
            "express" => Ok(Express),
            "full" => Ok(Full),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountUnificationAccountControllerStripeDashboardType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountUnificationAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountUnificationAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountUnificationAccountControllerStripeDashboardType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountUnificationAccountControllerStripeDashboardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for AccountUnificationAccountControllerStripeDashboardType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<AccountUnificationAccountControllerStripeDashboardType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountUnificationAccountControllerStripeDashboardType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerStripeDashboardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
