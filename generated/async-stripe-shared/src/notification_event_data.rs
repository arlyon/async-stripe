#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct NotificationEventData {
    /// Object containing the API resource relevant to the event.
    /// For example, an `invoice.created` event will have a full [invoice object](https://stripe.com/docs/api#invoice_object) as the value of the object key.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json")
    )]
    pub object: miniserde::json::Value,
    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json_opt")
    )]
    pub previous_attributes: Option<miniserde::json::Value>,
}
#[doc(hidden)]
pub struct NotificationEventDataBuilder {
    object: Option<miniserde::json::Value>,
    previous_attributes: Option<Option<miniserde::json::Value>>,
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

    impl Deserialize for NotificationEventData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<NotificationEventData>,
        builder: NotificationEventDataBuilder,
    }

    impl Visitor for Place<NotificationEventData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: NotificationEventDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for NotificationEventDataBuilder {
        type Out = NotificationEventData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "object" => Deserialize::begin(&mut self.object),
                "previous_attributes" => Deserialize::begin(&mut self.previous_attributes),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { object: Deserialize::default(), previous_attributes: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(object), Some(previous_attributes)) =
                (self.object.take(), self.previous_attributes.take())
            else {
                return None;
            };
            Some(Self::Out { object, previous_attributes })
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

    impl ObjectDeser for NotificationEventData {
        type Builder = NotificationEventDataBuilder;
    }

    impl FromValueOpt for NotificationEventData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = NotificationEventDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "object" => b.object = FromValueOpt::from_value(v),
                    "previous_attributes" => b.previous_attributes = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
