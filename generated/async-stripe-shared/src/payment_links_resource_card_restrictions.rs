#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceCardRestrictions {
    /// The card brands to block.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
    pub brands_blocked: Vec<stripe_shared::BrandsBlocked>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceCardRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceCardRestrictions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceCardRestrictionsBuilder {
    brands_blocked: Option<Vec<stripe_shared::BrandsBlocked>>,
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

    impl Deserialize for PaymentLinksResourceCardRestrictions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceCardRestrictions>,
        builder: PaymentLinksResourceCardRestrictionsBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceCardRestrictions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceCardRestrictionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceCardRestrictionsBuilder {
        type Out = PaymentLinksResourceCardRestrictions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brands_blocked" => Deserialize::begin(&mut self.brands_blocked),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { brands_blocked: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(brands_blocked),) = (self.brands_blocked.take(),) else {
                return None;
            };
            Some(Self::Out { brands_blocked })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentLinksResourceCardRestrictions {
        type Builder = PaymentLinksResourceCardRestrictionsBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceCardRestrictions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceCardRestrictionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brands_blocked" => b.brands_blocked = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The card brands to block.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
pub type PaymentLinksResourceCardRestrictionsBrandsBlocked = stripe_shared::BrandsBlocked;
