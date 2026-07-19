#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// The [Payment Method Configuration](/api/payment_method_configurations) to use for this portal session.
    /// When specified, customers will be able to update their payment method to one of the options specified by the payment method configuration.
    /// If not set, the default payment method configuration is used.
    pub payment_method_configuration: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalPaymentMethodUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalPaymentMethodUpdate").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalPaymentMethodUpdateBuilder {
    enabled: Option<bool>,
    payment_method_configuration: Option<Option<String>>,
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

    impl Deserialize for PortalPaymentMethodUpdate {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalPaymentMethodUpdate>,
        builder: PortalPaymentMethodUpdateBuilder,
    }

    impl Visitor for Place<PortalPaymentMethodUpdate> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalPaymentMethodUpdateBuilder {
                    enabled: Deserialize::default(),
                    payment_method_configuration: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "payment_method_configuration" => {
                    Deserialize::begin(&mut self.builder.payment_method_configuration)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(enabled), Some(payment_method_configuration)) =
                (self.builder.enabled, self.builder.payment_method_configuration.take())
            else {
                return Ok(());
            };
            *self.out = Some(PortalPaymentMethodUpdate { enabled, payment_method_configuration });
            Ok(())
        }
    }
};
