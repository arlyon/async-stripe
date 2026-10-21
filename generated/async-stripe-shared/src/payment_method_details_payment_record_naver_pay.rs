#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsPaymentRecordNaverPay {
    /// A unique identifier for the buyer as determined by the local payment processor.
    pub buyer_id: Option<String>,
    /// The Naver Pay transaction ID associated with this payment.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsPaymentRecordNaverPay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsPaymentRecordNaverPay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsPaymentRecordNaverPayBuilder {
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

    impl Deserialize for PaymentMethodDetailsPaymentRecordNaverPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsPaymentRecordNaverPay>,
        builder: PaymentMethodDetailsPaymentRecordNaverPayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsPaymentRecordNaverPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsPaymentRecordNaverPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsPaymentRecordNaverPayBuilder {
        type Out = PaymentMethodDetailsPaymentRecordNaverPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "buyer_id" => Deserialize::begin(&mut self.buyer_id),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { buyer_id: Some(None), transaction_id: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { buyer_id: Some(buyer_id), transaction_id: Some(transaction_id) } => {
                    Some(Self::Out { buyer_id, transaction_id })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for PaymentMethodDetailsPaymentRecordNaverPay {
        type Builder = PaymentMethodDetailsPaymentRecordNaverPayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsPaymentRecordNaverPay {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
