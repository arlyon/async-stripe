/// This hash contains whether the pricing table is enabled.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourcePricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponentsResourcePricingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponentsResourcePricingTable")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourcePricingTableBuilder {
    enabled: Option<bool>,
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

    impl Deserialize for CustomerSessionResourceComponentsResourcePricingTable {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponentsResourcePricingTable>,
        builder: CustomerSessionResourceComponentsResourcePricingTableBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourcePricingTable> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionResourceComponentsResourcePricingTableBuilder {
                    enabled: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled),) = (self.builder.enabled,) else {
                return Ok(());
            };
            *self.out = Some(CustomerSessionResourceComponentsResourcePricingTable { enabled });
            Ok(())
        }
    }
};
