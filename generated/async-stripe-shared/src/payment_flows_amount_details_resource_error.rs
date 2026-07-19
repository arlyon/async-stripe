#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsAmountDetailsResourceError {
    /// The code of the error that occurred when validating the current amount details.
    pub code: Option<PaymentFlowsAmountDetailsResourceErrorCode>,
    /// A message providing more details about the error.
    pub message: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAmountDetailsResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsAmountDetailsResourceError").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsAmountDetailsResourceErrorBuilder {
    code: Option<Option<PaymentFlowsAmountDetailsResourceErrorCode>>,
    message: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PaymentFlowsAmountDetailsResourceErrorBuilder {
                    code: Deserialize::default(),
                    message: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.builder.code),
                "message" => Deserialize::begin(&mut self.builder.message),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(code), Some(message)) =
                (self.builder.code.take(), self.builder.message.take())
            else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsAmountDetailsResourceError { code, message });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentFlowsAmountDetailsResourceErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsAmountDetailsResourceErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentFlowsAmountDetailsResourceErrorCode))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PaymentFlowsAmountDetailsResourceErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentFlowsAmountDetailsResourceErrorCode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentFlowsAmountDetailsResourceErrorCode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsAmountDetailsResourceErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
