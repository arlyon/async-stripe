#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DeletedSubscriptionItem {
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionItemId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeletedSubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeletedSubscriptionItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DeletedSubscriptionItemBuilder {
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_shared::SubscriptionItemId>,
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
                builder: DeletedSubscriptionItemBuilder {
                    deleted: Deserialize::default(),
                    id: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "deleted" => Deserialize::begin(&mut self.builder.deleted),
                "id" => Deserialize::begin(&mut self.builder.id),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(deleted), Some(id)) = (self.builder.deleted, self.builder.id.take()) else {
                return Ok(());
            };
            *self.out = Some(DeletedSubscriptionItem { deleted, id });
            Ok(())
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
