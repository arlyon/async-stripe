#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTransactionAchCreditTransferData {
    /// Customer data associated with the transfer.
    pub customer_data: Option<String>,
    /// Bank account fingerprint associated with the transfer.
    pub fingerprint: Option<String>,
    /// Last 4 digits of the account number associated with the transfer.
    pub last4: Option<String>,
    /// Routing number associated with the transfer.
    pub routing_number: Option<String>,
}
#[doc(hidden)]
pub struct SourceTransactionAchCreditTransferDataBuilder {
    customer_data: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    routing_number: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionAchCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionAchCreditTransferData>,
        builder: SourceTransactionAchCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionAchCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTransactionAchCreditTransferDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTransactionAchCreditTransferDataBuilder {
        type Out = SourceTransactionAchCreditTransferData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_data" => Deserialize::begin(&mut self.customer_data),
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "routing_number" => Deserialize::begin(&mut self.routing_number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                customer_data: Deserialize::default(),
                fingerprint: Deserialize::default(),
                last4: Deserialize::default(),
                routing_number: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                customer_data: self.customer_data.take()?,
                fingerprint: self.fingerprint.take()?,
                last4: self.last4.take()?,
                routing_number: self.routing_number.take()?,
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

    impl ObjectDeser for SourceTransactionAchCreditTransferData {
        type Builder = SourceTransactionAchCreditTransferDataBuilder;
    }

    impl FromValueOpt for SourceTransactionAchCreditTransferData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTransactionAchCreditTransferDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_data" => b.customer_data = Some(FromValueOpt::from_value(v)?),
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "routing_number" => b.routing_number = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
