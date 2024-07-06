#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceLineItemsProrationDetails {
    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<stripe_shared::InvoicesResourceLineItemsCreditedItems>,
}
#[doc(hidden)]
pub struct InvoicesResourceLineItemsProrationDetailsBuilder {
    credited_items: Option<Option<stripe_shared::InvoicesResourceLineItemsCreditedItems>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesResourceLineItemsProrationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceLineItemsProrationDetails>,
        builder: InvoicesResourceLineItemsProrationDetailsBuilder,
    }

    impl Visitor for Place<InvoicesResourceLineItemsProrationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesResourceLineItemsProrationDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesResourceLineItemsProrationDetailsBuilder {
        type Out = InvoicesResourceLineItemsProrationDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credited_items" => Deserialize::begin(&mut self.credited_items),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { credited_items: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { credited_items: self.credited_items.take()? })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for InvoicesResourceLineItemsProrationDetails {
        type Builder = InvoicesResourceLineItemsProrationDetailsBuilder;
    }

    impl FromValueOpt for InvoicesResourceLineItemsProrationDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesResourceLineItemsProrationDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "credited_items" => b.credited_items = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
