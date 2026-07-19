#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceTransferData {
    /// The amount in cents (or local equivalent) that will be transferred to the destination account when the invoice is paid.
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount will be transferred to the destination.
    pub amount_percent: Option<f64>,
    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: stripe_types::Expandable<stripe_shared::Account>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceTransferData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceTransferDataBuilder {
    amount: Option<Option<i64>>,
    amount_percent: Option<Option<f64>>,
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

    impl Deserialize for QuotesResourceTransferData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceTransferData>,
        builder: QuotesResourceTransferDataBuilder,
    }

    impl Visitor for Place<QuotesResourceTransferData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceTransferDataBuilder {
                    amount: Deserialize::default(),
                    amount_percent: Deserialize::default(),
                    destination: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_percent" => Deserialize::begin(&mut self.builder.amount_percent),
                "destination" => Deserialize::begin(&mut self.builder.destination),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(amount_percent), Some(destination)) =
                (self.builder.amount, self.builder.amount_percent, self.builder.destination.take())
            else {
                return Ok(());
            };
            *self.out = Some(QuotesResourceTransferData { amount, amount_percent, destination });
            Ok(())
        }
    }
};
