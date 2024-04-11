#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountUnificationAccountController {
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    /// Otherwise, this field is null.
    pub is_controller: Option<bool>,
    /// The controller type.
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: AccountUnificationAccountControllerType,
}
#[doc(hidden)]
pub struct AccountUnificationAccountControllerBuilder {
    is_controller: Option<Option<bool>>,
    type_: Option<AccountUnificationAccountControllerType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountUnificationAccountController {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountUnificationAccountController>,
        builder: AccountUnificationAccountControllerBuilder,
    }

    impl Visitor for Place<AccountUnificationAccountController> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountUnificationAccountControllerBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountUnificationAccountControllerBuilder {
        type Out = AccountUnificationAccountController;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "is_controller" => Deserialize::begin(&mut self.is_controller),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { is_controller: Deserialize::default(), type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { is_controller: self.is_controller?, type_: self.type_? })
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

    impl ObjectDeser for AccountUnificationAccountController {
        type Builder = AccountUnificationAccountControllerBuilder;
    }

    impl FromValueOpt for AccountUnificationAccountController {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountUnificationAccountControllerBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "is_controller" => b.is_controller = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The controller type.
/// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
}
impl AccountUnificationAccountControllerType {
    pub fn as_str(self) -> &'static str {
        use AccountUnificationAccountControllerType::*;
        match self {
            Account => "account",
            Application => "application",
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerType::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountUnificationAccountControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountUnificationAccountControllerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountUnificationAccountControllerType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AccountUnificationAccountControllerType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountUnificationAccountControllerType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountUnificationAccountControllerType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountUnificationAccountControllerType")
        })
    }
}
