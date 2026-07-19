#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceTransferData {
    /// The amount in cents (or local equivalent) that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    /// The connected account receiving the transfer.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceTransferData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceTransferDataBuilder {
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

    impl Deserialize for PaymentLinksResourceTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceTransferData>,
        builder: PaymentLinksResourceTransferDataBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceTransferDataBuilder {
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
            *self.out = Some(PaymentLinksResourceTransferData { amount, destination });
            Ok(())
        }
    }
};
