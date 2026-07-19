#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TransferData {
    /// The amount transferred to the destination account.
    /// This transfer will occur automatically after the payment succeeds.
    /// If no amount is specified, by default the entire payment amount is transferred to the destination account.
    /// The amount must be less than or equal to the [amount](https://docs.stripe.com/api/payment_intents/object#payment_intent_object-amount), and must be a positive integer.
    ///  representing how much to transfer in the smallest currency unit (e.g., 100 cents to charge $1.00).
    pub amount: Option<i64>,
    /// The account (if any) that the payment is attributed to for tax reporting, and where funds from the payment are transferred to after payment success.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TransferData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TransferDataBuilder {
    amount: Option<Option<i64>>,
    destination: Option<stripe_types::Expandable<stripe_shared::Account>>,
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

    impl Deserialize for TransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TransferData>,
        builder: TransferDataBuilder,
    }

    impl Visitor for Place<TransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TransferDataBuilder {
                    amount: Deserialize::default(),
                    destination: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "destination" => Deserialize::begin(&mut self.builder.destination),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(destination)) =
                (self.builder.amount, self.builder.destination.take())
            else {
                return Ok(());
            };
            *self.out = Some(TransferData { amount, destination });
            Ok(())
        }
    }
};
