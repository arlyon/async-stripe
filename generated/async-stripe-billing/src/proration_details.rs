#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ProrationDetails {
    /// Discount amounts applied when the proration was created.
    pub discount_amounts: Vec<stripe_shared::DiscountsResourceDiscountAmount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProrationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProrationDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ProrationDetailsBuilder {
    discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
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

    impl Deserialize for ProrationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ProrationDetails>,
        builder: ProrationDetailsBuilder,
    }

    impl Visitor for Place<ProrationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ProrationDetailsBuilder { discount_amounts: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discount_amounts" => Deserialize::begin(&mut self.builder.discount_amounts),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(discount_amounts),) = (self.builder.discount_amounts.take(),) else {
                return Ok(());
            };
            *self.out = Some(ProrationDetails { discount_amounts });
            Ok(())
        }
    }
};
