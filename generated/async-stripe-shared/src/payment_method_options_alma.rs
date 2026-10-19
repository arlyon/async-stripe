#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsAlma {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsAlma {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsAlma").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsAlmaBuilder {
    capture_method: Option<Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>>,
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

    impl Deserialize for PaymentMethodOptionsAlma {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsAlma>,
        builder: PaymentMethodOptionsAlmaBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsAlma> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsAlmaBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsAlmaBuilder {
        type Out = PaymentMethodOptionsAlma;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => Deserialize::begin(&mut self.capture_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { capture_method: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(capture_method),) = (self.capture_method.take(),) else {
                return None;
            };
            Some(Self::Out { capture_method })
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

    impl ObjectDeser for PaymentMethodOptionsAlma {
        type Builder = PaymentMethodOptionsAlmaBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsAlma {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsAlmaBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "capture_method" => b.capture_method = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls when the funds will be captured from the customer's account.
pub type PaymentMethodOptionsAlmaCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
