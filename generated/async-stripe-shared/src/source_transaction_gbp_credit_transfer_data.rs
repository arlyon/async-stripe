#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTransactionGbpCreditTransferData {
    /// Bank account fingerprint associated with the Stripe owned bank account receiving the transfer.
    pub fingerprint: Option<String>,
    /// The credit transfer rails the sender used to push this transfer.
    /// The possible rails are: Faster Payments, BACS, CHAPS, and wire transfers.
    /// Currently only Faster Payments is supported.
    pub funding_method: Option<String>,
    /// Last 4 digits of sender account number associated with the transfer.
    pub last4: Option<String>,
    /// Sender entered arbitrary information about the transfer.
    pub reference: Option<String>,
    /// Sender account number associated with the transfer.
    pub sender_account_number: Option<String>,
    /// Sender name associated with the transfer.
    pub sender_name: Option<String>,
    /// Sender sort code associated with the transfer.
    pub sender_sort_code: Option<String>,
}
#[doc(hidden)]
pub struct SourceTransactionGbpCreditTransferDataBuilder {
    fingerprint: Option<Option<String>>,
    funding_method: Option<Option<String>>,
    last4: Option<Option<String>>,
    reference: Option<Option<String>>,
    sender_account_number: Option<Option<String>>,
    sender_name: Option<Option<String>>,
    sender_sort_code: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionGbpCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionGbpCreditTransferData>,
        builder: SourceTransactionGbpCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionGbpCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTransactionGbpCreditTransferDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTransactionGbpCreditTransferDataBuilder {
        type Out = SourceTransactionGbpCreditTransferData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "funding_method" => Deserialize::begin(&mut self.funding_method),
                "last4" => Deserialize::begin(&mut self.last4),
                "reference" => Deserialize::begin(&mut self.reference),
                "sender_account_number" => Deserialize::begin(&mut self.sender_account_number),
                "sender_name" => Deserialize::begin(&mut self.sender_name),
                "sender_sort_code" => Deserialize::begin(&mut self.sender_sort_code),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                fingerprint: Deserialize::default(),
                funding_method: Deserialize::default(),
                last4: Deserialize::default(),
                reference: Deserialize::default(),
                sender_account_number: Deserialize::default(),
                sender_name: Deserialize::default(),
                sender_sort_code: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                fingerprint: self.fingerprint.take()?,
                funding_method: self.funding_method.take()?,
                last4: self.last4.take()?,
                reference: self.reference.take()?,
                sender_account_number: self.sender_account_number.take()?,
                sender_name: self.sender_name.take()?,
                sender_sort_code: self.sender_sort_code.take()?,
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

    impl ObjectDeser for SourceTransactionGbpCreditTransferData {
        type Builder = SourceTransactionGbpCreditTransferDataBuilder;
    }

    impl FromValueOpt for SourceTransactionGbpCreditTransferData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTransactionGbpCreditTransferDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "fingerprint" => b.fingerprint = Some(FromValueOpt::from_value(v)?),
                    "funding_method" => b.funding_method = Some(FromValueOpt::from_value(v)?),
                    "last4" => b.last4 = Some(FromValueOpt::from_value(v)?),
                    "reference" => b.reference = Some(FromValueOpt::from_value(v)?),
                    "sender_account_number" => {
                        b.sender_account_number = Some(FromValueOpt::from_value(v)?)
                    }
                    "sender_name" => b.sender_name = Some(FromValueOpt::from_value(v)?),
                    "sender_sort_code" => b.sender_sort_code = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
