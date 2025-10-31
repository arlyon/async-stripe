/// Custom processors represent payment processors not modeled directly in
/// the Stripe API. This resource consists of details about the custom processor
/// used for this payment attempt.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails {
    /// An opaque string for manual reconciliation of this payment, for example a check number or a payment processor ID.
    pub payment_reference: Option<String>,
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder {
    payment_reference: Option<Option<String>>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails,
        >,
        builder:
            PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder,
    }

    impl Visitor
        for Place<PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder
    {
        type Out = PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_reference" => Deserialize::begin(&mut self.payment_reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payment_reference: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_reference),) = (self.payment_reference.take(),) else {
                return None;
            };
            Some(Self::Out { payment_reference })
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

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails {
        type Builder =
            PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder;
    }

    impl FromValueOpt
        for PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetails
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentsPrimitivesPaymentRecordsResourceProcessorDetailsResourceCustomDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_reference" => b.payment_reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
