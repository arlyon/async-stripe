/// This hash contains whether the Payment Element is enabled and the features it supports.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CustomerSessionResourceComponentsResourcePaymentElement {
    /// Whether the Payment Element is enabled.
    pub enabled: bool,
    /// This hash defines whether the Payment Element supports certain features.
    pub features: Option<
        stripe_core::CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures,
    >,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CustomerSessionResourceComponentsResourcePaymentElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CustomerSessionResourceComponentsResourcePaymentElement")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CustomerSessionResourceComponentsResourcePaymentElementBuilder {
    enabled: Option<bool>,
    features: Option<
        Option<
            stripe_core::CustomerSessionResourceComponentsResourcePaymentElementResourceFeatures,
        >,
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

    impl Deserialize for CustomerSessionResourceComponentsResourcePaymentElement {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CustomerSessionResourceComponentsResourcePaymentElement>,
        builder: CustomerSessionResourceComponentsResourcePaymentElementBuilder,
    }

    impl Visitor for Place<CustomerSessionResourceComponentsResourcePaymentElement> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CustomerSessionResourceComponentsResourcePaymentElementBuilder {
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
                Some(CustomerSessionResourceComponentsResourcePaymentElement { enabled, features });
            Ok(())
        }
    }
};
