#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCollectedInformation {
    /// Customer’s business name for this Checkout Session
    pub business_name: Option<String>,
    /// Customer’s individual name for this Checkout Session
    pub individual_name: Option<String>,
    /// Shipping information for this Checkout Session.
    pub shipping_details: Option<stripe_shared::PaymentPagesCheckoutSessionCheckoutAddressDetails>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCollectedInformationBuilder {
    business_name: Option<Option<String>>,
    individual_name: Option<Option<String>>,
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
    use miniserde::{Deserialize, Result, make_place};
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
                "business_name" => Deserialize::begin(&mut self.business_name),
                "individual_name" => Deserialize::begin(&mut self.individual_name),
                "shipping_details" => Deserialize::begin(&mut self.shipping_details),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                business_name: Deserialize::default(),
                individual_name: Deserialize::default(),
                shipping_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(business_name), Some(individual_name), Some(shipping_details)) = (
                self.business_name.take(),
                self.individual_name.take(),
                self.shipping_details.take(),
            ) else {
                return None;
            };
            Some(Self::Out { business_name, individual_name, shipping_details })
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
                    "business_name" => b.business_name = FromValueOpt::from_value(v),
                    "individual_name" => b.individual_name = FromValueOpt::from_value(v),
                    "shipping_details" => b.shipping_details = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
