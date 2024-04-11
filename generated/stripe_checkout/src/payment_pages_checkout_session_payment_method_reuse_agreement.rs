#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    /// When set to `auto`, Stripe's defaults will be used.
    ///
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionPaymentMethodReuseAgreementBuilder {
    position: Option<PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Self { position: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { position: self.position? })
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
                    "position" => b.position = Some(FromValueOpt::from_value(v)?),

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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}
impl PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::*;
        match self {
            Auto => "auto",
            Hidden => "hidden",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::*;
        match s {
            "auto" => Ok(Auto),
            "hidden" => Ok(Hidden),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition",
            )
        })
    }
}
