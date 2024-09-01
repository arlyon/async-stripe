#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BankConnectionsResourceAccountholder {
    /// The ID of the Stripe account this account belongs to.
    /// Should only be present if `account_holder.type` is `account`.
    pub account: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// ID of the Stripe customer this account belongs to.
    /// Present if and only if `account_holder.type` is `customer`.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Type of account holder that this account belongs to.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: BankConnectionsResourceAccountholderType,
}
#[doc(hidden)]
pub struct BankConnectionsResourceAccountholderBuilder {
    account: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    type_: Option<BankConnectionsResourceAccountholderType>,
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
                builder: BankConnectionsResourceAccountholderBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BankConnectionsResourceAccountholderBuilder {
        type Out = BankConnectionsResourceAccountholder;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account" => Deserialize::begin(&mut self.account),
                "customer" => Deserialize::begin(&mut self.customer),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account: Deserialize::default(),
                customer: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(account), Some(customer), Some(type_)) =
                (self.account.take(), self.customer.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { account, customer, type_ })
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

    impl ObjectDeser for BankConnectionsResourceAccountholder {
        type Builder = BankConnectionsResourceAccountholderBuilder;
    }

    impl FromValueOpt for BankConnectionsResourceAccountholder {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BankConnectionsResourceAccountholderBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account" => b.account = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of account holder that this account belongs to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BankConnectionsResourceAccountholderType {
    Account,
    Customer,
}
impl BankConnectionsResourceAccountholderType {
    pub fn as_str(self) -> &'static str {
        use BankConnectionsResourceAccountholderType::*;
        match self {
            Account => "account",
            Customer => "customer",
        }
    }
}

impl std::str::FromStr for BankConnectionsResourceAccountholderType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BankConnectionsResourceAccountholderType::*;
        match s {
            "account" => Ok(Account),
            "customer" => Ok(Customer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BankConnectionsResourceAccountholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BankConnectionsResourceAccountholderType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BankConnectionsResourceAccountholderType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BankConnectionsResourceAccountholderType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BankConnectionsResourceAccountholderType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BankConnectionsResourceAccountholderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BankConnectionsResourceAccountholderType")
        })
    }
}
