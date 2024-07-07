#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTransactionChfCreditTransferData {
    /// Reference associated with the transfer.
    pub reference: Option<String>,
    /// Sender's country address.
    pub sender_address_country: Option<String>,
    /// Sender's line 1 address.
    pub sender_address_line1: Option<String>,
    /// Sender's bank account IBAN.
    pub sender_iban: Option<String>,
    /// Sender's name.
    pub sender_name: Option<String>,
}
#[doc(hidden)]
pub struct SourceTransactionChfCreditTransferDataBuilder {
    reference: Option<Option<String>>,
    sender_address_country: Option<Option<String>>,
    sender_address_line1: Option<Option<String>>,
    sender_iban: Option<Option<String>>,
    sender_name: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTransactionChfCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionChfCreditTransferData>,
        builder: SourceTransactionChfCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionChfCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTransactionChfCreditTransferDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTransactionChfCreditTransferDataBuilder {
        type Out = SourceTransactionChfCreditTransferData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.reference),
                "sender_address_country" => Deserialize::begin(&mut self.sender_address_country),
                "sender_address_line1" => Deserialize::begin(&mut self.sender_address_line1),
                "sender_iban" => Deserialize::begin(&mut self.sender_iban),
                "sender_name" => Deserialize::begin(&mut self.sender_name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                reference: Deserialize::default(),
                sender_address_country: Deserialize::default(),
                sender_address_line1: Deserialize::default(),
                sender_iban: Deserialize::default(),
                sender_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                reference: self.reference.take()?,
                sender_address_country: self.sender_address_country.take()?,
                sender_address_line1: self.sender_address_line1.take()?,
                sender_iban: self.sender_iban.take()?,
                sender_name: self.sender_name.take()?,
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

    impl ObjectDeser for SourceTransactionChfCreditTransferData {
        type Builder = SourceTransactionChfCreditTransferDataBuilder;
    }

    impl FromValueOpt for SourceTransactionChfCreditTransferData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTransactionChfCreditTransferDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "reference" => b.reference = Some(FromValueOpt::from_value(v)?),
                    "sender_address_country" => {
                        b.sender_address_country = Some(FromValueOpt::from_value(v)?)
                    }
                    "sender_address_line1" => {
                        b.sender_address_line1 = Some(FromValueOpt::from_value(v)?)
                    }
                    "sender_iban" => b.sender_iban = Some(FromValueOpt::from_value(v)?),
                    "sender_name" => b.sender_name = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
