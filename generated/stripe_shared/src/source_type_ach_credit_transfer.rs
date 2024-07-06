#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAchCreditTransfer {
    pub account_number: Option<String>,
    pub bank_name: Option<String>,
    pub fingerprint: Option<String>,
    pub refund_account_holder_name: Option<String>,
    pub refund_account_holder_type: Option<String>,
    pub refund_routing_number: Option<String>,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeAchCreditTransferBuilder {
    account_number: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    refund_account_holder_name: Option<Option<String>>,
    refund_account_holder_type: Option<Option<String>>,
    refund_routing_number: Option<Option<String>>,
    routing_number: Option<Option<String>>,
    swift_code: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAchCreditTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAchCreditTransfer>,
        builder: SourceTypeAchCreditTransferBuilder,
    }

    impl Visitor for Place<SourceTypeAchCreditTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeAchCreditTransferBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeAchCreditTransferBuilder {
        type Out = SourceTypeAchCreditTransfer;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_number" => Deserialize::begin(&mut self.account_number),
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "refund_account_holder_name" => {
                    Deserialize::begin(&mut self.refund_account_holder_name)
                }
                "refund_account_holder_type" => {
                    Deserialize::begin(&mut self.refund_account_holder_type)
                }
                "refund_routing_number" => Deserialize::begin(&mut self.refund_routing_number),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                "swift_code" => Deserialize::begin(&mut self.swift_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_number: Deserialize::default(),
                bank_name: Deserialize::default(),
                fingerprint: Deserialize::default(),
                refund_account_holder_name: Deserialize::default(),
                refund_account_holder_type: Deserialize::default(),
                refund_routing_number: Deserialize::default(),
                routing_number: Deserialize::default(),
                swift_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account_number: self.account_number.take()?,
                bank_name: self.bank_name.take()?,
                fingerprint: self.fingerprint.take()?,
                refund_account_holder_name: self.refund_account_holder_name.take()?,
                refund_account_holder_type: self.refund_account_holder_type.take()?,
                refund_routing_number: self.refund_routing_number.take()?,
                routing_number: self.routing_number.take()?,
                swift_code: self.swift_code.take()?,
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

    impl ObjectDeser for SourceTypeAchCreditTransfer {
        type Builder = SourceTypeAchCreditTransferBuilder;
    }

    impl FromValueOpt for SourceTypeAchCreditTransfer {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeAchCreditTransferBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_number" => b.account_number = Some(FromValueOpt::from_value(v)?),
                    "bank_name" => b.bank_name = Some(FromValueOpt::from_value(v)?),
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "refund_account_holder_name" => {
                        b.refund_account_holder_name = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_account_holder_type" => {
                        b.refund_account_holder_type = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_routing_number" => {
                        b.refund_routing_number = Some(FromValueOpt::from_value(v)?)
                    }
                    "routing_number" => b.routing_number = Some(FromValueOpt::from_value(v)?),
                    "swift_code" => b.swift_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
