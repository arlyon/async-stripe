#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPrivatePaymentMethodsAlipayDetails").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentFlowsPrivatePaymentMethodsAlipayDetailsBuilder {
                    buyer_id: Deserialize::default(),
                    fingerprint: Deserialize::default(),
                    transaction_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.builder.buyer_id),
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "transaction_id" => Deserialize::begin(&mut self.builder.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(buyer_id), Some(fingerprint), Some(transaction_id)) = (
                self.builder.buyer_id.take(),
                self.builder.fingerprint.take(),
                self.builder.transaction_id.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsPrivatePaymentMethodsAlipayDetails {
                buyer_id,
                fingerprint,
                transaction_id,
            });
            Ok(())
        }
    }
};
