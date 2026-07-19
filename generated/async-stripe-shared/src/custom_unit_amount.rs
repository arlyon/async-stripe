#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomUnitAmount {
    /// The maximum unit amount the customer can specify for this item.
    pub maximum: Option<i64>,
    /// The minimum unit amount the customer can specify for this item.
    /// Must be at least the minimum charge amount.
    pub minimum: Option<i64>,
    /// The starting unit amount which can be updated by the customer.
    pub preset: Option<i64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomUnitAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomUnitAmount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomUnitAmountBuilder {
    maximum: Option<Option<i64>>,
    minimum: Option<Option<i64>>,
    preset: Option<Option<i64>>,
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

    impl Deserialize for CustomUnitAmount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomUnitAmount>,
        builder: CustomUnitAmountBuilder,
    }

    impl Visitor for Place<CustomUnitAmount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomUnitAmountBuilder {
                    maximum: Deserialize::default(),
                    minimum: Deserialize::default(),
                    preset: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "maximum" => Deserialize::begin(&mut self.builder.maximum),
                "minimum" => Deserialize::begin(&mut self.builder.minimum),
                "preset" => Deserialize::begin(&mut self.builder.preset),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(maximum), Some(minimum), Some(preset)) =
                (self.builder.maximum, self.builder.minimum, self.builder.preset)
            else {
                return Ok(());
            };
            *self.out = Some(CustomUnitAmount { maximum, minimum, preset });
            Ok(())
        }
    }
};
