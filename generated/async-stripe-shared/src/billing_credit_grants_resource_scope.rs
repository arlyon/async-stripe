#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct BillingCreditGrantsResourceScopeBuilder {
    price_type: Option<Option<BillingCreditGrantsResourceScopePriceType>>,
    prices: Option<Option<Vec<stripe_shared::BillingCreditGrantsResourceApplicablePrice>>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: BillingCreditGrantsResourceScopeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantsResourceScopeBuilder {
        type Out = BillingCreditGrantsResourceScope;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "price_type" => Deserialize::begin(&mut self.price_type),
                "prices" => Deserialize::begin(&mut self.prices),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { price_type: Deserialize::default(), prices: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(price_type), Some(prices)) = (self.price_type, self.prices.take()) else {
                return None;
            };
            Some(Self::Out { price_type, prices })
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

    impl ObjectDeser for BillingCreditGrantsResourceScope {
        type Builder = BillingCreditGrantsResourceScopeBuilder;
    }

    impl FromValueOpt for BillingCreditGrantsResourceScope {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantsResourceScopeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "price_type" => b.price_type = FromValueOpt::from_value(v),
                    "prices" => b.prices = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The price type that credit grants can apply to.
/// We currently only support the `metered` price type.
/// This refers to prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
/// Cannot be used in combination with `prices`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BillingCreditGrantsResourceScopePriceType {
    Metered,
}
impl BillingCreditGrantsResourceScopePriceType {
    pub fn as_str(self) -> &'static str {
        use BillingCreditGrantsResourceScopePriceType::*;
        match self {
            Metered => "metered",
        }
    }
}

impl std::str::FromStr for BillingCreditGrantsResourceScopePriceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BillingCreditGrantsResourceScopePriceType::*;
        match s {
            "metered" => Ok(Metered),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for BillingCreditGrantsResourceScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for BillingCreditGrantsResourceScopePriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for BillingCreditGrantsResourceScopePriceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<BillingCreditGrantsResourceScopePriceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            BillingCreditGrantsResourceScopePriceType::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(BillingCreditGrantsResourceScopePriceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for BillingCreditGrantsResourceScopePriceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for BillingCreditGrantsResourceScopePriceType")
        })
    }
}
