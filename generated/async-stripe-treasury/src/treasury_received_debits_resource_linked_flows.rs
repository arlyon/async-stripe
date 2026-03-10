#[derive(Clone, Debug, Eq, PartialEq)]
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
                builder: TreasuryReceivedDebitsResourceLinkedFlowsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedDebitsResourceLinkedFlowsBuilder {
        type Out = TreasuryReceivedDebitsResourceLinkedFlows;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "debit_reversal" => Deserialize::begin(&mut self.debit_reversal),
                "inbound_transfer" => Deserialize::begin(&mut self.inbound_transfer),
                "issuing_authorization" => Deserialize::begin(&mut self.issuing_authorization),
                "issuing_transaction" => Deserialize::begin(&mut self.issuing_transaction),
                "payout" => Deserialize::begin(&mut self.payout),
                "topup" => Deserialize::begin(&mut self.topup),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                debit_reversal: Deserialize::default(),
                inbound_transfer: Deserialize::default(),
                issuing_authorization: Deserialize::default(),
                issuing_transaction: Deserialize::default(),
                payout: Deserialize::default(),
                topup: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(debit_reversal),
                Some(inbound_transfer),
                Some(issuing_authorization),
                Some(issuing_transaction),
                Some(payout),
                Some(topup),
            ) = (
                self.debit_reversal.take(),
                self.inbound_transfer.take(),
                self.issuing_authorization.take(),
                self.issuing_transaction.take(),
                self.payout.take(),
                self.topup.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                debit_reversal,
                inbound_transfer,
                issuing_authorization,
                issuing_transaction,
                payout,
                topup,
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

    impl ObjectDeser for TreasuryReceivedDebitsResourceLinkedFlows {
        type Builder = TreasuryReceivedDebitsResourceLinkedFlowsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedDebitsResourceLinkedFlows {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryReceivedDebitsResourceLinkedFlowsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "debit_reversal" => b.debit_reversal = FromValueOpt::from_value(v),
                    "inbound_transfer" => b.inbound_transfer = FromValueOpt::from_value(v),
                    "issuing_authorization" => {
                        b.issuing_authorization = FromValueOpt::from_value(v)
                    }
                    "issuing_transaction" => b.issuing_transaction = FromValueOpt::from_value(v),
                    "payout" => b.payout = FromValueOpt::from_value(v),
                    "topup" => b.topup = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
