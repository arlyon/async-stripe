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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: SourceReceiverFlowBuilder {
                    address: Deserialize::default(),
                    amount_charged: Deserialize::default(),
                    amount_received: Deserialize::default(),
                    amount_returned: Deserialize::default(),
                    refund_attributes_method: Deserialize::default(),
                    refund_attributes_status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.builder.address),
                "amount_charged" => Deserialize::begin(&mut self.builder.amount_charged),
                "amount_received" => Deserialize::begin(&mut self.builder.amount_received),
                "amount_returned" => Deserialize::begin(&mut self.builder.amount_returned),
                "refund_attributes_method" => {
                    Deserialize::begin(&mut self.builder.refund_attributes_method)
                }
                "refund_attributes_status" => {
                    Deserialize::begin(&mut self.builder.refund_attributes_status)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(address),
                Some(amount_charged),
                Some(amount_received),
                Some(amount_returned),
                Some(refund_attributes_method),
                Some(refund_attributes_status),
            ) = (
                self.builder.address.take(),
                self.builder.amount_charged,
                self.builder.amount_received,
                self.builder.amount_returned,
                self.builder.refund_attributes_method.take(),
                self.builder.refund_attributes_status.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceReceiverFlow {
                address,
                amount_charged,
                amount_received,
                amount_returned,
                refund_attributes_method,
                refund_attributes_status,
            });
            Ok(())
        }
    }
};
