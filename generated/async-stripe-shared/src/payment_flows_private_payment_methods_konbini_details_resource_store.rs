#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<stripe_shared::Chain>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreBuilder {
    chain: Option<Option<stripe_shared::Chain>>,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore>,
        builder: PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreBuilder {
        type Out = PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "chain" => Deserialize::begin(&mut self.chain),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { chain: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(chain),) = (self.chain.take(),) else {
                return None;
            };
            Some(Self::Out { chain })
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore {
        type Builder = PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreBuilder;
    }

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "chain" => b.chain = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The name of the convenience store chain where the payment was completed.
pub type PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain = stripe_shared::Chain;
