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
    pub type_: stripe_shared::TreasuryTransactionFlowType,
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
    type_: Option<stripe_shared::TreasuryTransactionFlowType>,
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
                credit_reversal: Some(None),
                debit_reversal: Some(None),
                inbound_transfer: Some(None),
                issuing_authorization: Some(None),
                outbound_payment: Some(None),
                outbound_transfer: Some(None),
                received_credit: Some(None),
                received_debit: Some(None),
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.credit_reversal.take(),
                self.debit_reversal.take(),
                self.inbound_transfer.take(),
                self.issuing_authorization.take(),
                self.outbound_payment.take(),
                self.outbound_transfer.take(),
                self.received_credit.take(),
                self.received_debit.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                credit_reversal,
                debit_reversal,
                inbound_transfer,
                issuing_authorization,
                outbound_payment,
                outbound_transfer,
                received_credit,
                received_debit,
                type_,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
                    "credit_reversal" => b.credit_reversal = FromValueOpt::from_value(v),
                    "debit_reversal" => b.debit_reversal = FromValueOpt::from_value(v),
                    "inbound_transfer" => b.inbound_transfer = FromValueOpt::from_value(v),
                    "issuing_authorization" => {
                        b.issuing_authorization = FromValueOpt::from_value(v)
                    }
                    "outbound_payment" => b.outbound_payment = FromValueOpt::from_value(v),
                    "outbound_transfer" => b.outbound_transfer = FromValueOpt::from_value(v),
                    "received_credit" => b.received_credit = FromValueOpt::from_value(v),
                    "received_debit" => b.received_debit = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of the flow that created the Transaction. Set to the same value as `flow_type`.
pub type TreasuryTransactionsResourceFlowDetailsType = stripe_shared::TreasuryTransactionFlowType;
