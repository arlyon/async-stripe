#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedDebitsResourceLinkedFlows {
    /// The DebitReversal created as a result of this ReceivedDebit being reversed.
    pub debit_reversal: Option<String>,
    /// Set if the ReceivedDebit is associated with an InboundTransfer's return of funds.
    pub inbound_transfer: Option<String>,
    /// Set if the ReceivedDebit was created due to an [Issuing Authorization](https://api.stripe.com#issuing_authorizations) object.
    pub issuing_authorization: Option<String>,
    /// Set if the ReceivedDebit is also viewable as an [Issuing Dispute](https://api.stripe.com#issuing_disputes) object.
    pub issuing_transaction: Option<String>,
    /// Set if the ReceivedDebit was created due to a [Payout](https://api.stripe.com#payouts) object.
    pub payout: Option<String>,
    /// Set if the ReceivedDebit was created due to a [Topup](https://api.stripe.com#topups) object.
    pub topup: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryReceivedDebitsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryReceivedDebitsResourceLinkedFlows").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryReceivedDebitsResourceLinkedFlowsBuilder {
    debit_reversal: Option<Option<String>>,
    inbound_transfer: Option<Option<String>>,
    issuing_authorization: Option<Option<String>>,
    issuing_transaction: Option<Option<String>>,
    payout: Option<Option<String>>,
    topup: Option<Option<String>>,
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

    impl Deserialize for TreasuryReceivedDebitsResourceLinkedFlows {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedDebitsResourceLinkedFlows>,
        builder: TreasuryReceivedDebitsResourceLinkedFlowsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedDebitsResourceLinkedFlows> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryReceivedDebitsResourceLinkedFlowsBuilder {
                    debit_reversal: Deserialize::default(),
                    inbound_transfer: Deserialize::default(),
                    issuing_authorization: Deserialize::default(),
                    issuing_transaction: Deserialize::default(),
                    payout: Deserialize::default(),
                    topup: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "debit_reversal" => Deserialize::begin(&mut self.builder.debit_reversal),
                "inbound_transfer" => Deserialize::begin(&mut self.builder.inbound_transfer),
                "issuing_authorization" => {
                    Deserialize::begin(&mut self.builder.issuing_authorization)
                }
                "issuing_transaction" => Deserialize::begin(&mut self.builder.issuing_transaction),
                "payout" => Deserialize::begin(&mut self.builder.payout),
                "topup" => Deserialize::begin(&mut self.builder.topup),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(debit_reversal),
                Some(inbound_transfer),
                Some(issuing_authorization),
                Some(issuing_transaction),
                Some(payout),
                Some(topup),
            ) = (
                self.builder.debit_reversal.take(),
                self.builder.inbound_transfer.take(),
                self.builder.issuing_authorization.take(),
                self.builder.issuing_transaction.take(),
                self.builder.payout.take(),
                self.builder.topup.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TreasuryReceivedDebitsResourceLinkedFlows {
                debit_reversal,
                inbound_transfer,
                issuing_authorization,
                issuing_transaction,
                payout,
                topup,
            });
            Ok(())
        }
    }
};
