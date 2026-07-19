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
    source_flow_details:
        Option<Option<stripe_treasury::TreasuryReceivedCreditsResourceSourceFlowsDetails>>,
    source_flow_type: Option<Option<String>>,
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
                builder: TreasuryReceivedCreditsResourceLinkedFlowsBuilder {
                    credit_reversal: Deserialize::default(),
                    issuing_authorization: Deserialize::default(),
                    issuing_transaction: Deserialize::default(),
                    source_flow: Deserialize::default(),
                    source_flow_details: Deserialize::default(),
                    source_flow_type: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credit_reversal" => Deserialize::begin(&mut self.builder.credit_reversal),
                "issuing_authorization" => {
                    Deserialize::begin(&mut self.builder.issuing_authorization)
                }
                "issuing_transaction" => Deserialize::begin(&mut self.builder.issuing_transaction),
                "source_flow" => Deserialize::begin(&mut self.builder.source_flow),
                "source_flow_details" => Deserialize::begin(&mut self.builder.source_flow_details),
                "source_flow_type" => Deserialize::begin(&mut self.builder.source_flow_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(credit_reversal),
                Some(issuing_authorization),
                Some(issuing_transaction),
                Some(source_flow),
                Some(source_flow_details),
                Some(source_flow_type),
            ) = (
                self.builder.credit_reversal.take(),
                self.builder.issuing_authorization.take(),
                self.builder.issuing_transaction.take(),
                self.builder.source_flow.take(),
                self.builder.source_flow_details.take(),
                self.builder.source_flow_type.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryReceivedCreditsResourceLinkedFlows {
                credit_reversal,
                issuing_authorization,
                issuing_transaction,
                source_flow,
                source_flow_details,
                source_flow_type,
            });
            Ok(())
        }
    }
};
