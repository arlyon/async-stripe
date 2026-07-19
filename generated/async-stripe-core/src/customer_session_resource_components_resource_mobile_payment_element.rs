/// This hash contains whether the mobile payment element is enabled and the features it supports.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElement {
    /// Whether the mobile payment element is enabled.
    pub enabled: bool,
    /// This hash defines whether the mobile payment element supports certain features.
    pub features: Option<
        stripe_core::CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures,
    >,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponentsResourceMobilePaymentElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponentsResourceMobilePaymentElement")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourceMobilePaymentElementBuilder {
    enabled: Option<bool>,
features: Option<Option<stripe_core::CustomerSessionResourceComponentsResourceMobilePaymentElementResourceFeatures>>,

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

    impl Deserialize for CustomerSessionResourceComponentsResourceMobilePaymentElement {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponentsResourceMobilePaymentElement>,
        builder: CustomerSessionResourceComponentsResourceMobilePaymentElementBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourceMobilePaymentElement> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionResourceComponentsResourceMobilePaymentElementBuilder {
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
            *self.out = Some(CustomerSessionResourceComponentsResourceMobilePaymentElement {
                enabled,
                features,
            });
            Ok(())
        }
    }
};
