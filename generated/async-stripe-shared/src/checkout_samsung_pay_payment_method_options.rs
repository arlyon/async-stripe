#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutSamsungPayPaymentMethodOptions {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<CheckoutSamsungPayPaymentMethodOptionsCaptureMethod>,
}
#[doc(hidden)]
pub struct CheckoutSamsungPayPaymentMethodOptionsBuilder {
    capture_method: Option<Option<CheckoutSamsungPayPaymentMethodOptionsCaptureMethod>>,
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

    impl Deserialize for CheckoutSamsungPayPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSamsungPayPaymentMethodOptions>,
        builder: CheckoutSamsungPayPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutSamsungPayPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutSamsungPayPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutSamsungPayPaymentMethodOptionsBuilder {
        type Out = CheckoutSamsungPayPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => Deserialize::begin(&mut self.capture_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { capture_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(capture_method),) = (self.capture_method,) else {
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for CheckoutSamsungPayPaymentMethodOptions {
        type Builder = CheckoutSamsungPayPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutSamsungPayPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutSamsungPayPaymentMethodOptionsBuilder::deser_default();
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    Manual,
}
impl CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use CheckoutSamsungPayPaymentMethodOptionsCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSamsungPayPaymentMethodOptionsCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSamsungPayPaymentMethodOptionsCaptureMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutSamsungPayPaymentMethodOptionsCaptureMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSamsungPayPaymentMethodOptionsCaptureMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSamsungPayPaymentMethodOptionsCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutSamsungPayPaymentMethodOptionsCaptureMethod",
            )
        })
    }
}
