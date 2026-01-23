/// Details of an rejected card outcome attached to this payment evaluation.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InsightsResourcesPaymentEvaluationRejectedCard {
    /// Result of the address line 1 check.
    pub address_line1_check: InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check,
    /// Indicates whether the cardholder provided a postal code and if it matched the cardholder’s billing address.
    pub address_postal_code_check:
        InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck,
    /// Result of the CVC check.
    pub cvc_check: InsightsResourcesPaymentEvaluationRejectedCardCvcCheck,
    /// Card issuer's reason for the network decline.
    pub reason: InsightsResourcesPaymentEvaluationRejectedCardReason,
}
#[doc(hidden)]
pub struct InsightsResourcesPaymentEvaluationRejectedCardBuilder {
    address_line1_check: Option<InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check>,
    address_postal_code_check:
        Option<InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck>,
    cvc_check: Option<InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
    reason: Option<InsightsResourcesPaymentEvaluationRejectedCardReason>,
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

    impl Deserialize for InsightsResourcesPaymentEvaluationRejectedCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InsightsResourcesPaymentEvaluationRejectedCard>,
        builder: InsightsResourcesPaymentEvaluationRejectedCardBuilder,
    }

    impl Visitor for Place<InsightsResourcesPaymentEvaluationRejectedCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InsightsResourcesPaymentEvaluationRejectedCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InsightsResourcesPaymentEvaluationRejectedCardBuilder {
        type Out = InsightsResourcesPaymentEvaluationRejectedCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_postal_code_check" => {
                    Deserialize::begin(&mut self.address_postal_code_check)
                }
                "cvc_check" => Deserialize::begin(&mut self.cvc_check),
                "reason" => Deserialize::begin(&mut self.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address_line1_check: Deserialize::default(),
                address_postal_code_check: Deserialize::default(),
                cvc_check: Deserialize::default(),
                reason: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(address_line1_check),
                Some(address_postal_code_check),
                Some(cvc_check),
                Some(reason),
            ) = (
                self.address_line1_check.take(),
                self.address_postal_code_check.take(),
                self.cvc_check.take(),
                self.reason.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { address_line1_check, address_postal_code_check, cvc_check, reason })
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

    impl ObjectDeser for InsightsResourcesPaymentEvaluationRejectedCard {
        type Builder = InsightsResourcesPaymentEvaluationRejectedCardBuilder;
    }

    impl FromValueOpt for InsightsResourcesPaymentEvaluationRejectedCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InsightsResourcesPaymentEvaluationRejectedCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address_line1_check" => b.address_line1_check = FromValueOpt::from_value(v),
                    "address_postal_code_check" => {
                        b.address_postal_code_check = FromValueOpt::from_value(v)
                    }
                    "cvc_check" => b.cvc_check = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Result of the address line 1 check.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    Fail,
    Pass,
    Unavailable,
    Unchecked,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::*;
        match self {
            Fail => "fail",
            Pass => "pass",
            Unavailable => "unavailable",
            Unchecked => "unchecked",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::*;
        match s {
            "fail" => Ok(Fail),
            "pass" => Ok(Pass),
            "unavailable" => Ok(Unavailable),
            "unchecked" => Ok(Unchecked),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Indicates whether the cardholder provided a postal code and if it matched the cardholder’s billing address.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck {
    Fail,
    Pass,
    Unavailable,
    Unchecked,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck::*;
        match self {
            Fail => "fail",
            Pass => "pass",
            Unavailable => "unavailable",
            Unchecked => "unchecked",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck::*;
        match s {
            "fail" => Ok(Fail),
            "pass" => Ok(Pass),
            "unavailable" => Ok(Unavailable),
            "unchecked" => Ok(Unchecked),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for InsightsResourcesPaymentEvaluationRejectedCardAddressPostalCodeCheck
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Result of the CVC check.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    Fail,
    Pass,
    Unavailable,
    Unchecked,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationRejectedCardCvcCheck::*;
        match self {
            Fail => "fail",
            Pass => "pass",
            Unavailable => "unavailable",
            Unchecked => "unchecked",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationRejectedCardCvcCheck::*;
        match s {
            "fail" => Ok(Fail),
            "pass" => Ok(Pass),
            "unavailable" => Ok(Unavailable),
            "unchecked" => Ok(Unchecked),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationRejectedCardCvcCheck"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationRejectedCardCvcCheck::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InsightsResourcesPaymentEvaluationRejectedCardCvcCheck);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationRejectedCardCvcCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Card issuer's reason for the network decline.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InsightsResourcesPaymentEvaluationRejectedCardReason {
    AuthenticationFailed,
    DoNotHonor,
    Expired,
    IncorrectCvc,
    IncorrectNumber,
    IncorrectPostalCode,
    InsufficientFunds,
    InvalidAccount,
    LostCard,
    Other,
    ProcessingError,
    ReportedStolen,
    TryAgainLater,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InsightsResourcesPaymentEvaluationRejectedCardReason {
    pub fn as_str(&self) -> &str {
        use InsightsResourcesPaymentEvaluationRejectedCardReason::*;
        match self {
            AuthenticationFailed => "authentication_failed",
            DoNotHonor => "do_not_honor",
            Expired => "expired",
            IncorrectCvc => "incorrect_cvc",
            IncorrectNumber => "incorrect_number",
            IncorrectPostalCode => "incorrect_postal_code",
            InsufficientFunds => "insufficient_funds",
            InvalidAccount => "invalid_account",
            LostCard => "lost_card",
            Other => "other",
            ProcessingError => "processing_error",
            ReportedStolen => "reported_stolen",
            TryAgainLater => "try_again_later",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InsightsResourcesPaymentEvaluationRejectedCardReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InsightsResourcesPaymentEvaluationRejectedCardReason::*;
        match s {
            "authentication_failed" => Ok(AuthenticationFailed),
            "do_not_honor" => Ok(DoNotHonor),
            "expired" => Ok(Expired),
            "incorrect_cvc" => Ok(IncorrectCvc),
            "incorrect_number" => Ok(IncorrectNumber),
            "incorrect_postal_code" => Ok(IncorrectPostalCode),
            "insufficient_funds" => Ok(InsufficientFunds),
            "invalid_account" => Ok(InvalidAccount),
            "lost_card" => Ok(LostCard),
            "other" => Ok(Other),
            "processing_error" => Ok(ProcessingError),
            "reported_stolen" => Ok(ReportedStolen),
            "try_again_later" => Ok(TryAgainLater),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InsightsResourcesPaymentEvaluationRejectedCardReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InsightsResourcesPaymentEvaluationRejectedCardReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InsightsResourcesPaymentEvaluationRejectedCardReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InsightsResourcesPaymentEvaluationRejectedCardReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InsightsResourcesPaymentEvaluationRejectedCardReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InsightsResourcesPaymentEvaluationRejectedCardReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InsightsResourcesPaymentEvaluationRejectedCardReason::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InsightsResourcesPaymentEvaluationRejectedCardReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InsightsResourcesPaymentEvaluationRejectedCardReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
