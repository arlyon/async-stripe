#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    pub credit_reversal: Option<stripe_treasury::TreasuryCreditReversal>,
    pub outbound_payment: Option<stripe_treasury::TreasuryOutboundPayment>,
    pub outbound_transfer: Option<stripe_treasury::TreasuryOutboundTransfer>,
    pub payout: Option<stripe_shared::Payout>,
    /// The type of the source flow that originated the ReceivedCredit.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::ListTreasuryReceivedCreditLinkedFlowsSourceFlowType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryReceivedCreditsResourceSourceFlowsDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryReceivedCreditsResourceSourceFlowsDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetailsBuilder {
    credit_reversal: Option<Option<stripe_treasury::TreasuryCreditReversal>>,
    outbound_payment: Option<Option<stripe_treasury::TreasuryOutboundPayment>>,
    outbound_transfer: Option<Option<stripe_treasury::TreasuryOutboundTransfer>>,
    payout: Option<Option<stripe_shared::Payout>>,
    type_: Option<stripe_shared::ListTreasuryReceivedCreditLinkedFlowsSourceFlowType>,
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
                credit_reversal: Some(None),
                outbound_payment: Some(None),
                outbound_transfer: Some(None),
                payout: Some(None),
                type_: None,
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
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
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
pub type TreasuryReceivedCreditsResourceSourceFlowsDetailsType =
    stripe_shared::ListTreasuryReceivedCreditLinkedFlowsSourceFlowType;
