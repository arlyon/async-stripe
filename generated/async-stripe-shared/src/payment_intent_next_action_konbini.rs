#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentNextActionKonbini {
    /// The timestamp at which the pending Konbini payment expires.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,
    pub stores: stripe_shared::PaymentIntentNextActionKonbiniStores,
}
#[doc(hidden)]
pub struct PaymentIntentNextActionKonbiniBuilder {
    expires_at: Option<stripe_types::Timestamp>,
    hosted_voucher_url: Option<Option<String>>,
    stores: Option<stripe_shared::PaymentIntentNextActionKonbiniStores>,
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

    impl Deserialize for PaymentIntentNextActionKonbini {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentNextActionKonbini>,
        builder: PaymentIntentNextActionKonbiniBuilder,
    }

    impl Visitor for Place<PaymentIntentNextActionKonbini> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentNextActionKonbiniBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentNextActionKonbiniBuilder {
        type Out = PaymentIntentNextActionKonbini;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "hosted_voucher_url" => Deserialize::begin(&mut self.hosted_voucher_url),
                "stores" => Deserialize::begin(&mut self.stores),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                expires_at: Deserialize::default(),
                hosted_voucher_url: Deserialize::default(),
                stores: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(expires_at), Some(hosted_voucher_url), Some(stores)) =
                (self.expires_at, self.hosted_voucher_url.take(), self.stores.take())
            else {
                return None;
            };
            Some(Self::Out { expires_at, hosted_voucher_url, stores })
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

    impl ObjectDeser for PaymentIntentNextActionKonbini {
        type Builder = PaymentIntentNextActionKonbiniBuilder;
    }

    impl FromValueOpt for PaymentIntentNextActionKonbini {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentNextActionKonbiniBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "hosted_voucher_url" => b.hosted_voucher_url = FromValueOpt::from_value(v),
                    "stores" => b.stores = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
