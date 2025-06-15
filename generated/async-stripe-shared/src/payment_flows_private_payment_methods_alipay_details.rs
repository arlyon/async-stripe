#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    /// Uniquely identifies this particular Alipay account.
    /// You can use this attribute to check whether two Alipay accounts are the same.
    pub buyer_id: Option<String>,
    /// Uniquely identifies this particular Alipay account.
    /// You can use this attribute to check whether two Alipay accounts are the same.
    pub fingerprint: Option<String>,
    /// Transaction ID of this particular Alipay transaction.
    pub transaction_id: Option<String>,
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipayDetailsBuilder {
    buyer_id: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsAlipayDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsAlipayDetails>,
        builder: PaymentFlowsPrivatePaymentMethodsAlipayDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsAlipayDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPrivatePaymentMethodsAlipayDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsAlipayDetailsBuilder {
        type Out = PaymentFlowsPrivatePaymentMethodsAlipayDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.buyer_id),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                buyer_id: Deserialize::default(),
                fingerprint: Deserialize::default(),
                transaction_id: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(buyer_id), Some(fingerprint), Some(transaction_id)) =
                (self.buyer_id.take(), self.fingerprint.take(), self.transaction_id.take())
            else {
                return None;
            };
            Some(Self::Out { buyer_id, fingerprint, transaction_id })
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsAlipayDetails {
        type Builder = PaymentFlowsPrivatePaymentMethodsAlipayDetailsBuilder;
    }

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsAlipayDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPrivatePaymentMethodsAlipayDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "buyer_id" => b.buyer_id = FromValueOpt::from_value(v),
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
