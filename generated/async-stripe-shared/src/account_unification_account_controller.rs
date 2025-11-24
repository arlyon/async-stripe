#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountUnificationAccountController {
    pub fees: Option<stripe_shared::AccountUnificationAccountControllerFees>,
    /// `true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts).
    /// Otherwise, this field is null.
    pub is_controller: Option<bool>,
    pub losses: Option<stripe_shared::AccountUnificationAccountControllerLosses>,
    /// A value indicating responsibility for collecting requirements on this account.
    /// Only returned when the Connect application retrieving the resource controls the account.
    pub requirement_collection: Option<AccountUnificationAccountControllerRequirementCollection>,
    pub stripe_dashboard: Option<stripe_shared::AccountUnificationAccountControllerStripeDashboard>,
    /// The controller type.
    /// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: AccountUnificationAccountControllerType,
}
#[doc(hidden)]
pub struct AccountUnificationAccountControllerBuilder {
    fees: Option<Option<stripe_shared::AccountUnificationAccountControllerFees>>,
    is_controller: Option<Option<bool>>,
    losses: Option<Option<stripe_shared::AccountUnificationAccountControllerLosses>>,
    requirement_collection:
        Option<Option<AccountUnificationAccountControllerRequirementCollection>>,
    stripe_dashboard:
        Option<Option<stripe_shared::AccountUnificationAccountControllerStripeDashboard>>,
    type_: Option<AccountUnificationAccountControllerType>,
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
                "fees" => Deserialize::begin(&mut self.fees),
                "is_controller" => Deserialize::begin(&mut self.is_controller),
                "losses" => Deserialize::begin(&mut self.losses),
                "requirement_collection" => Deserialize::begin(&mut self.requirement_collection),
                "stripe_dashboard" => Deserialize::begin(&mut self.stripe_dashboard),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                fees: Deserialize::default(),
                is_controller: Deserialize::default(),
                losses: Deserialize::default(),
                requirement_collection: Deserialize::default(),
                stripe_dashboard: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(fees),
                Some(is_controller),
                Some(losses),
                Some(requirement_collection),
                Some(stripe_dashboard),
                Some(type_),
            ) = (
                self.fees.take(),
                self.is_controller,
                self.losses.take(),
                self.requirement_collection.take(),
                self.stripe_dashboard.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                fees,
                is_controller,
                losses,
                requirement_collection,
                stripe_dashboard,
                type_,
            })
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
                    "fees" => b.fees = FromValueOpt::from_value(v),
                    "is_controller" => b.is_controller = FromValueOpt::from_value(v),
                    "losses" => b.losses = FromValueOpt::from_value(v),
                    "requirement_collection" => {
                        b.requirement_collection = FromValueOpt::from_value(v)
                    }
                    "stripe_dashboard" => b.stripe_dashboard = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A value indicating responsibility for collecting requirements on this account.
/// Only returned when the Connect application retrieving the resource controls the account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountUnificationAccountControllerRequirementCollection {
    Application,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountUnificationAccountControllerRequirementCollection {
    pub fn as_str(&self) -> &str {
        use AccountUnificationAccountControllerRequirementCollection::*;
        match self {
            Application => "application",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerRequirementCollection {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerRequirementCollection::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountUnificationAccountControllerRequirementCollection"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountUnificationAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccountUnificationAccountControllerRequirementCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AccountUnificationAccountControllerRequirementCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<AccountUnificationAccountControllerRequirementCollection>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountUnificationAccountControllerRequirementCollection::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    AccountUnificationAccountControllerRequirementCollection
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerRequirementCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The controller type.
/// Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AccountUnificationAccountControllerType {
    Account,
    Application,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AccountUnificationAccountControllerType {
    pub fn as_str(&self) -> &str {
        use AccountUnificationAccountControllerType::*;
        match self {
            Account => "account",
            Application => "application",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerType::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "AccountUnificationAccountControllerType"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        self.out = Some(AccountUnificationAccountControllerType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountUnificationAccountControllerType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
