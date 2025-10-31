#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKakaoPay {
    /// A unique identifier for the buyer as determined by the local payment processor.
    pub buyer_id: Option<String>,
    /// The Kakao Pay transaction ID associated with this payment.
    pub transaction_id: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsKakaoPayBuilder {
    buyer_id: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsKakaoPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsKakaoPay>,
        builder: PaymentMethodDetailsKakaoPayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsKakaoPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsKakaoPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsKakaoPayBuilder {
        type Out = PaymentMethodDetailsKakaoPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.buyer_id),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { buyer_id: Deserialize::default(), transaction_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(buyer_id), Some(transaction_id)) =
                (self.buyer_id.take(), self.transaction_id.take())
            else {
                return None;
            };
            Some(Self::Out { buyer_id, transaction_id })
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

    impl ObjectDeser for PaymentMethodDetailsKakaoPay {
        type Builder = PaymentMethodDetailsKakaoPayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsKakaoPay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsKakaoPayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "buyer_id" => b.buyer_id = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
