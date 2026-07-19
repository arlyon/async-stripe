/// This hash contains whether the customer sheet is enabled and the features it supports.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourceCustomerSheet {
    /// Whether the customer sheet is enabled.
    pub enabled: bool,
    /// This hash defines whether the customer sheet supports certain features.
    pub features:
        Option<stripe_core::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponentsResourceCustomerSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponentsResourceCustomerSheet")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourceCustomerSheetBuilder {
    enabled: Option<bool>,
    features: Option<
        Option<stripe_core::CustomerSessionResourceComponentsResourceCustomerSheetResourceFeatures>,
    >,
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

    impl Deserialize for CustomerSessionResourceComponentsResourceCustomerSheet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponentsResourceCustomerSheet>,
        builder: CustomerSessionResourceComponentsResourceCustomerSheetBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourceCustomerSheet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionResourceComponentsResourceCustomerSheetBuilder {
                    enabled: Deserialize::default(),
                    features: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "features" => Deserialize::begin(&mut self.builder.features),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled), Some(features)) =
                (self.builder.enabled, self.builder.features.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(CustomerSessionResourceComponentsResourceCustomerSheet { enabled, features });
            Ok(())
        }
    }
};
