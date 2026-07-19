#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceAccountholder {
    /// The ID of the Stripe account that this account belongs to.
    /// Only available when `account_holder.type` is `account`.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The ID for an Account representing a customer that this account belongs to.
    /// Only available when `account_holder.type` is `customer`.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    pub customer_account: Option<String>,
    /// Type of account holder that this account belongs to.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BankConnectionsResourceAccountholderType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountholder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BankConnectionsResourceAccountholder").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BankConnectionsResourceAccountholderBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_account: Option<Option<String>>,
    type_: Option<BankConnectionsResourceAccountholderType>,
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

    impl Deserialize for BankConnectionsResourceAccountholder {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BankConnectionsResourceAccountholder>,
        builder: BankConnectionsResourceAccountholderBuilder,
    }

    impl Visitor for Place<BankConnectionsResourceAccountholder> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BankConnectionsResourceAccountholderBuilder {
                    account: Deserialize::default(),
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
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(account), Some(customer), Some(customer_account), Some(type_)) = (
                self.builder.account.take(),
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(BankConnectionsResourceAccountholder {
                account,
                customer,
                customer_account,
                type_,
            });
            Ok(())
        }
    }
};
/// Type of account holder that this account belongs to.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BankConnectionsResourceAccountholderType {
    Account,
    Customer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BankConnectionsResourceAccountholderType {
    pub fn as_str(&self) -> &str {
        use BankConnectionsResourceAccountholderType::*;
        match self {
            Account => "account",
            Customer => "customer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceAccountholderType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountholderType::*;
        match s {
            "account" => Ok(Account),
            "customer" => Ok(Customer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BankConnectionsResourceAccountholderType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BankConnectionsResourceAccountholderType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BankConnectionsResourceAccountholderType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BankConnectionsResourceAccountholderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BankConnectionsResourceAccountholderType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(BankConnectionsResourceAccountholderType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceAccountholderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
