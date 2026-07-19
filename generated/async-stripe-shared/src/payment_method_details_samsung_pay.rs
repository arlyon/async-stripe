#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsSamsungPay {
    /// A unique identifier for the buyer as determined by the local payment processor.
    pub buyer_id: Option<String>,
    /// The Samsung Pay transaction ID associated with this payment.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsSamsungPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsSamsungPay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsSamsungPayBuilder {
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

    impl Deserialize for PaymentMethodDetailsSamsungPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsSamsungPay>,
        builder: PaymentMethodDetailsSamsungPayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsSamsungPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsSamsungPayBuilder {
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
            *self.out = Some(PaymentMethodDetailsSamsungPay { buyer_id, transaction_id });
            Ok(())
        }
    }
};
