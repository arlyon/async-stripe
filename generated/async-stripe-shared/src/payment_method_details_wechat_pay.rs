#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsWechatPay {
    /// Uniquely identifies this particular WeChat Pay account.
    /// You can use this attribute to check whether two WeChat accounts are the same.
    pub fingerprint: Option<String>,
    /// Transaction ID of this particular WeChat Pay transaction.
    pub transaction_id: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsWechatPayBuilder {
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

    impl Deserialize for PaymentMethodDetailsWechatPay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsWechatPay>,
        builder: PaymentMethodDetailsWechatPayBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsWechatPay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsWechatPayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsWechatPayBuilder {
        type Out = PaymentMethodDetailsWechatPay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "transaction_id" => Deserialize::begin(&mut self.transaction_id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { fingerprint: Deserialize::default(), transaction_id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fingerprint), Some(transaction_id)) =
                (self.fingerprint.take(), self.transaction_id.take())
            else {
                return None;
            };
            Some(Self::Out { fingerprint, transaction_id })
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

    impl ObjectDeser for PaymentMethodDetailsWechatPay {
        type Builder = PaymentMethodDetailsWechatPayBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsWechatPay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsWechatPayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "transaction_id" => b.transaction_id = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
