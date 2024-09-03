#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsSwish {
    /// Uniquely identifies the payer's Swish account.
    /// You can use this attribute to check whether two Swish transactions were paid for by the same payer.
    pub fingerprint: Option<String>,
    /// Payer bank reference number for the payment
    pub payment_reference: Option<String>,
    /// The last four digits of the Swish account phone number
    pub verified_phone_last4: Option<String>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsSwishBuilder {
    fingerprint: Option<Option<String>>,
    payment_reference: Option<Option<String>>,
    verified_phone_last4: Option<Option<String>>,
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

    impl Deserialize for PaymentMethodDetailsSwish {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsSwish>,
        builder: PaymentMethodDetailsSwishBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsSwish> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsSwishBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsSwishBuilder {
        type Out = PaymentMethodDetailsSwish;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "payment_reference" => Deserialize::begin(&mut self.payment_reference),
                "verified_phone_last4" => Deserialize::begin(&mut self.verified_phone_last4),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                fingerprint: Deserialize::default(),
                payment_reference: Deserialize::default(),
                verified_phone_last4: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(fingerprint), Some(payment_reference), Some(verified_phone_last4)) = (
                self.fingerprint.take(),
                self.payment_reference.take(),
                self.verified_phone_last4.take(),
            ) else {
                return None;
            };
            Some(Self::Out { fingerprint, payment_reference, verified_phone_last4 })
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

    impl ObjectDeser for PaymentMethodDetailsSwish {
        type Builder = PaymentMethodDetailsSwishBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsSwish {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsSwishBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fingerprint" => b.fingerprint = FromValueOpt::from_value(v),
                    "payment_reference" => b.payment_reference = FromValueOpt::from_value(v),
                    "verified_phone_last4" => b.verified_phone_last4 = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
