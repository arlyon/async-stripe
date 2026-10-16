#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountPaypaySiteSettings {
    pub accessible: Option<stripe_shared::AccountPaypaySiteAccessibleSettings>,
    pub in_development: Option<stripe_shared::AccountPaypaySiteInDevelopmentSettings>,
    pub restricted: Option<stripe_shared::AccountPaypaySiteRestrictedSettings>,
    /// The status of your business's website.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<AccountPaypaySiteSettingsType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPaypaySiteSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountPaypaySiteSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountPaypaySiteSettingsBuilder {
    accessible: Option<Option<stripe_shared::AccountPaypaySiteAccessibleSettings>>,
    in_development: Option<Option<stripe_shared::AccountPaypaySiteInDevelopmentSettings>>,
    restricted: Option<Option<stripe_shared::AccountPaypaySiteRestrictedSettings>>,
    type_: Option<Option<AccountPaypaySiteSettingsType>>,
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

    impl Deserialize for AccountPaypaySiteSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountPaypaySiteSettings>,
        builder: AccountPaypaySiteSettingsBuilder,
    }

    impl Visitor for Place<AccountPaypaySiteSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountPaypaySiteSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountPaypaySiteSettingsBuilder {
        type Out = AccountPaypaySiteSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "accessible" => Deserialize::begin(&mut self.accessible),
                "in_development" => Deserialize::begin(&mut self.in_development),
                "restricted" => Deserialize::begin(&mut self.restricted),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                accessible: Some(None),
                in_development: Some(None),
                restricted: Some(None),
                type_: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(accessible), Some(in_development), Some(restricted), Some(type_)) = (
                self.accessible,
                self.in_development.take(),
                self.restricted.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { accessible, in_development, restricted, type_ })
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

    impl ObjectDeser for AccountPaypaySiteSettings {
        type Builder = AccountPaypaySiteSettingsBuilder;
    }

    impl FromValueOpt for AccountPaypaySiteSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountPaypaySiteSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "accessible" => b.accessible = FromValueOpt::from_value(v),
                    "in_development" => b.in_development = FromValueOpt::from_value(v),
                    "restricted" => b.restricted = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The status of your business's website.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountPaypaySiteSettingsType {
    Accessible,
    InDevelopment,
    Restricted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountPaypaySiteSettingsType {
    pub fn as_str(&self) -> &str {
        use AccountPaypaySiteSettingsType::*;
        match self {
            Accessible => "accessible",
            InDevelopment => "in_development",
            Restricted => "restricted",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountPaypaySiteSettingsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountPaypaySiteSettingsType::*;
        match s {
            "accessible" => Ok(Accessible),
            "in_development" => Ok(InDevelopment),
            "restricted" => Ok(Restricted),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountPaypaySiteSettingsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountPaypaySiteSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AccountPaypaySiteSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountPaypaySiteSettingsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AccountPaypaySiteSettingsType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountPaypaySiteSettingsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountPaypaySiteSettingsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountPaypaySiteSettingsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AccountPaypaySiteSettingsType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountPaypaySiteSettingsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountPaypaySiteSettingsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
