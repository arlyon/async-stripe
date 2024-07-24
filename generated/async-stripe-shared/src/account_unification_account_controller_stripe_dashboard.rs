#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountUnificationAccountControllerStripeDashboard {
    /// A value indicating the Stripe dashboard this account has access to independent of the Connect application.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: AccountUnificationAccountControllerStripeDashboardType,
}
#[doc(hidden)]
pub struct AccountUnificationAccountControllerStripeDashboardBuilder {
    type_: Option<AccountUnificationAccountControllerStripeDashboardType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: AccountUnificationAccountControllerStripeDashboardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountUnificationAccountControllerStripeDashboardBuilder {
        type Out = AccountUnificationAccountControllerStripeDashboard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { type_: self.type_? })
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

    impl ObjectDeser for AccountUnificationAccountControllerStripeDashboard {
        type Builder = AccountUnificationAccountControllerStripeDashboardBuilder;
    }

    impl FromValueOpt for AccountUnificationAccountControllerStripeDashboard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountUnificationAccountControllerStripeDashboardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A value indicating the Stripe dashboard this account has access to independent of the Connect application.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountUnificationAccountControllerStripeDashboardType {
    Express,
    Full,
    None,
}
impl AccountUnificationAccountControllerStripeDashboardType {
    pub fn as_str(self) -> &'static str {
        use AccountUnificationAccountControllerStripeDashboardType::*;
        match self {
            Express => "express",
            Full => "full",
            None => "none",
        }
    }
}

impl std::str::FromStr for AccountUnificationAccountControllerStripeDashboardType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountUnificationAccountControllerStripeDashboardType::*;
        match s {
            "express" => Ok(Express),
            "full" => Ok(Full),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for AccountUnificationAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountUnificationAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for AccountUnificationAccountControllerStripeDashboardType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<AccountUnificationAccountControllerStripeDashboardType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            AccountUnificationAccountControllerStripeDashboardType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AccountUnificationAccountControllerStripeDashboardType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AccountUnificationAccountControllerStripeDashboardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AccountUnificationAccountControllerStripeDashboardType",
            )
        })
    }
}
