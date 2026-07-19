#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder:
                    TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsBuilder {
                        ach: Deserialize::default(),
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
                Some(TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {
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
pub enum TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    Ach,
    UsDomesticWire,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType {
    pub fn as_str(&self) -> &str {
        use TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::*;
        match self {
            Ach => "ach",
            UsDomesticWire => "us_domestic_wire",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::*;
        match s {
            "ach" => Ok(Ach),
            "us_domestic_wire" => Ok(UsDomesticWire),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
        ))
        .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetailsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
