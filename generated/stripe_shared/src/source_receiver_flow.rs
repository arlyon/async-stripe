#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct SourceReceiverFlowBuilder {
    address: Option<Option<String>>,
    amount_charged: Option<i64>,
    amount_received: Option<i64>,
    amount_returned: Option<i64>,
    refund_attributes_method: Option<String>,
    refund_attributes_status: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                address: Deserialize::default(),
                amount_charged: Deserialize::default(),
                amount_received: Deserialize::default(),
                amount_returned: Deserialize::default(),
                refund_attributes_method: Deserialize::default(),
                refund_attributes_status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                address: self.address.take()?,
                amount_charged: self.amount_charged?,
                amount_received: self.amount_received?,
                amount_returned: self.amount_returned?,
                refund_attributes_method: self.refund_attributes_method.take()?,
                refund_attributes_status: self.refund_attributes_status.take()?,
            })
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

    impl ObjectDeser for SourceReceiverFlow {
        type Builder = SourceReceiverFlowBuilder;
    }

    impl FromValueOpt for SourceReceiverFlow {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceReceiverFlowBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = Some(FromValueOpt::from_value(v)?),
                    "amount_charged" => b.amount_charged = Some(FromValueOpt::from_value(v)?),
                    "amount_received" => b.amount_received = Some(FromValueOpt::from_value(v)?),
                    "amount_returned" => b.amount_returned = Some(FromValueOpt::from_value(v)?),
                    "refund_attributes_method" => {
                        b.refund_attributes_method = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_attributes_status" => {
                        b.refund_attributes_status = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
