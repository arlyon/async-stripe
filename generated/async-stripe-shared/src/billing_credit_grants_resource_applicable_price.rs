#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceApplicablePrice {
    /// Unique identifier for the object.
    pub id: Option<stripe_shared::BillingCreditGrantsResourceApplicablePriceId>,
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceApplicablePriceBuilder {
    id: Option<Option<stripe_shared::BillingCreditGrantsResourceApplicablePriceId>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: BillingCreditGrantsResourceApplicablePriceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantsResourceApplicablePriceBuilder {
        type Out = BillingCreditGrantsResourceApplicablePrice;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(id),) = (self.id.take(),) else {
                return None;
            };
            Some(Self::Out { id })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for BillingCreditGrantsResourceApplicablePrice {
        type Builder = BillingCreditGrantsResourceApplicablePriceBuilder;
    }

    impl FromValueOpt for BillingCreditGrantsResourceApplicablePrice {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantsResourceApplicablePriceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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
