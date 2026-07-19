/// This hash contains whether the buy button is enabled.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourceBuyButton {
    /// Whether the buy button is enabled.
    pub enabled: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponentsResourceBuyButton {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponentsResourceBuyButton").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourceBuyButtonBuilder {
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

    impl Deserialize for CustomerSessionResourceComponentsResourceBuyButton {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponentsResourceBuyButton>,
        builder: CustomerSessionResourceComponentsResourceBuyButtonBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourceBuyButton> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionResourceComponentsResourceBuyButtonBuilder {
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
            *self.out = Some(CustomerSessionResourceComponentsResourceBuyButton { enabled });
            Ok(())
        }
    }
};
