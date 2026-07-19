#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceComputed {
    /// The definitive totals and line items the customer will be charged on a recurring basis.
    /// Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only.
    /// Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Option<stripe_billing::QuotesResourceRecurring>,
    pub upfront: stripe_billing::QuotesResourceUpfront,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceComputed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceComputed").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceComputedBuilder {
    recurring: Option<Option<stripe_billing::QuotesResourceRecurring>>,
    upfront: Option<stripe_billing::QuotesResourceUpfront>,
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

    impl Deserialize for QuotesResourceComputed {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceComputed>,
        builder: QuotesResourceComputedBuilder,
    }

    impl Visitor for Place<QuotesResourceComputed> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceComputedBuilder {
                    recurring: Deserialize::default(),
                    upfront: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "recurring" => Deserialize::begin(&mut self.builder.recurring),
                "upfront" => Deserialize::begin(&mut self.builder.upfront),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(recurring), Some(upfront)) =
                (self.builder.recurring.take(), self.builder.upfront.take())
            else {
                return Ok(());
            };
            *self.out = Some(QuotesResourceComputed { recurring, upfront });
            Ok(())
        }
    }
};
