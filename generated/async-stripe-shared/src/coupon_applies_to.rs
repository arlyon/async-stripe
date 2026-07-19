#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CouponAppliesTo {
    /// A list of product IDs this coupon applies to
    pub products: Vec<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CouponAppliesTo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CouponAppliesTo").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CouponAppliesToBuilder {
    products: Option<Vec<String>>,
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

    impl Deserialize for CouponAppliesTo {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CouponAppliesTo>,
        builder: CouponAppliesToBuilder,
    }

    impl Visitor for Place<CouponAppliesTo> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CouponAppliesToBuilder { products: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "products" => Deserialize::begin(&mut self.builder.products),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(products),) = (self.builder.products.take(),) else {
                return Ok(());
            };
            *self.out = Some(CouponAppliesTo { products });
            Ok(())
        }
    }
};
