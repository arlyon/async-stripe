#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTransactionGbpCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTransactionGbpCreditTransferData").finish_non_exhaustive()
    }
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
                builder: SourceTransactionGbpCreditTransferDataBuilder {
                    fingerprint: Deserialize::default(),
                    funding_method: Deserialize::default(),
                    last4: Deserialize::default(),
                    reference: Deserialize::default(),
                    sender_account_number: Deserialize::default(),
                    sender_name: Deserialize::default(),
                    sender_sort_code: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "fingerprint" => Deserialize::begin(&mut self.builder.fingerprint),
                "funding_method" => Deserialize::begin(&mut self.builder.funding_method),
                "last4" => Deserialize::begin(&mut self.builder.last4),
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "sender_account_number" => {
                    Deserialize::begin(&mut self.builder.sender_account_number)
                }
                "sender_name" => Deserialize::begin(&mut self.builder.sender_name),
                "sender_sort_code" => Deserialize::begin(&mut self.builder.sender_sort_code),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(fingerprint),
                Some(funding_method),
                Some(last4),
                Some(reference),
                Some(sender_account_number),
                Some(sender_name),
                Some(sender_sort_code),
            ) = (
                self.builder.fingerprint.take(),
                self.builder.funding_method.take(),
                self.builder.last4.take(),
                self.builder.reference.take(),
                self.builder.sender_account_number.take(),
                self.builder.sender_name.take(),
                self.builder.sender_sort_code.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTransactionGbpCreditTransferData {
                fingerprint,
                funding_method,
                last4,
                reference,
                sender_account_number,
                sender_name,
                sender_sort_code,
            });
            Ok(())
        }
    }
};
