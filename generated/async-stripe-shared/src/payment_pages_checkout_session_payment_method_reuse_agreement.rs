#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    /// When set to `auto`, Stripe's defaults will be used.
    ///
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: stripe_shared::Position,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionPaymentMethodReuseAgreement")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionPaymentMethodReuseAgreementBuilder {
    position: Option<stripe_shared::Position>,
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

    impl Deserialize for PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionPaymentMethodReuseAgreement>,
        builder: PaymentPagesCheckoutSessionPaymentMethodReuseAgreementBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionPaymentMethodReuseAgreement> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    PaymentPagesCheckoutSessionPaymentMethodReuseAgreementBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementBuilder {
        type Out = PaymentPagesCheckoutSessionPaymentMethodReuseAgreement;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "position" => Deserialize::begin(&mut self.position),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { position: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(position),) = (self.position.take(),) else {
                return None;
            };
            Some(Self::Out { position })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
        type Builder = PaymentPagesCheckoutSessionPaymentMethodReuseAgreementBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentPagesCheckoutSessionPaymentMethodReuseAgreementBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "position" => b.position = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines the position and visibility of the payment method reuse agreement in the UI.
/// When set to `auto`, Stripe's defaults will be used.
///
/// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
pub type PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition = stripe_shared::Position;
