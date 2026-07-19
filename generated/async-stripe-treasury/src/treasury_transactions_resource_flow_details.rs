#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionsResourceFlowDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryTransactionsResourceFlowDetails").finish_non_exhaustive()
    }
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
                builder: TreasuryTransactionsResourceFlowDetailsBuilder {
                    credit_reversal: Deserialize::default(),
                    debit_reversal: Deserialize::default(),
                    inbound_transfer: Deserialize::default(),
                    issuing_authorization: Deserialize::default(),
                    outbound_payment: Deserialize::default(),
                    outbound_transfer: Deserialize::default(),
                    received_credit: Deserialize::default(),
                    received_debit: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credit_reversal" => Deserialize::begin(&mut self.builder.credit_reversal),
                "debit_reversal" => Deserialize::begin(&mut self.builder.debit_reversal),
                "inbound_transfer" => Deserialize::begin(&mut self.builder.inbound_transfer),
                "issuing_authorization" => {
                    Deserialize::begin(&mut self.builder.issuing_authorization)
                }
                "outbound_payment" => Deserialize::begin(&mut self.builder.outbound_payment),
                "outbound_transfer" => Deserialize::begin(&mut self.builder.outbound_transfer),
                "received_credit" => Deserialize::begin(&mut self.builder.received_credit),
                "received_debit" => Deserialize::begin(&mut self.builder.received_debit),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(credit_reversal),
                Some(debit_reversal),
                Some(inbound_transfer),
                Some(issuing_authorization),
                Some(outbound_payment),
                Some(outbound_transfer),
                Some(received_credit),
                Some(received_debit),
                Some(type_),
            ) = (
                self.builder.credit_reversal.take(),
                self.builder.debit_reversal.take(),
                self.builder.inbound_transfer.take(),
                self.builder.issuing_authorization.take(),
                self.builder.outbound_payment.take(),
                self.builder.outbound_transfer.take(),
                self.builder.received_credit.take(),
                self.builder.received_debit.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryTransactionsResourceFlowDetails {
                credit_reversal,
                debit_reversal,
                inbound_transfer,
                issuing_authorization,
                outbound_payment,
                outbound_transfer,
                received_credit,
                received_debit,
                type_,
            });
            Ok(())
        }
    }
};
/// Type of the flow that created the Transaction. Set to the same value as `flow_type`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TreasuryTransactionsResourceFlowDetailsType {
    pub fn as_str(&self) -> &str {
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TreasuryTransactionsResourceFlowDetailsType {
    type Err = std::convert::Infallible;
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
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TreasuryTransactionsResourceFlowDetailsType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TreasuryTransactionsResourceFlowDetailsType))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TreasuryTransactionsResourceFlowDetailsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TreasuryTransactionsResourceFlowDetailsType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TreasuryTransactionsResourceFlowDetailsType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TreasuryTransactionsResourceFlowDetailsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
