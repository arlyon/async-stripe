#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsAmazonPay {
    pub funding: Option<stripe_shared::AmazonPayUnderlyingPaymentMethodFundingDetails>,
    /// The Amazon Pay transaction ID associated with this payment.
    pub transaction_id: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsAmazonPayBuilder {
    funding: Option<Option<stripe_shared::AmazonPayUnderlyingPaymentMethodFundingDetails>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetailsAmazonPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsAmazonPay>,
        builder: PaymentMethodDetailsAmazonPayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsAmazonPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsAmazonPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsAmazonPayBuilder {
        type Out = PaymentMethodDetailsAmazonPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "funding" => Deserialize::begin(&mut self.funding),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { funding: Deserialize::default(), transaction_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(funding), Some(transaction_id)) =
                (self.funding.take(), self.transaction_id.take())
            else {
                return None;
            };
            Some(Self::Out { funding, transaction_id })
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

    impl ObjectDeser for PaymentMethodDetailsAmazonPay {
        type Builder = PaymentMethodDetailsAmazonPayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsAmazonPay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsAmazonPayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "funding" => b.funding = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
