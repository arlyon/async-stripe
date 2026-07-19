#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingTransactionNetworkData {
    /// A code created by Stripe which is shared with the merchant to validate the authorization.
    /// This field will be populated if the authorization message was approved.
    /// The code typically starts with the letter "S", followed by a six-digit number.
    /// For example, "S498162".
    /// Please note that the code is not guaranteed to be unique across authorizations.
    pub authorization_code: Option<String>,
    /// The date the transaction was processed by the card network.
    /// This can be different from the date the seller recorded the transaction depending on when the acquirer submits the transaction to the network.
    pub processing_date: Option<String>,
    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingTransactionNetworkData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingTransactionNetworkData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingTransactionNetworkDataBuilder {
    authorization_code: Option<Option<String>>,
    processing_date: Option<Option<String>>,
    transaction_id: Option<Option<String>>,
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

    impl Deserialize for IssuingTransactionNetworkData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingTransactionNetworkData>,
        builder: IssuingTransactionNetworkDataBuilder,
    }

    impl Visitor for Place<IssuingTransactionNetworkData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingTransactionNetworkDataBuilder {
                    authorization_code: Deserialize::default(),
                    processing_date: Deserialize::default(),
                    transaction_id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "authorization_code" => Deserialize::begin(&mut self.builder.authorization_code),
                "processing_date" => Deserialize::begin(&mut self.builder.processing_date),
                "transaction_id" => Deserialize::begin(&mut self.builder.transaction_id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(authorization_code), Some(processing_date), Some(transaction_id)) = (
                self.builder.authorization_code.take(),
                self.builder.processing_date.take(),
                self.builder.transaction_id.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(IssuingTransactionNetworkData {
                authorization_code,
                processing_date,
                transaction_id,
            });
            Ok(())
        }
    }
};
