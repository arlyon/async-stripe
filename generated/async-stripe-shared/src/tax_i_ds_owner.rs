#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxIDsOwner {
    /// The account being referenced when `type` is `account`.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The Connect Application being referenced when `type` is `application`.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The customer being referenced when `type` is `customer`.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// The Account representing the customer being referenced when `type` is `customer`.
    pub customer_account: Option<String>,
    /// Type of owner referenced.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxIDsOwnerType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxIDsOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxIDsOwner").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxIDsOwnerBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_account: Option<Option<String>>,
    type_: Option<TaxIDsOwnerType>,
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
                builder: TaxIDsOwnerBuilder {
                    account: Deserialize::default(),
                    application: Deserialize::default(),
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.builder.account),
                "application" => Deserialize::begin(&mut self.builder.application),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(account),
                Some(application),
                Some(customer),
                Some(customer_account),
                Some(type_),
            ) = (
                self.builder.account.take(),
                self.builder.application.take(),
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out =
                Some(TaxIDsOwner { account, application, customer, customer_account, type_ });
            Ok(())
        }
    }
};
/// Type of owner referenced.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxIDsOwnerType {
    Account,
    Application,
    Customer,
    Self_,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxIDsOwnerType {
    pub fn as_str(&self) -> &str {
        use TaxIDsOwnerType::*;
        match self {
            Account => "account",
            Application => "application",
            Customer => "customer",
            Self_ => "self",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxIDsOwnerType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxIDsOwnerType::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            "customer" => Ok(Customer),
            "self" => Ok(Self_),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TaxIDsOwnerType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxIDsOwnerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxIDsOwnerType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TaxIDsOwnerType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TaxIDsOwnerType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxIDsOwnerType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxIDsOwnerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
