#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionDetailsData {
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) defined as subscription metadata when an invoice is created.
    /// Becomes an immutable snapshot of the subscription metadata at the time of invoice finalization.
    ///  *Note: This attribute is populated only for invoices created on or after June 29, 2023.*
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
#[doc(hidden)]
pub struct SubscriptionDetailsDataBuilder {
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionDetailsData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionDetailsData>,
        builder: SubscriptionDetailsDataBuilder,
    }

    impl Visitor for Place<SubscriptionDetailsData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionDetailsDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionDetailsDataBuilder {
        type Out = SubscriptionDetailsData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "metadata" => Deserialize::begin(&mut self.metadata),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { metadata: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(metadata),) = (self.metadata.take(),) else {
                return None;
            };
            Some(Self::Out { metadata })
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

    impl ObjectDeser for SubscriptionDetailsData {
        type Builder = SubscriptionDetailsDataBuilder;
    }

    impl FromValueOpt for SubscriptionDetailsData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionDetailsDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "metadata" => b.metadata = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
