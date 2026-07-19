#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ChargeTransferData {
    /// The amount transferred to the destination account, if specified.
    /// By default, the entire charge amount is transferred to the destination account.
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ChargeTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ChargeTransferData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ChargeTransferDataBuilder {
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

    impl Deserialize for ChargeTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ChargeTransferData>,
        builder: ChargeTransferDataBuilder,
    }

    impl Visitor for Place<ChargeTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ChargeTransferDataBuilder {
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
            *self.out = Some(ChargeTransferData { amount, destination });
            Ok(())
        }
    }
};
