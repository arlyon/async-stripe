#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPaymentDetails {
    /// A unique value to identify the customer. This field is available only for card payments.
    ///
    /// This field is truncated to 25 alphanumeric characters, excluding spaces, before being sent to card networks.
    pub customer_reference: Option<String>,
    /// A unique value assigned by the business to identify the transaction. Required for L2 and L3 rates.
    ///
    /// Required when the Payment Method Types array contains `card`, including when [automatic_payment_methods.enabled](/api/payment_intents/create#create_payment_intent-automatic_payment_methods-enabled) is set to `true`.
    ///
    /// For Cards, this field is truncated to 25 alphanumeric characters, excluding spaces, before being sent to card networks.
    /// For Klarna, this field is truncated to 255 characters and is visible to customers when they view the order in the Klarna app.
    pub order_reference: Option<String>,
}
#[doc(hidden)]
pub struct PaymentFlowsPaymentDetailsBuilder {
    customer_reference: Option<Option<String>>,
    order_reference: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsPaymentDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPaymentDetails>,
        builder: PaymentFlowsPaymentDetailsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPaymentDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPaymentDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsPaymentDetailsBuilder {
        type Out = PaymentFlowsPaymentDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_reference" => Deserialize::begin(&mut self.customer_reference),
                "order_reference" => Deserialize::begin(&mut self.order_reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                customer_reference: Deserialize::default(),
                order_reference: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(customer_reference), Some(order_reference)) =
                (self.customer_reference.take(), self.order_reference.take())
            else {
                return None;
            };
            Some(Self::Out { customer_reference, order_reference })
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

    impl ObjectDeser for PaymentFlowsPaymentDetails {
        type Builder = PaymentFlowsPaymentDetailsBuilder;
    }

    impl FromValueOpt for PaymentFlowsPaymentDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsPaymentDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_reference" => b.customer_reference = FromValueOpt::from_value(v),
                    "order_reference" => b.order_reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
