#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    pub credit_reversal: Option<stripe_treasury::TreasuryCreditReversal>,
    pub outbound_payment: Option<stripe_treasury::TreasuryOutboundPayment>,
    pub outbound_transfer: Option<stripe_treasury::TreasuryOutboundTransfer>,
    pub payout: Option<stripe_shared::Payout>,
    /// The type of the source flow that originated the ReceivedCredit.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TreasuryReceivedCreditsResourceSourceFlowsDetailsType,
}
#[doc(hidden)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder {
    credit_reversal: Option<Option<stripe_treasury::TreasuryCreditReversal>>,
    outbound_payment: Option<Option<stripe_treasury::TreasuryOutboundPayment>>,
    outbound_transfer: Option<Option<stripe_treasury::TreasuryOutboundTransfer>>,
    payout: Option<Option<stripe_shared::Payout>>,
    type_: Option<TreasuryReceivedCreditsResourceSourceFlowsDetailsType>,
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

    impl Deserialize for TreasuryReceivedCreditsResourceSourceFlowsDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceSourceFlowsDetails>,
        builder: TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceSourceFlowsDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder {
        type Out = TreasuryReceivedCreditsResourceSourceFlowsDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credit_reversal" => Deserialize::begin(&mut self.credit_reversal),
                "outbound_payment" => Deserialize::begin(&mut self.outbound_payment),
                "outbound_transfer" => Deserialize::begin(&mut self.outbound_transfer),
                "payout" => Deserialize::begin(&mut self.payout),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                credit_reversal: Deserialize::default(),
                outbound_payment: Deserialize::default(),
                outbound_transfer: Deserialize::default(),
                payout: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(credit_reversal),
                Some(outbound_payment),
                Some(outbound_transfer),
                Some(payout),
                Some(type_),
            ) = (
                self.credit_reversal.take(),
                self.outbound_payment.take(),
                self.outbound_transfer.take(),
                self.payout.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { credit_reversal, outbound_payment, outbound_transfer, payout, type_ })
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

    impl ObjectDeser for TreasuryReceivedCreditsResourceSourceFlowsDetails {
        type Builder = TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedCreditsResourceSourceFlowsDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "credit_reversal" => b.credit_reversal = FromValueOpt::from_value(v),
                    "outbound_payment" => b.outbound_payment = FromValueOpt::from_value(v),
                    "outbound_transfer" => b.outbound_transfer = FromValueOpt::from_value(v),
                    "payout" => b.payout = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the source flow that originated the ReceivedCredit.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    OutboundTransfer,
    Payout,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    pub fn as_str(&self) -> &str {
        use TreasuryReceivedCreditsResourceSourceFlowsDetailsType::*;
        match self {
            CreditReversal => "credit_reversal",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundTransfer => "outbound_transfer",
            Payout => "payout",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryReceivedCreditsResourceSourceFlowsDetailsType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_transfer" => Ok(OutboundTransfer),
            "payout" => Ok(Payout),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryReceivedCreditsResourceSourceFlowsDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TreasuryReceivedCreditsResourceSourceFlowsDetailsType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryReceivedCreditsResourceSourceFlowsDetailsType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryReceivedCreditsResourceSourceFlowsDetailsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
