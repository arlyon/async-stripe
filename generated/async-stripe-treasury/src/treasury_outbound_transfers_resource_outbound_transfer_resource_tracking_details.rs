#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
            builder: TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsBuilder { ach: Deserialize::default(),
type_: Deserialize::default(),
us_domestic_wire: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach" => Deserialize::begin(&mut self.builder.ach),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "us_domestic_wire" => Deserialize::begin(&mut self.builder.us_domestic_wire),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(ach), Some(type_), Some(us_domestic_wire)) = (
                self.builder.ach.take(),
                self.builder.type_.take(),
                self.builder.us_domestic_wire.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails {
                    ach,
                    type_,
                    us_domestic_wire,
                });
            Ok(())
        }
    }
};
/// The US bank account network used to send funds.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType {
    Ach,
    UsDomesticWire,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType {
    pub fn as_str(&self) -> &str {
        use TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
        ))
        .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType::from_str(
                s,
            )
            .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
