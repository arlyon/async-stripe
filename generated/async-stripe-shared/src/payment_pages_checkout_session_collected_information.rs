#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCollectedInformation {
    /// Shipping information for this Checkout Session.
    pub shipping_details: Option<stripe_shared::PaymentPagesCheckoutSessionCheckoutAddressDetails>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCollectedInformationBuilder {
    shipping_details:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionCheckoutAddressDetails>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCollectedInformation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCollectedInformation>,
        builder: PaymentPagesCheckoutSessionCollectedInformationBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCollectedInformation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCollectedInformationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCollectedInformationBuilder {
        type Out = PaymentPagesCheckoutSessionCollectedInformation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "shipping_details" => Deserialize::begin(&mut self.shipping_details),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { shipping_details: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(shipping_details),) = (self.shipping_details.take(),) else {
                return None;
            };
            Some(Self::Out { shipping_details })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCollectedInformation {
        type Builder = PaymentPagesCheckoutSessionCollectedInformationBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCollectedInformation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCollectedInformationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "shipping_details" => b.shipping_details = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
