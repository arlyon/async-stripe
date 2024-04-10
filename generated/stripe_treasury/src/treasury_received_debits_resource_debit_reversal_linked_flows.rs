#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
    /// Set if there is an Issuing dispute associated with the DebitReversal.
    pub issuing_dispute: Option<String>,
}
#[doc(hidden)]
pub struct TreasuryReceivedDebitsResourceDebitReversalLinkedFlowsBuilder {
    issuing_dispute: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,
        builder: TreasuryReceivedDebitsResourceDebitReversalLinkedFlowsBuilder,
    }

    impl Visitor for Place<TreasuryReceivedDebitsResourceDebitReversalLinkedFlows> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TreasuryReceivedDebitsResourceDebitReversalLinkedFlowsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryReceivedDebitsResourceDebitReversalLinkedFlowsBuilder {
        type Out = TreasuryReceivedDebitsResourceDebitReversalLinkedFlows;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "issuing_dispute" => Deserialize::begin(&mut self.issuing_dispute),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { issuing_dispute: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { issuing_dispute: self.issuing_dispute.take()? })
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

    impl ObjectDeser for TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
        type Builder = TreasuryReceivedDebitsResourceDebitReversalLinkedFlowsBuilder;
    }

    impl FromValueOpt for TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TreasuryReceivedDebitsResourceDebitReversalLinkedFlowsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "issuing_dispute" => b.issuing_dispute = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
