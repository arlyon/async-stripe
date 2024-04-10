#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryTransactionsResourceFlowDetails {
    pub credit_reversal: Option<stripe_treasury::TreasuryCreditReversal>,
    pub debit_reversal: Option<stripe_treasury::TreasuryDebitReversal>,
    pub inbound_transfer: Option<stripe_treasury::TreasuryInboundTransfer>,
    pub issuing_authorization: Option<stripe_shared::IssuingAuthorization>,
    pub outbound_payment: Option<stripe_treasury::TreasuryOutboundPayment>,
    pub outbound_transfer: Option<stripe_treasury::TreasuryOutboundTransfer>,
    pub received_credit: Option<stripe_treasury::TreasuryReceivedCredit>,
    pub received_debit: Option<stripe_treasury::TreasuryReceivedDebit>,
    /// Type of the flow that created the Transaction. Set to the same value as `flow_type`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TreasuryTransactionsResourceFlowDetailsType,
}
#[doc(hidden)]
pub struct TreasuryTransactionsResourceFlowDetailsBuilder {
    credit_reversal: Option<Option<stripe_treasury::TreasuryCreditReversal>>,
    debit_reversal: Option<Option<stripe_treasury::TreasuryDebitReversal>>,
    inbound_transfer: Option<Option<stripe_treasury::TreasuryInboundTransfer>>,
    issuing_authorization: Option<Option<stripe_shared::IssuingAuthorization>>,
    outbound_payment: Option<Option<stripe_treasury::TreasuryOutboundPayment>>,
    outbound_transfer: Option<Option<stripe_treasury::TreasuryOutboundTransfer>>,
    received_credit: Option<Option<stripe_treasury::TreasuryReceivedCredit>>,
    received_debit: Option<Option<stripe_treasury::TreasuryReceivedDebit>>,
    type_: Option<TreasuryTransactionsResourceFlowDetailsType>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryTransactionsResourceFlowDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransactionsResourceFlowDetails>,
        builder: TreasuryTransactionsResourceFlowDetailsBuilder,
    }

    impl Visitor for Place<TreasuryTransactionsResourceFlowDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryTransactionsResourceFlowDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryTransactionsResourceFlowDetailsBuilder {
        type Out = TreasuryTransactionsResourceFlowDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credit_reversal" => Deserialize::begin(&mut self.credit_reversal),
                "debit_reversal" => Deserialize::begin(&mut self.debit_reversal),
                "inbound_transfer" => Deserialize::begin(&mut self.inbound_transfer),
                "issuing_authorization" => Deserialize::begin(&mut self.issuing_authorization),
                "outbound_payment" => Deserialize::begin(&mut self.outbound_payment),
                "outbound_transfer" => Deserialize::begin(&mut self.outbound_transfer),
                "received_credit" => Deserialize::begin(&mut self.received_credit),
                "received_debit" => Deserialize::begin(&mut self.received_debit),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                credit_reversal: Deserialize::default(),
                debit_reversal: Deserialize::default(),
                inbound_transfer: Deserialize::default(),
                issuing_authorization: Deserialize::default(),
                outbound_payment: Deserialize::default(),
                outbound_transfer: Deserialize::default(),
                received_credit: Deserialize::default(),
                received_debit: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                credit_reversal: self.credit_reversal.take()?,
                debit_reversal: self.debit_reversal.take()?,
                inbound_transfer: self.inbound_transfer.take()?,
                issuing_authorization: self.issuing_authorization.take()?,
                outbound_payment: self.outbound_payment.take()?,
                outbound_transfer: self.outbound_transfer.take()?,
                received_credit: self.received_credit.take()?,
                received_debit: self.received_debit.take()?,
                type_: self.type_?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for TreasuryTransactionsResourceFlowDetails {
        type Builder = TreasuryTransactionsResourceFlowDetailsBuilder;
    }

    impl FromValueOpt for TreasuryTransactionsResourceFlowDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryTransactionsResourceFlowDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "credit_reversal" => b.credit_reversal = Some(FromValueOpt::from_value(v)?),
                    "debit_reversal" => b.debit_reversal = Some(FromValueOpt::from_value(v)?),
                    "inbound_transfer" => b.inbound_transfer = Some(FromValueOpt::from_value(v)?),
                    "issuing_authorization" => {
                        b.issuing_authorization = Some(FromValueOpt::from_value(v)?)
                    }
                    "outbound_payment" => b.outbound_payment = Some(FromValueOpt::from_value(v)?),
                    "outbound_transfer" => b.outbound_transfer = Some(FromValueOpt::from_value(v)?),
                    "received_credit" => b.received_credit = Some(FromValueOpt::from_value(v)?),
                    "received_debit" => b.received_debit = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of the flow that created the Transaction. Set to the same value as `flow_type`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TreasuryTransactionsResourceFlowDetailsType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}
impl TreasuryTransactionsResourceFlowDetailsType {
    pub fn as_str(self) -> &'static str {
        use TreasuryTransactionsResourceFlowDetailsType::*;
        match self {
            CreditReversal => "credit_reversal",
            DebitReversal => "debit_reversal",
            InboundTransfer => "inbound_transfer",
            IssuingAuthorization => "issuing_authorization",
            Other => "other",
            OutboundPayment => "outbound_payment",
            OutboundTransfer => "outbound_transfer",
            ReceivedCredit => "received_credit",
            ReceivedDebit => "received_debit",
        }
    }
}

impl std::str::FromStr for TreasuryTransactionsResourceFlowDetailsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TreasuryTransactionsResourceFlowDetailsType::*;
        match s {
            "credit_reversal" => Ok(CreditReversal),
            "debit_reversal" => Ok(DebitReversal),
            "inbound_transfer" => Ok(InboundTransfer),
            "issuing_authorization" => Ok(IssuingAuthorization),
            "other" => Ok(Other),
            "outbound_payment" => Ok(OutboundPayment),
            "outbound_transfer" => Ok(OutboundTransfer),
            "received_credit" => Ok(ReceivedCredit),
            "received_debit" => Ok(ReceivedDebit),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TreasuryTransactionsResourceFlowDetailsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TreasuryTransactionsResourceFlowDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TreasuryTransactionsResourceFlowDetailsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TreasuryTransactionsResourceFlowDetailsType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TreasuryTransactionsResourceFlowDetailsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionsResourceFlowDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TreasuryTransactionsResourceFlowDetailsType",
            )
        })
    }
}
