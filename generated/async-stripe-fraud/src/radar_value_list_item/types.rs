/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing list items](https://stripe.com/docs/radar/lists#managing-list-items)
///
/// For more details see <<https://stripe.com/docs/api/radar/value_list_items/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarValueListItem {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListItemId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The value of the item.
    pub value: String,
    /// The identifier of the value list this item belongs to.
    pub value_list: String,
}
#[doc(hidden)]
pub struct RadarValueListItemBuilder {
    created: Option<stripe_types::Timestamp>,
    created_by: Option<String>,
    id: Option<stripe_fraud::RadarValueListItemId>,
    livemode: Option<bool>,
    value: Option<String>,
    value_list: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for RadarValueListItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarValueListItem>,
        builder: RadarValueListItemBuilder,
    }

    impl Visitor for Place<RadarValueListItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RadarValueListItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RadarValueListItemBuilder {
        type Out = RadarValueListItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "created_by" => Deserialize::begin(&mut self.created_by),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "value" => Deserialize::begin(&mut self.value),
                "value_list" => Deserialize::begin(&mut self.value_list),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                created_by: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                value: Deserialize::default(),
                value_list: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(created_by),
                Some(id),
                Some(livemode),
                Some(value),
                Some(value_list),
            ) = (
                self.created,
                self.created_by.take(),
                self.id.take(),
                self.livemode,
                self.value.take(),
                self.value_list.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { created, created_by, id, livemode, value, value_list })
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

    impl ObjectDeser for RadarValueListItem {
        type Builder = RadarValueListItemBuilder;
    }

    impl FromValueOpt for RadarValueListItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RadarValueListItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "created_by" => b.created_by = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),
                    "value_list" => b.value_list = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for RadarValueListItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("RadarValueListItem", 7)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("created_by", &self.created_by)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("value", &self.value)?;
        s.serialize_field("value_list", &self.value_list)?;

        s.serialize_field("object", "radar.value_list_item")?;
        s.end()
    }
}
impl stripe_types::Object for RadarValueListItem {
    type Id = stripe_fraud::RadarValueListItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(RadarValueListItemId);
