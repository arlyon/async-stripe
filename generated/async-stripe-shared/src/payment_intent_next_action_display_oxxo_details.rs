#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    /// The timestamp after which the OXXO voucher expires.
    pub expires_after: Option<stripe_types::Timestamp>,
    /// The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher.
    pub hosted_voucher_url: Option<String>,
    /// OXXO reference number.
    pub number: Option<String>,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionDisplayOxxoDetailsBuilder {
    expires_after: Option<Option<stripe_types::Timestamp>>,
    hosted_voucher_url: Option<Option<String>>,
    number: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionDisplayOxxoDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayOxxoDetails>,
        builder: PaymentIntentNextActionDisplayOxxoDetailsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayOxxoDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionDisplayOxxoDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionDisplayOxxoDetailsBuilder {
        type Out = PaymentIntentNextActionDisplayOxxoDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_after" => Deserialize::begin(&mut self.expires_after),
                "hosted_voucher_url" => Deserialize::begin(&mut self.hosted_voucher_url),
                "number" => Deserialize::begin(&mut self.number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                expires_after: Deserialize::default(),
                hosted_voucher_url: Deserialize::default(),
                number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(expires_after), Some(hosted_voucher_url), Some(number)) =
                (self.expires_after, self.hosted_voucher_url.take(), self.number.take())
            else {
                return None;
            };
            Some(Self::Out { expires_after, hosted_voucher_url, number })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentIntentNextActionDisplayOxxoDetails {
        type Builder = PaymentIntentNextActionDisplayOxxoDetailsBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionDisplayOxxoDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionDisplayOxxoDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "expires_after" => b.expires_after = FromValueOpt::from_value(v),
                    "hosted_voucher_url" => b.hosted_voucher_url = FromValueOpt::from_value(v),
                    "number" => b.number = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
