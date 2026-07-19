/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing list items](https://docs.stripe.com/radar/lists#managing-list-items)
///
/// For more details see <<https://stripe.com/docs/api/radar/value_list_items/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarValueListItem {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,
    /// Unique identifier for the object.
    pub id: stripe_fraud::RadarValueListItemId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The value of the item.
    pub value: String,
    /// The identifier of the value list this item belongs to.
    pub value_list: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RadarValueListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadarValueListItem").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: RadarValueListItemBuilder {
                    created: Deserialize::default(),
                    created_by: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    value: Deserialize::default(),
                    value_list: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "created_by" => Deserialize::begin(&mut self.builder.created_by),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "value" => Deserialize::begin(&mut self.builder.value),
                "value_list" => Deserialize::begin(&mut self.builder.value_list),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(created_by),
                Some(id),
                Some(livemode),
                Some(value),
                Some(value_list),
            ) = (
                self.builder.created,
                self.builder.created_by.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.value.take(),
                self.builder.value_list.take(),
            )
            else {
                return Ok(());
            };
            *self.out =
                Some(RadarValueListItem { created, created_by, id, livemode, value, value_list });
            Ok(())
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
