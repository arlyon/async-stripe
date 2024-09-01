#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxIDsOwner {
    /// The account being referenced when `type` is `account`.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The Connect Application being referenced when `type` is `application`.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The customer being referenced when `type` is `customer`.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Type of owner referenced.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxIDsOwnerType,
}
#[doc(hidden)]
pub struct TaxIDsOwnerBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    type_: Option<TaxIDsOwnerType>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for TaxIDsOwner {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxIDsOwner>,
        builder: TaxIDsOwnerBuilder,
    }

    impl Visitor for Place<TaxIDsOwner> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxIDsOwnerBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxIDsOwnerBuilder {
        type Out = TaxIDsOwner;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "application" => Deserialize::begin(&mut self.application),
                "customer" => Deserialize::begin(&mut self.customer),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                application: Deserialize::default(),
                customer: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account), Some(application), Some(customer), Some(type_)) =
                (self.account.take(), self.application.take(), self.customer.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { account, application, customer, type_ })
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

    impl ObjectDeser for TaxIDsOwner {
        type Builder = TaxIDsOwnerBuilder;
    }

    impl FromValueOpt for TaxIDsOwner {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxIDsOwnerBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "application" => b.application = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of owner referenced.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxIDsOwnerType {
    Account,
    Application,
    Customer,
    Self_,
}
impl TaxIDsOwnerType {
    pub fn as_str(self) -> &'static str {
        use TaxIDsOwnerType::*;
        match self {
            Account => "account",
            Application => "application",
            Customer => "customer",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for TaxIDsOwnerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxIDsOwnerType::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            "customer" => Ok(Customer),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxIDsOwnerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxIDsOwnerType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxIDsOwnerType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxIDsOwnerType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxIDsOwnerType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxIDsOwnerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxIDsOwnerType"))
    }
}
