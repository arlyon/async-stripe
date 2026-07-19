#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceApplicablePrice {
    /// Unique identifier for the object.
    pub id: Option<stripe_shared::BillingCreditGrantsResourceApplicablePriceId>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingCreditGrantsResourceApplicablePrice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingCreditGrantsResourceApplicablePrice").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceApplicablePriceBuilder {
    id: Option<Option<stripe_shared::BillingCreditGrantsResourceApplicablePriceId>>,
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

    impl Deserialize for BillingCreditGrantsResourceApplicablePrice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceApplicablePrice>,
        builder: BillingCreditGrantsResourceApplicablePriceBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceApplicablePrice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceApplicablePriceBuilder {
                    id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(id),) = (self.builder.id.take(),) else {
                return Ok(());
            };
            *self.out = Some(BillingCreditGrantsResourceApplicablePrice { id });
            Ok(())
        }
    }
};
impl stripe_types::Object for BillingCreditGrantsResourceApplicablePrice {
    type Id = Option<stripe_shared::BillingCreditGrantsResourceApplicablePriceId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(BillingCreditGrantsResourceApplicablePriceId);
