#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionDisplayMultibancoDetails {
    /// Entity number associated with this Multibanco payment.
    pub entity: Option<String>,
    /// The timestamp at which the Multibanco voucher expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The URL for the hosted Multibanco voucher page, which allows customers to view a Multibanco voucher.
    pub hosted_voucher_url: Option<String>,
    /// Reference number associated with this Multibanco payment.
    pub reference: Option<String>,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionDisplayMultibancoDetailsBuilder {
    entity: Option<Option<String>>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    hosted_voucher_url: Option<Option<String>>,
    reference: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentNextActionDisplayMultibancoDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionDisplayMultibancoDetails>,
        builder: PaymentIntentNextActionDisplayMultibancoDetailsBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionDisplayMultibancoDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionDisplayMultibancoDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionDisplayMultibancoDetailsBuilder {
        type Out = PaymentIntentNextActionDisplayMultibancoDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "entity" => Deserialize::begin(&mut self.entity),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "hosted_voucher_url" => Deserialize::begin(&mut self.hosted_voucher_url),
                "reference" => Deserialize::begin(&mut self.reference),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                entity: Deserialize::default(),
                expires_at: Deserialize::default(),
                hosted_voucher_url: Deserialize::default(),
                reference: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(entity), Some(expires_at), Some(hosted_voucher_url), Some(reference)) = (
                self.entity.take(),
                self.expires_at,
                self.hosted_voucher_url.take(),
                self.reference.take(),
            ) else {
                return None;
            };
            Some(Self::Out { entity, expires_at, hosted_voucher_url, reference })
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

    impl ObjectDeser for PaymentIntentNextActionDisplayMultibancoDetails {
        type Builder = PaymentIntentNextActionDisplayMultibancoDetailsBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionDisplayMultibancoDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionDisplayMultibancoDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "entity" => b.entity = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "hosted_voucher_url" => b.hosted_voucher_url = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
