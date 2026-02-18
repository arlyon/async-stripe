#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceError {
    /// The code of the error that occurred when validating the current amount details.
    pub code: Option<PaymentFlowsAmountDetailsResourceErrorCode>,
    /// A message providing more details about the error.
    pub message: Option<String>,
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsResourceErrorBuilder {
    code: Option<Option<PaymentFlowsAmountDetailsResourceErrorCode>>,
    message: Option<Option<String>>,
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

    impl Deserialize for PaymentFlowsAmountDetailsResourceError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsAmountDetailsResourceError>,
        builder: PaymentFlowsAmountDetailsResourceErrorBuilder,
    }

    impl Visitor for Place<PaymentFlowsAmountDetailsResourceError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsAmountDetailsResourceErrorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentFlowsAmountDetailsResourceErrorBuilder {
        type Out = PaymentFlowsAmountDetailsResourceError;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "message" => Deserialize::begin(&mut self.message),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Deserialize::default(), message: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(message)) = (self.code.take(), self.message.take()) else {
                return None;
            };
            Some(Self::Out { code, message })
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

    impl ObjectDeser for PaymentFlowsAmountDetailsResourceError {
        type Builder = PaymentFlowsAmountDetailsResourceErrorBuilder;
    }

    impl FromValueOpt for PaymentFlowsAmountDetailsResourceError {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentFlowsAmountDetailsResourceErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "message" => b.message = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The code of the error that occurred when validating the current amount details.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsAmountDetailsResourceErrorCode {
    AmountDetailsAmountMismatch,
    AmountDetailsTaxShippingDiscountGreaterThanAmount,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentFlowsAmountDetailsResourceErrorCode {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsAmountDetailsResourceErrorCode::*;
        match self {
            AmountDetailsAmountMismatch => "amount_details_amount_mismatch",
            AmountDetailsTaxShippingDiscountGreaterThanAmount => {
                "amount_details_tax_shipping_discount_greater_than_amount"
            }
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentFlowsAmountDetailsResourceErrorCode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsAmountDetailsResourceErrorCode::*;
        match s {
            "amount_details_amount_mismatch" => Ok(AmountDetailsAmountMismatch),
            "amount_details_tax_shipping_discount_greater_than_amount" => {
                Ok(AmountDetailsTaxShippingDiscountGreaterThanAmount)
            }
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsAmountDetailsResourceErrorCode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentFlowsAmountDetailsResourceErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentFlowsAmountDetailsResourceErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsAmountDetailsResourceErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentFlowsAmountDetailsResourceErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentFlowsAmountDetailsResourceErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentFlowsAmountDetailsResourceErrorCode::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentFlowsAmountDetailsResourceErrorCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsAmountDetailsResourceErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
