#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceScope {
    /// The price type that credit grants can apply to.
    /// We currently only support the `metered` price type.
    /// This refers to prices that have a [Billing Meter](https://docs.stripe.com/api/billing/meter) attached to them.
    /// Cannot be used in combination with `prices`.
    pub price_type: Option<stripe_shared::BillingCreditGrantsResourceScopePriceType>,
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
    price_type: Option<Option<stripe_shared::BillingCreditGrantsResourceScopePriceType>>,
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
    use miniserde::{Deserialize, Result, make_place};
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
            Self { price_type: Some(None), prices: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(price_type), Some(prices)) = (self.price_type.take(), self.prices.take())
            else {
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
pub type BillingCreditGrantsResourceScopePriceType =
    stripe_shared::BillingCreditGrantsResourceScopePriceType;
