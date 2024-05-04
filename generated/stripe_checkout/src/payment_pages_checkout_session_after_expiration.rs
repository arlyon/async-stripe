#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionAfterExpiration {
    /// When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<stripe_checkout::PaymentPagesCheckoutSessionAfterExpirationRecovery>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionAfterExpirationBuilder {
    recovery: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionAfterExpirationRecovery>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentPagesCheckoutSessionAfterExpiration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionAfterExpiration>,
        builder: PaymentPagesCheckoutSessionAfterExpirationBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionAfterExpiration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionAfterExpirationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionAfterExpirationBuilder {
        type Out = PaymentPagesCheckoutSessionAfterExpiration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "recovery" => Deserialize::begin(&mut self.recovery),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { recovery: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { recovery: self.recovery.take()? })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionAfterExpiration {
        type Builder = PaymentPagesCheckoutSessionAfterExpirationBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionAfterExpiration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionAfterExpirationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "recovery" => b.recovery = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};