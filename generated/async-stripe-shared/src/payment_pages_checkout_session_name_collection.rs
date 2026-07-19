#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionNameCollection {
    pub business: Option<stripe_shared::PaymentPagesCheckoutSessionBusinessName>,
    pub individual: Option<stripe_shared::PaymentPagesCheckoutSessionIndividualName>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionNameCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionNameCollection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionNameCollectionBuilder {
    business: Option<Option<stripe_shared::PaymentPagesCheckoutSessionBusinessName>>,
    individual: Option<Option<stripe_shared::PaymentPagesCheckoutSessionIndividualName>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionNameCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionNameCollection>,
        builder: PaymentPagesCheckoutSessionNameCollectionBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionNameCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionNameCollectionBuilder {
                    business: Deserialize::default(),
                    individual: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "business" => Deserialize::begin(&mut self.builder.business),
                "individual" => Deserialize::begin(&mut self.builder.individual),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(business), Some(individual)) =
                (self.builder.business, self.builder.individual)
            else {
                return Ok(());
            };
            *self.out = Some(PaymentPagesCheckoutSessionNameCollection { business, individual });
            Ok(())
        }
    }
};
