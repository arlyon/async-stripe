#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails {
    pub ach: Option<stripe_treasury::TreasuryOutboundTransfersResourceAchTrackingDetails>,
    /// The US bank account network used to send funds.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType,
    pub us_domestic_wire:
        Option<stripe_treasury::TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails>,
}
#[doc(hidden)]
pub struct TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsBuilder {
    ach: Option<Option<stripe_treasury::TreasuryOutboundTransfersResourceAchTrackingDetails>>,
    type_: Option<TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType>,
    us_domestic_wire: Option<
        Option<stripe_treasury::TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails,
        >,
        builder: TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsBuilder
    {
        type Out = TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails;
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

    impl ObjectDeser for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails {
        type Builder =
            TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsBuilder::deser_default();
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
pub enum TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType {
    Ach,
    UsDomesticWire,
}
impl TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl std::str::FromStr
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType::from_str(
                s,
            )
            .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType"))
    }
}
