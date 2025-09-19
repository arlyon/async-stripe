/// This hash contains whether the pricing table is enabled.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourcePricingTable {
    /// Whether the pricing table is enabled.
    pub enabled: bool,
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourcePricingTableBuilder {
    enabled: Option<bool>,
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
                builder:
                    CustomerSessionResourceComponentsResourcePricingTableBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CustomerSessionResourceComponentsResourcePricingTableBuilder {
        type Out = CustomerSessionResourceComponentsResourcePricingTable;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled),) = (self.enabled,) else {
                return None;
            };
            Some(Self::Out { enabled })
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

    impl ObjectDeser for CustomerSessionResourceComponentsResourcePricingTable {
        type Builder = CustomerSessionResourceComponentsResourcePricingTableBuilder;
    }

    impl FromValueOpt for CustomerSessionResourceComponentsResourcePricingTable {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                CustomerSessionResourceComponentsResourcePricingTableBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
