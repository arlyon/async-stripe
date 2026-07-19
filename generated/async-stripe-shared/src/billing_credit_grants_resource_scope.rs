#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceScope {
    /// The price type that credit grants can apply to.
    /// We currently only support the `metered` price type.
    /// This refers to prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
    /// Cannot be used in combination with `prices`.
    pub price_type: Option<BillingCreditGrantsResourceScopePriceType>,
    /// The prices that credit grants can apply to.
    /// We currently only support `metered` prices.
    /// This refers to prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
    /// Cannot be used in combination with `price_type`.
    pub prices: Option<Vec<stripe_shared::BillingCreditGrantsResourceApplicablePrice>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingCreditGrantsResourceScope").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceScopeBuilder {
    price_type: Option<Option<BillingCreditGrantsResourceScopePriceType>>,
    prices: Option<Option<Vec<stripe_shared::BillingCreditGrantsResourceApplicablePrice>>>,
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

    impl Deserialize for BillingCreditGrantsResourceScope {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceScope>,
        builder: BillingCreditGrantsResourceScopeBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceScope> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceScopeBuilder {
                    price_type: Deserialize::default(),
                    prices: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "price_type" => Deserialize::begin(&mut self.builder.price_type),
                "prices" => Deserialize::begin(&mut self.builder.prices),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(price_type), Some(prices)) =
                (self.builder.price_type.take(), self.builder.prices.take())
            else {
                return Ok(());
            };
            *self.out = Some(BillingCreditGrantsResourceScope { price_type, prices });
            Ok(())
        }
    }
};
/// The price type that credit grants can apply to.
/// We currently only support the `metered` price type.
/// This refers to prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
/// Cannot be used in combination with `prices`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum BillingCreditGrantsResourceScopePriceType {
    Metered,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl BillingCreditGrantsResourceScopePriceType {
    pub fn as_str(&self) -> &str {
        use BillingCreditGrantsResourceScopePriceType::*;
        match self {
            Metered => "metered",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for BillingCreditGrantsResourceScopePriceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantsResourceScopePriceType::*;
        match s {
            "metered" => Ok(Metered),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "BillingCreditGrantsResourceScopePriceType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for BillingCreditGrantsResourceScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for BillingCreditGrantsResourceScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(BillingCreditGrantsResourceScopePriceType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BillingCreditGrantsResourceScopePriceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for BillingCreditGrantsResourceScopePriceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<BillingCreditGrantsResourceScopePriceType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(BillingCreditGrantsResourceScopePriceType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantsResourceScopePriceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
