#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsKakaoPay {
    /// A unique identifier for the buyer as determined by the local payment processor.
    pub buyer_id: Option<String>,
    /// The Kakao Pay transaction ID associated with this payment.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsKakaoPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsKakaoPay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsKakaoPayBuilder {
    buyer_id: Option<Option<String>>,
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
                builder: PaymentMethodDetailsKakaoPayBuilder {
                    buyer_id: Deserialize::default(),
                    transaction_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.builder.buyer_id),
                "transaction_id" => Deserialize::begin(&mut self.builder.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(buyer_id), Some(transaction_id)) =
                (self.builder.buyer_id.take(), self.builder.transaction_id.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsKakaoPay { buyer_id, transaction_id });
            Ok(())
        }
    }
};
