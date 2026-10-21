#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceReceiverFlow {
    /// The address of the receiver source.
    /// This is the value that should be communicated to the customer to send their funds to.
    pub address: Option<String>,
    /// The total amount that was moved to your balance.
    /// This is almost always equal to the amount charged.
    /// In rare cases when customers deposit excess funds and we are unable to refund those, those funds get moved to your balance and show up in amount_charged as well.
    /// The amount charged is expressed in the source's currency.
    pub amount_charged: i64,
    /// The total amount received by the receiver source.
    /// `amount_received = amount_returned + amount_charged` should be true for consumed sources unless customers deposit excess funds.
    /// The amount received is expressed in the source's currency.
    pub amount_received: i64,
    /// The total amount that was returned to the customer.
    /// The amount returned is expressed in the source's currency.
    pub amount_returned: i64,
    /// Type of refund attribute method, one of `email`, `manual`, or `none`.
    pub refund_attributes_method: String,
    /// Type of refund attribute status, one of `missing`, `requested`, or `available`.
    pub refund_attributes_status: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceReceiverFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceReceiverFlow").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceReceiverFlowBuilder {
    address: Option<Option<String>>,
    amount_charged: Option<i64>,
    amount_received: Option<i64>,
    amount_returned: Option<i64>,
    refund_attributes_method: Option<String>,
    refund_attributes_status: Option<String>,
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

    impl Deserialize for SourceReceiverFlow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceReceiverFlow>,
        builder: SourceReceiverFlowBuilder,
    }

    impl Visitor for Place<SourceReceiverFlow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceReceiverFlowBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceReceiverFlowBuilder {
        type Out = SourceReceiverFlow;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "amount_charged" => Deserialize::begin(&mut self.amount_charged),
                "amount_received" => Deserialize::begin(&mut self.amount_received),
                "amount_returned" => Deserialize::begin(&mut self.amount_returned),
                "refund_attributes_method" => {
                    Deserialize::begin(&mut self.refund_attributes_method)
                }
                "refund_attributes_status" => {
                    Deserialize::begin(&mut self.refund_attributes_status)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: Some(None),
                amount_charged: None,
                amount_received: None,
                amount_returned: None,
                refund_attributes_method: None,
                refund_attributes_status: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    address: Some(address),
                    amount_charged: Some(amount_charged),
                    amount_received: Some(amount_received),
                    amount_returned: Some(amount_returned),
                    refund_attributes_method: Some(refund_attributes_method),
                    refund_attributes_status: Some(refund_attributes_status),
                } => Some(Self::Out {
                    address,
                    amount_charged,
                    amount_received,
                    amount_returned,
                    refund_attributes_method,
                    refund_attributes_status,
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

    impl ObjectDeser for SourceReceiverFlow {
        type Builder = SourceReceiverFlowBuilder;
    }

    impl FromValueOpt for SourceReceiverFlow {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
