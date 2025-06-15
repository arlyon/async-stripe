#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {
    pub ach: Option<stripe_treasury::TreasuryOutboundPaymentsResourceAchTrackingDetails>,
    /// The US bank account network used to send funds.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType,
    pub us_domestic_wire:
        Option<stripe_treasury::TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails>,
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsBuilder {
    ach: Option<Option<stripe_treasury::TreasuryOutboundPaymentsResourceAchTrackingDetails>>,
    type_: Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType>,
    us_domestic_wire: Option<
        Option<stripe_treasury::TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails>,
    >,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails>,
        builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsBuilder {
        type Out = TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach" => Deserialize::begin(&mut self.ach),
                "type" => Deserialize::begin(&mut self.type_),
                "us_domestic_wire" => Deserialize::begin(&mut self.us_domestic_wire),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ach: Deserialize::default(),
                type_: Deserialize::default(),
                us_domestic_wire: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(ach), Some(type_), Some(us_domestic_wire)) =
                (self.ach.take(), self.type_, self.us_domestic_wire.take())
            else {
                return None;
            };
            Some(Self::Out { ach, type_, us_domestic_wire })
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

    impl ObjectDeser for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {
        type Builder =
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ach" => b.ach = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "us_domestic_wire" => b.us_domestic_wire = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The US bank account network used to send funds.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    Ach,
    UsDomesticWire,
}
impl TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType"))
    }
}
