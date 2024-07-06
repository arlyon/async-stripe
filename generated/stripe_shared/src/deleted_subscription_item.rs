#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DeletedSubscriptionItem {
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionItemId,
}
#[doc(hidden)]
pub struct DeletedSubscriptionItemBuilder {
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_shared::SubscriptionItemId>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for DeletedSubscriptionItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedSubscriptionItem>,
        builder: DeletedSubscriptionItemBuilder,
    }

    impl Visitor for Place<DeletedSubscriptionItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DeletedSubscriptionItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DeletedSubscriptionItemBuilder {
        type Out = DeletedSubscriptionItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "deleted" => Deserialize::begin(&mut self.deleted),
                "id" => Deserialize::begin(&mut self.id),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { deleted: Deserialize::default(), id: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { deleted: self.deleted?, id: self.id.take()? })
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

    impl ObjectDeser for DeletedSubscriptionItem {
        type Builder = DeletedSubscriptionItemBuilder;
    }

    impl FromValueOpt for DeletedSubscriptionItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DeletedSubscriptionItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "deleted" => b.deleted = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for DeletedSubscriptionItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("DeletedSubscriptionItem", 3)?;
        s.serialize_field("deleted", &self.deleted)?;
        s.serialize_field("id", &self.id)?;

        s.serialize_field("object", "subscription_item")?;
        s.end()
    }
}
impl stripe_types::Object for DeletedSubscriptionItem {
    type Id = stripe_shared::SubscriptionItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
