#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    /// Indicates whether or not multiple captures are supported.
    pub status:
        stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder {
    status: Option<
        stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus,
    >,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>,
        builder: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder {
        type Out = PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(status),) = (self.status.take(),) else {
                return None;
            };
            Some(Self::Out { status })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
        type Builder = PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder;
    }

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates whether or not multiple captures are supported.
pub type PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus =
    stripe_shared::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus;
