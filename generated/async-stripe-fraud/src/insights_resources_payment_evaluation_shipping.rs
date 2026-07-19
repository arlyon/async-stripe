/// Shipping details attached to this payment.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationShipping {
    pub address: stripe_fraud::InsightsResourcesPaymentEvaluationAddress,
    /// Shipping name.
    pub name: Option<String>,
    /// Shipping phone number.
    pub phone: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InsightsResourcesPaymentEvaluationShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InsightsResourcesPaymentEvaluationShipping").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationShippingBuilder {
    address: Option<stripe_fraud::InsightsResourcesPaymentEvaluationAddress>,
    name: Option<Option<String>>,
    phone: Option<Option<String>>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationShipping {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationShipping>,
        builder: InsightsResourcesPaymentEvaluationShippingBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationShipping> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationShippingBuilder {
                    address: Deserialize::default(),
                    name: Deserialize::default(),
                    phone: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "name" => Deserialize::begin(&mut self.builder.name),
                "phone" => Deserialize::begin(&mut self.builder.phone),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(address), Some(name), Some(phone)) =
                (self.builder.address.take(), self.builder.name.take(), self.builder.phone.take())
            else {
                return Ok(());
            };
            *self.out = Some(InsightsResourcesPaymentEvaluationShipping { address, name, phone });
            Ok(())
        }
    }
};
