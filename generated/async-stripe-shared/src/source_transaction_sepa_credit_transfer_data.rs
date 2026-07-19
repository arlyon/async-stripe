#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTransactionSepaCreditTransferData {
    /// Reference associated with the transfer.
    pub reference: Option<String>,
    /// Sender's bank account IBAN.
    pub sender_iban: Option<String>,
    /// Sender's name.
    pub sender_name: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTransactionSepaCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTransactionSepaCreditTransferData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTransactionSepaCreditTransferDataBuilder {
    reference: Option<Option<String>>,
    sender_iban: Option<Option<String>>,
    sender_name: Option<Option<String>>,
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

    impl Deserialize for SourceTransactionSepaCreditTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTransactionSepaCreditTransferData>,
        builder: SourceTransactionSepaCreditTransferDataBuilder,
    }

    impl Visitor for Place<SourceTransactionSepaCreditTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTransactionSepaCreditTransferDataBuilder {
                    reference: Deserialize::default(),
                    sender_iban: Deserialize::default(),
                    sender_name: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "sender_iban" => Deserialize::begin(&mut self.builder.sender_iban),
                "sender_name" => Deserialize::begin(&mut self.builder.sender_name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(reference), Some(sender_iban), Some(sender_name)) = (
                self.builder.reference.take(),
                self.builder.sender_iban.take(),
                self.builder.sender_name.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceTransactionSepaCreditTransferData {
                reference,
                sender_iban,
                sender_name,
            });
            Ok(())
        }
    }
};
