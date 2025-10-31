#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks {
pub address_line1_check: Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check>,
pub address_postal_code_check: Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck>,
pub cvc_check: Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck>,

}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder {
    address_line1_check: Option<Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check>>,
address_postal_code_check: Option<Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck>>,
cvc_check: Option<Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck>>,

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

    impl Deserialize
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks,
        >,
        builder:
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder,
    }

    impl Visitor
        for Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder
    {
        type Out = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => Deserialize::begin(&mut self.address_line1_check),
                "address_postal_code_check" => {
                    Deserialize::begin(&mut self.address_postal_code_check)
                }
                "cvc_check" => Deserialize::begin(&mut self.cvc_check),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address_line1_check: Deserialize::default(),
                address_postal_code_check: Deserialize::default(),
                cvc_check: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address_line1_check), Some(address_postal_code_check), Some(cvc_check)) =
                (self.address_line1_check, self.address_postal_code_check, self.cvc_check)
            else {
                return None;
            };
            Some(Self::Out { address_line1_check, address_postal_code_check, cvc_check })
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

    impl ObjectDeser
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks
    {
        type Builder =
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder;
    }

    impl FromValueOpt
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks
    {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address_line1_check" => b.address_line1_check = FromValueOpt::from_value(v),
                    "address_postal_code_check" => {
                        b.address_postal_code_check = FromValueOpt::from_value(v)
                    }
                    "cvc_check" => b.cvc_check = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check
{
    Fail,
    Pass,
    Unavailable,
    Unchecked,
}
impl
    PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check
{
    pub fn as_str(self) -> &'static str {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check::*;
        match self {
            Fail => "fail",
            Pass => "pass",
            Unavailable => "unavailable",
            Unchecked => "unchecked",
        }
    }
}

impl std::str::FromStr for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check::*;
        match s {
    "fail" => Ok(Fail),
"pass" => Ok(Pass),
"unavailable" => Ok(Unavailable),
"unchecked" => Ok(Unchecked),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck
{
    Fail,
    Pass,
    Unavailable,
    Unchecked,
}
impl PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck {
    pub fn as_str(self) -> &'static str {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck::*;
        match self {
Fail => "fail",
Pass => "pass",
Unavailable => "unavailable",
Unchecked => "unchecked",

        }
    }
}

impl std::str::FromStr for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck::*;
        match s {
    "fail" => Ok(Fail),
"pass" => Ok(Pass),
"unavailable" => Ok(Unavailable),
"unchecked" => Ok(Unchecked),
_ => Err(stripe_types::StripeParseError)

        }
    }
}
impl std::fmt::Display for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck {
    Fail,
    Pass,
    Unavailable,
    Unchecked,
}
impl PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck {
    pub fn as_str(self) -> &'static str {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck::*;
        match self {
            Fail => "fail",
            Pass => "pass",
            Unavailable => "unavailable",
            Unchecked => "unchecked",
        }
    }
}

impl std::str::FromStr
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck::*;
        match s {
            "fail" => Ok(Fail),
            "pass" => Ok(Pass),
            "unavailable" => Ok(Unavailable),
            "unchecked" => Ok(Unchecked),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<
        PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck,
    >
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck"))
    }
}
