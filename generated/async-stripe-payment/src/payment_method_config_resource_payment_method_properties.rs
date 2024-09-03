#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    /// Whether this payment method may be offered at checkout.
    /// True if `display_preference` is `on` and the payment method's capability is active.
    pub available: bool,
    pub display_preference: stripe_payment::PaymentMethodConfigResourceDisplayPreference,
}
#[doc(hidden)]
pub struct PaymentMethodConfigResourcePaymentMethodPropertiesBuilder {
    available: Option<bool>,
    display_preference: Option<stripe_payment::PaymentMethodConfigResourceDisplayPreference>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodConfigResourcePaymentMethodProperties {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfigResourcePaymentMethodProperties>,
        builder: PaymentMethodConfigResourcePaymentMethodPropertiesBuilder,
    }

    impl Visitor for Place<PaymentMethodConfigResourcePaymentMethodProperties> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodConfigResourcePaymentMethodPropertiesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodConfigResourcePaymentMethodPropertiesBuilder {
        type Out = PaymentMethodConfigResourcePaymentMethodProperties;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.available),
                "display_preference" => Deserialize::begin(&mut self.display_preference),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default(), display_preference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(available), Some(display_preference)) =
                (self.available, self.display_preference)
            else {
                return None;
            };
            Some(Self::Out { available, display_preference })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentMethodConfigResourcePaymentMethodProperties {
        type Builder = PaymentMethodConfigResourcePaymentMethodPropertiesBuilder;
    }

    impl FromValueOpt for PaymentMethodConfigResourcePaymentMethodProperties {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodConfigResourcePaymentMethodPropertiesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available" => b.available = FromValueOpt::from_value(v),
                    "display_preference" => b.display_preference = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
