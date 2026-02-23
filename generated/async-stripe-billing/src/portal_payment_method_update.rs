#[derive(Clone, Debug, Eq, PartialEq)]
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
#[doc(hidden)]
pub struct PortalPaymentMethodUpdateBuilder {
    enabled: Option<bool>,
    payment_method_configuration: Option<Option<String>>,
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
                builder: PortalPaymentMethodUpdateBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalPaymentMethodUpdateBuilder {
        type Out = PortalPaymentMethodUpdate;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "payment_method_configuration" => {
                    Deserialize::begin(&mut self.payment_method_configuration)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                enabled: Deserialize::default(),
                payment_method_configuration: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled), Some(payment_method_configuration)) =
                (self.enabled, self.payment_method_configuration.take())
            else {
                return None;
            };
            Some(Self::Out { enabled, payment_method_configuration })
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

    impl ObjectDeser for PortalPaymentMethodUpdate {
        type Builder = PortalPaymentMethodUpdateBuilder;
    }

    impl FromValueOpt for PortalPaymentMethodUpdate {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalPaymentMethodUpdateBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "payment_method_configuration" => {
                        b.payment_method_configuration = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
