#[derive(Clone, Debug)]
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
                "source_flow_details" => Deserialize::begin(&mut self.source_flow_details),
                "source_flow_type" => Deserialize::begin(&mut self.source_flow_type),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                credit_reversal: Deserialize::default(),
                issuing_authorization: Deserialize::default(),
                issuing_transaction: Deserialize::default(),
                source_flow: Deserialize::default(),
                source_flow_details: Deserialize::default(),
                source_flow_type: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(credit_reversal),
                Some(issuing_authorization),
                Some(issuing_transaction),
                Some(source_flow),
                Some(source_flow_details),
                Some(source_flow_type),
            ) = (
                self.credit_reversal.take(),
                self.issuing_authorization.take(),
                self.issuing_transaction.take(),
                self.source_flow.take(),
                self.source_flow_details.take(),
                self.source_flow_type.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                credit_reversal,
                issuing_authorization,
                issuing_transaction,
                source_flow,
                source_flow_details,
                source_flow_type,
            })
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

    impl ObjectDeser for TreasuryReceivedCreditsResourceLinkedFlows {
        type Builder = TreasuryReceivedCreditsResourceLinkedFlowsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedCreditsResourceLinkedFlows {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryReceivedCreditsResourceLinkedFlowsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "credit_reversal" => b.credit_reversal = FromValueOpt::from_value(v),
                    "issuing_authorization" => {
                        b.issuing_authorization = FromValueOpt::from_value(v)
                    }
                    "issuing_transaction" => b.issuing_transaction = FromValueOpt::from_value(v),
                    "source_flow" => b.source_flow = FromValueOpt::from_value(v),
                    "source_flow_details" => b.source_flow_details = FromValueOpt::from_value(v),
                    "source_flow_type" => b.source_flow_type = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
