#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceNameCollection {
    pub business: Option<stripe_shared::PaymentLinksResourceBusinessName>,
    pub individual: Option<stripe_shared::PaymentLinksResourceIndividualName>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceNameCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceNameCollection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceNameCollectionBuilder {
    business: Option<Option<stripe_shared::PaymentLinksResourceBusinessName>>,
    individual: Option<Option<stripe_shared::PaymentLinksResourceIndividualName>>,
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

    impl Deserialize for PaymentLinksResourceNameCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceNameCollection>,
        builder: PaymentLinksResourceNameCollectionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceNameCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceNameCollectionBuilder {
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
            *self.out = Some(PaymentLinksResourceNameCollection { business, individual });
            Ok(())
        }
    }
};
