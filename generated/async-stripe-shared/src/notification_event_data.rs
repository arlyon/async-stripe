#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct NotificationEventData {
    /// Object containing the API resource relevant to the event.
    /// For example, an `invoice.created` event will have a full [invoice object](https://api.stripe.com#invoice_object) as the value of the object key.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json")
    )]
    pub object: stripe_miniserde::json::Value,
    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    #[cfg_attr(
        any(feature = "deserialize", feature = "serialize"),
        serde(with = "stripe_types::with_serde_json_opt")
    )]
    pub previous_attributes: Option<stripe_miniserde::json::Value>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for NotificationEventData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("NotificationEventData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct NotificationEventDataBuilder {
    object: Option<stripe_miniserde::json::Value>,
    previous_attributes: Option<Option<stripe_miniserde::json::Value>>,
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
                builder: NotificationEventDataBuilder {
                    object: Deserialize::default(),
                    previous_attributes: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "object" => Deserialize::begin(&mut self.builder.object),
                "previous_attributes" => Deserialize::begin(&mut self.builder.previous_attributes),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(object), Some(previous_attributes)) =
                (self.builder.object.take(), self.builder.previous_attributes.take())
            else {
                return Ok(());
            };
            *self.out = Some(NotificationEventData { object, previous_attributes });
            Ok(())
        }
    }
};
