#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PlanTier {
    /// Price for the entire tier.
    pub flat_amount: Option<i64>,
    /// Same as `flat_amount`, but contains a decimal value with at most 12 decimal places.
    pub flat_amount_decimal: Option<String>,
    /// Per unit price for units relevant to the tier.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    /// Up to and including to this quantity will be contained in the tier.
    pub up_to: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PlanTier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PlanTier").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PlanTierBuilder {
    flat_amount: Option<Option<i64>>,
    flat_amount_decimal: Option<Option<String>>,
    unit_amount: Option<Option<i64>>,
    unit_amount_decimal: Option<Option<String>>,
    up_to: Option<Option<i64>>,
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

    impl Deserialize for PlanTier {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PlanTier>,
        builder: PlanTierBuilder,
    }

    impl Visitor for Place<PlanTier> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PlanTierBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PlanTierBuilder {
        type Out = PlanTier;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flat_amount" => Deserialize::begin(&mut self.flat_amount),
                "flat_amount_decimal" => Deserialize::begin(&mut self.flat_amount_decimal),
                "unit_amount" => Deserialize::begin(&mut self.unit_amount),
                "unit_amount_decimal" => Deserialize::begin(&mut self.unit_amount_decimal),
                "up_to" => Deserialize::begin(&mut self.up_to),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                flat_amount: Some(None),
                flat_amount_decimal: Some(None),
                unit_amount: Some(None),
                unit_amount_decimal: Some(None),
                up_to: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    flat_amount: Some(flat_amount),
                    flat_amount_decimal: Some(flat_amount_decimal),
                    unit_amount: Some(unit_amount),
                    unit_amount_decimal: Some(unit_amount_decimal),
                    up_to: Some(up_to),
                } => Some(Self::Out {
                    flat_amount,
                    flat_amount_decimal,
                    unit_amount,
                    unit_amount_decimal,
                    up_to,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for PlanTier {
        type Builder = PlanTierBuilder;
    }

    impl FromValueOpt for PlanTier {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
