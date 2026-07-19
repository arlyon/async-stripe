#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCollectedInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionCollectedInformation").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentPagesCheckoutSessionCollectedInformationBuilder {
                    business_name: Deserialize::default(),
                    individual_name: Deserialize::default(),
                    shipping_details: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "business_name" => Deserialize::begin(&mut self.builder.business_name),
                "individual_name" => Deserialize::begin(&mut self.builder.individual_name),
                "shipping_details" => Deserialize::begin(&mut self.builder.shipping_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(business_name), Some(individual_name), Some(shipping_details)) = (
                self.builder.business_name.take(),
                self.builder.individual_name.take(),
                self.builder.shipping_details.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionCollectedInformation {
                business_name,
                individual_name,
                shipping_details,
            });
            Ok(())
        }
    }
};
