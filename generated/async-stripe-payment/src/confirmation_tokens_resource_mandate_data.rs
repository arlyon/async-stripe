/// Data used for generating a Mandate.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourceMandateData {
    pub customer_acceptance:
        stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptance,
}
#[doc(hidden)]
pub struct ConfirmationTokensResourceMandateDataBuilder {
    customer_acceptance:
        Option<stripe_payment::ConfirmationTokensResourceMandateDataResourceCustomerAcceptance>,
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

    impl Deserialize for ConfirmationTokensResourceMandateData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokensResourceMandateData>,
        builder: ConfirmationTokensResourceMandateDataBuilder,
    }

    impl Visitor for Place<ConfirmationTokensResourceMandateData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokensResourceMandateDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConfirmationTokensResourceMandateDataBuilder {
        type Out = ConfirmationTokensResourceMandateData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_acceptance" => Deserialize::begin(&mut self.customer_acceptance),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { customer_acceptance: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer_acceptance),) = (self.customer_acceptance.take(),) else {
                return None;
            };
            Some(Self::Out { customer_acceptance })
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

    impl ObjectDeser for ConfirmationTokensResourceMandateData {
        type Builder = ConfirmationTokensResourceMandateDataBuilder;
    }

    impl FromValueOpt for ConfirmationTokensResourceMandateData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConfirmationTokensResourceMandateDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_acceptance" => b.customer_acceptance = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
