#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DeletedRadarValueListItem {
    #[allow(dead_code)]
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListItemId,
}
#[doc(hidden)]
pub struct DeletedRadarValueListItemBuilder {
    deleted: Option<stripe_types::AlwaysTrue>,
    id: Option<stripe_fraud::RadarValueListItemId>,
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

    impl Deserialize for DeletedRadarValueListItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DeletedRadarValueListItem>,
        builder: DeletedRadarValueListItemBuilder,
    }

    impl Visitor for Place<DeletedRadarValueListItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DeletedRadarValueListItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DeletedRadarValueListItemBuilder {
        type Out = DeletedRadarValueListItem;
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
            let (Some(deleted), Some(id)) = (self.deleted, self.id.take()) else {
                return None;
            };
            Some(Self::Out { deleted, id })
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

    impl ObjectDeser for DeletedRadarValueListItem {
        type Builder = DeletedRadarValueListItemBuilder;
    }

    impl FromValueOpt for DeletedRadarValueListItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DeletedRadarValueListItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "deleted" => b.deleted = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for DeletedRadarValueListItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("DeletedRadarValueListItem", 3)?;
        s.serialize_field("deleted", &self.deleted)?;
        s.serialize_field("id", &self.id)?;

        s.serialize_field("object", "radar.value_list_item")?;
        s.end()
    }
}
impl stripe_types::Object for DeletedRadarValueListItem {
    type Id = stripe_fraud::RadarValueListItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
