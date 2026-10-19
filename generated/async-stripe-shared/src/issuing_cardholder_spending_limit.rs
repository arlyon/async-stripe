#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderSpendingLimit {
    /// Maximum amount allowed to spend per interval.
    /// This amount is in the card's currency and in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal).
    pub amount: i64,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    /// Omitting this field will apply the limit to all categories.
    pub categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Interval (or event) to which the amount applies.
    pub interval: stripe_shared::IssuingCardSpendingLimitInterval,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderSpendingLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardholderSpendingLimit").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardholderSpendingLimitBuilder {
    amount: Option<i64>,
    categories: Option<Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>>,
    interval: Option<stripe_shared::IssuingCardSpendingLimitInterval>,
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

    impl Deserialize for IssuingCardholderSpendingLimit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderSpendingLimit>,
        builder: IssuingCardholderSpendingLimitBuilder,
    }

    impl Visitor for Place<IssuingCardholderSpendingLimit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderSpendingLimitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardholderSpendingLimitBuilder {
        type Out = IssuingCardholderSpendingLimit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "categories" => Deserialize::begin(&mut self.categories),
                "interval" => Deserialize::begin(&mut self.interval),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount: None, categories: Some(None), interval: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(categories), Some(interval)) =
                (self.amount, self.categories.take(), self.interval.take())
            else {
                return None;
            };
            Some(Self::Out { amount, categories, interval })
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

    impl ObjectDeser for IssuingCardholderSpendingLimit {
        type Builder = IssuingCardholderSpendingLimitBuilder;
    }

    impl FromValueOpt for IssuingCardholderSpendingLimit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardholderSpendingLimitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "categories" => b.categories = FromValueOpt::from_value(v),
                    "interval" => b.interval = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
/// Omitting this field will apply the limit to all categories.
pub type IssuingCardholderSpendingLimitCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Interval (or event) to which the amount applies.
pub type IssuingCardholderSpendingLimitInterval = stripe_shared::IssuingCardSpendingLimitInterval;
