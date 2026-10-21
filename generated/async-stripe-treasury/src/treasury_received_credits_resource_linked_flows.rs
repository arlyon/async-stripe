#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {
    /// The CreditReversal created as a result of this ReceivedCredit being reversed.
    pub credit_reversal: Option<String>,
    /// Set if the ReceivedCredit was created due to an [Issuing Authorization](https://api.stripe.com#issuing_authorizations) object.
    pub issuing_authorization: Option<String>,
    /// Set if the ReceivedCredit is also viewable as an [Issuing transaction](https://api.stripe.com#issuing_transactions) object.
    pub issuing_transaction: Option<String>,
    /// ID of the source flow.
    /// Set if `network` is `stripe` and the source flow is visible to the user.
    /// Examples of source flows include OutboundPayments, payouts, or CreditReversals.
    pub source_flow: Option<String>,
    /// The expandable object of the source flow.
    pub source_flow_details:
        Option<stripe_treasury::TreasuryReceivedCreditsResourceSourceFlowsDetails>,
    /// The type of flow that originated the ReceivedCredit (for example, `outbound_payment`).
    pub source_flow_type: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryReceivedCreditsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryReceivedCreditsResourceLinkedFlows").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryReceivedCreditsResourceLinkedFlowsBuilder {
    credit_reversal: Option<Option<String>>,
    issuing_authorization: Option<Option<String>>,
    issuing_transaction: Option<Option<String>>,
    source_flow: Option<Option<String>>,
    source_flow_details: Option<stripe_treasury::TreasuryReceivedCreditsResourceSourceFlowsDetails>,
    source_flow_type: Option<Option<String>>,
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

    impl Deserialize for TreasuryReceivedCreditsResourceLinkedFlows {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedCreditsResourceLinkedFlows>,
        builder: TreasuryReceivedCreditsResourceLinkedFlowsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedCreditsResourceLinkedFlows> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedCreditsResourceLinkedFlowsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedCreditsResourceLinkedFlowsBuilder {
        type Out = TreasuryReceivedCreditsResourceLinkedFlows;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credit_reversal" => Deserialize::begin(&mut self.credit_reversal),
                "issuing_authorization" => Deserialize::begin(&mut self.issuing_authorization),
                "issuing_transaction" => Deserialize::begin(&mut self.issuing_transaction),
                "source_flow" => Deserialize::begin(&mut self.source_flow),
                "source_flow_details" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.source_flow_details)
                }
                "source_flow_type" => Deserialize::begin(&mut self.source_flow_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                credit_reversal: Some(None),
                issuing_authorization: Some(None),
                issuing_transaction: Some(None),
                source_flow: Some(None),
                source_flow_details: None,
                source_flow_type: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    credit_reversal: Some(credit_reversal),
                    issuing_authorization: Some(issuing_authorization),
                    issuing_transaction: Some(issuing_transaction),
                    source_flow: Some(source_flow),
                    source_flow_details,
                    source_flow_type: Some(source_flow_type),
                } => Some(Self::Out {
                    credit_reversal,
                    issuing_authorization,
                    issuing_transaction,
                    source_flow,
                    source_flow_details,
                    source_flow_type,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for TreasuryReceivedCreditsResourceLinkedFlows {
        type Builder = TreasuryReceivedCreditsResourceLinkedFlowsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedCreditsResourceLinkedFlows {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
