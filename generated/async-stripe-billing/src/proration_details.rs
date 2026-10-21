#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ProrationDetails {
    /// For a credit proration, links to the debit invoice line items or invoice item that the credit applies to.
    pub credited_items: Option<stripe_billing::InvoiceItemProrationCreditedItems>,
    /// Discount amounts applied when the proration was created.
    pub discount_amounts: Vec<stripe_shared::DiscountsResourceDiscountAmount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProrationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProrationDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ProrationDetailsBuilder {
    credited_items: Option<stripe_billing::InvoiceItemProrationCreditedItems>,
    discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
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

    impl Deserialize for ProrationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ProrationDetails>,
        builder: ProrationDetailsBuilder,
    }

    impl Visitor for Place<ProrationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ProrationDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ProrationDetailsBuilder {
        type Out = ProrationDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credited_items" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.credited_items)
                }
                "discount_amounts" => Deserialize::begin(&mut self.discount_amounts),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { credited_items: None, discount_amounts: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { credited_items, discount_amounts: Some(discount_amounts) } => {
                    Some(Self::Out { credited_items, discount_amounts })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for ProrationDetails {
        type Builder = ProrationDetailsBuilder;
    }

    impl FromValueOpt for ProrationDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
